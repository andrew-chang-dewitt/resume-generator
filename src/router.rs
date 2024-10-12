//! A simple, generalized, & nestable routing solution, capable of sending a given Request to the
//! appropriate Route (or sub-Router) for handling & of processing the result returned.

use std::fmt::Display;

use anyhow::anyhow;
use log::{error, info};

pub struct Router<Routes: Copy + Matcher> {
    location: Routes,
    state: RouterState<Routes>,
}

impl<Routes: Copy + Matcher> Router<Routes> {
    pub fn new(initial_path: Routes) -> Self {
        Self {
            location: initial_path,
            state: RouterState::Initializing,
        }
    }

    pub fn navigate(&mut self) -> anyhow::Result<RouterEvent<Routes>> {
        self.match_path()
    }

    fn is_running(&self) -> bool {
        if let RouterState::Running(_) = self.state {
            true
        } else {
            false
        }
    }

    fn dispatch(&mut self, event: RouterEvent<Routes>) {
        match event {
            RouterEvent::Goto(route) => self.location = route,
            RouterEvent::Cancel => todo!(),
            RouterEvent::Exit(msg) => self.exit(msg).unwrap(),
            RouterEvent::Error(_) => todo!(),
        }
    }

    fn exit(&mut self, msg: String) -> Result<(), RouterError> {
        self.state = RouterState::Exiting;
        info!("{msg}");
        info!("Exit requested, shutting down...");
        // TODO: do any graceful shutdown here? ...
        Ok(())
    }

    pub fn run(mut self) -> Result<Routes, RouterError> {
        self.state = RouterState::Running(RunningState::Listening);

        while self.is_running() {
            match self.match_path() {
                Ok(event) => self.dispatch(event),
                Err(err) => self.dispatch(err.into()),
            }
        }

        Ok(self.location)
    }
}

#[derive(Debug)]
enum RouterState<Routes> {
    Initializing,
    Idle,
    Running(RunningState<Routes>),
    Exiting(String),
}

#[derive(Debug)]
enum RunningState<Routes> {
    Listening,
    Navigating(Routes),
}

#[derive(Debug)]
pub enum RouterEvent<Routes> {
    Initialized,
    Start,
    Goto(Routes),
    DoneNavigating(Routes),
    Cancel(Routes),
    Error(RouterError),
    Exit(String),
}

impl<Routes: Copy + std::fmt::Debug> RouterState<Routes> {
    fn next_from_event(self, event: RouterEvent<Routes>) -> Result<Self, RouterError> {
        match self {
            Self::Initializing => match event {
                RouterEvent::Initialized => Ok(Self::Idle),
                // TODO: not sure this error propagation is the best choice here...
                RouterEvent::Error(err) => Err(err),
                _ => Err(RouterError::NotReady(format!(
                    "Received {event:?}, however router is currently {self:?}"
                ))),
            },
            Self::Idle => match event {
                RouterEvent::Start => Ok(Self::Running(RunningState::Listening)),
                RouterEvent::Exit(msg) => Ok(Self::Exiting(msg)),
                RouterEvent::Error(err) => Err(err),
                RouterEvent::Goto(_) => Err(RouterError::NotReady(format!(
                    "Router not yet started, please try again later."
                ))),
                _ => Err(RouterError::NotReady(format!(
                    "Received {event:?}, however router is currently {self:?}"
                ))),
            },
            Self::Running(RunningState::Listening) => match event {
                RouterEvent::Goto(route) => Ok(Self::Running(RunningState::Navigating(route))),
                RouterEvent::Exit(msg) => Ok(Self::Exiting(msg)),
                RouterEvent::Cancel(_) => Err(RouterError::NotReady(format!(
                    "Received {event:?}, however there's no navigation to cancel."
                ))),
                _ => Err(RouterError::NotReady(format!(
                    "Received {event:?}, however router is currently {self:?}"
                ))),
            },
            Self::Running(RunningState::Navigating(_going_to)) => match event {
                RouterEvent::Cancel(_cancel_going_to) => todo!(),
                RouterEvent::DoneNavigating(_) => Ok(Self::Running(RunningState::Listening)),
                _ => Err(RouterError::NotReady(format!(
                    "Received {event:?}, however router is currently {self:?}",
                ))),
            },
            Self::Exiting(_) => Err(RouterError::NotReady(format!(
                "Received {event:?}, however router is currently {self:?}"
            ))),
        }
    }
}

#[derive(Debug)]
pub enum RouterError {
    // For when the router needs to exit? TODO: maybe not needed anymore?
    Exit(String),
    // For when the router is in a state that's not ready to handle the given request
    NotReady(String),
    // For everything else
    Unhandled(anyhow::Error),
}

// impl<Routes> Display for RouterError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::Exit(msg) => write!(f, "RouterError::Exit: {msg}"),
//             Self::NotReady(msg) => write!(f, "RouterError::NotReady: {msg}"),
//         }
//     }
// }

impl<E, R> From<E> for RouterEvent<R>
where
    E: Into<RouterError>,
{
    fn from(value: E) -> Self {
        match value.into() {
            // unhandled errors cancel navigation & keep router at current location?
            RouterError::Unhandled(err) => {
                error!("Unhandled error caught at router: {err:#?}");
                RouterEvent::Cancel(_)
            }

            // TODO: irrecoverable errors cancel navigation & trigger program exit

            // program exit causes program exit
            RouterError::Exit(msg) => RouterEvent::Exit(msg),
        }
    }
}

impl From<anyhow::Error> for RouterError {
    fn from(value: anyhow::Error) -> Self {
        RouterError::Unhandled(value)
    }
}

pub trait Matcher<Routes = Self> {
    fn match_path(&self) -> anyhow::Result<RouterEvent<Routes>>;
}

impl<Routes: Copy + Matcher> Matcher<Routes> for Router<Routes> {
    fn match_path(&self) -> anyhow::Result<RouterEvent<Routes>> {
        self.location.match_path()
    }
}

#[cfg(test)]
mod tests {
    use anyhow::anyhow;

    use super::*;

    #[derive(Copy, Clone, Debug, PartialEq)]
    enum RoutesTable {
        Somewhere,
        AfterSomewhere,
        GoingDeeper(NestedRoutes),
        Exit,
    }

    impl Matcher for RoutesTable {
        fn match_path(&self) -> anyhow::Result<RouterEvent<RoutesTable>> {
            match self {
                // these match arms should really call handlers described elsewhere
                // or just execute simple expressions in place
                Self::Somewhere => Ok(RouterEvent::Goto(Self::AfterSomewhere)),
                Self::AfterSomewhere => {
                    Ok(RouterEvent::Goto(Self::GoingDeeper(NestedRoutes::WentDeep)))
                }
                Self::GoingDeeper(deeper) => deeper.match_path(),
                Self::Exit => Ok(RouterEvent::Exit("Time to quit...".into())),
            }
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq)]
    enum NestedRoutes {
        WentDeep,
    }

    impl Matcher<RoutesTable> for NestedRoutes {
        fn match_path(&self) -> anyhow::Result<RouterEvent<RoutesTable>> {
            Ok(RouterEvent::Goto(RoutesTable::Exit))
        }
    }

    #[tokio::test]
    async fn motivating_example() -> anyhow::Result<()> {
        let mut router = Router::new(RoutesTable::Somewhere);

        let mut actual = router.navigate()?;
        let mut expected = RouterEvent::Goto(RoutesTable::AfterSomewhere);

        match actual {
            RouterEvent::Goto(RoutesTable::AfterSomewhere) => router.dispatch(actual),
            _ => panic!(
                "first navigation should have returned {expected:?}, instead got: {actual:?}"
            ),
        }

        actual = router.navigate()?;
        expected = RouterEvent::Goto(RoutesTable::GoingDeeper(NestedRoutes::WentDeep));

        match actual {
            RouterEvent::Goto(RoutesTable::GoingDeeper(NestedRoutes::WentDeep)) => {
                router.dispatch(actual)
            }
            _ => panic!(
                "second navigation should have returned {expected:?}, instead got: {actual:?}"
            ),
        }

        actual = router.navigate()?;
        expected = RouterEvent::Goto(RoutesTable::Exit);

        match actual {
            RouterEvent::Goto(RoutesTable::Exit) => router.dispatch(actual),
            _ => panic!(
                "final navigation should have returned {expected:?}, instead got: {actual:?}"
            ),
        }

        match router.state {
            RouterState::Exiting => Ok(()),
            _ => panic!(
                "Router state should have ended as RouterState::Exiting, instead got {:?}",
                router.state
            ),
        }
    }

    #[tokio::test]
    async fn router_processing_loop() {
        let router = Router::new(RoutesTable::Somewhere);

        match router.run() {
            Ok(RoutesTable::Exit) => (),
            Ok(r) => panic!("Ended as some route {r:?}, expected RoutesTable::Exit"),
            Err(e) => panic!("Ended in some error {e:?}"),
        }
    }
}
