//! A simple, generalized, & nestable routing solution, capable of sending a given Request to the
//! appropriate Route (or sub-Router) for handling & of processing the result returned.

use log::{error, info};

type RouterResult<R = ()> = Result<R, RouterError>;

pub struct Router<Routes: Copy + Matcher> {
    context: RouterConext<Routes>,
    state: RouterState<Routes>,
}

pub struct RouterConext<Routes: Copy + Matcher> {
    location: Routes,
}

impl<Routes: Copy + std::fmt::Debug + Matcher + PartialEq> Router<Routes> {
    pub fn new(initial_path: Routes) -> Self {
        Self {
            context: RouterConext {
                location: initial_path,
            },
            state: RouterState::Initializing,
        }
    }

    pub fn dispatch(&mut self, event: RouterEvent<Routes>) -> RouterResult {
        // first get next state
        let next_state = self.state.next_from_event(event)?;
        // then, if next state is not the same as current state, make sure current state is ready to exit
        if next_state != self.state {
            self.context = self.state.on_exit(self.context)?;
        };
        // make sure router is ready to enter next state and change state
        let (state, context) = next_state.on_entry(self.context)?;
        self.state = state;
        self.context = context;

        // if we get here, event was dispatched & state changed accordingly w/out error
        Ok(())
    }

    pub fn init(&mut self) -> RouterResult {
        self.dispatch(RouterEvent::StartListening)
    }

    pub fn navigate(&mut self, path: Routes) -> RouterResult<Routes> {
        self.location = path;
        self.match_path().map_err(|e| e.into())
    }

    fn is_running(&self) -> bool {
        if let RouterState::Running(_) = self.state {
            true
        } else {
            false
        }
    }

    fn exit(&mut self, msg: String) -> Result<Routes, RouterError> {
        self.state = RouterState::Exiting;
        info!("{msg}");
        info!("Exit requested, shutting down...");

        // do any graceful shutdown here? ...

        Ok(self.location)
    }

    pub fn run(mut self) -> RouterResult {
        todo!()
        // self.state = RouterState::Running(RunningState::Listening);

        // // TODO: instead of trying to coerce everything to return a Route, even when it doesn't
        // // make sense, why not also impl an event dispatcher on the Router?
        // // then this can call self.dispatch on each loop iteration, issuing a `Navigate(next)` on
        // // successful routings or successful error recovery, and a `Exit` on Program exit?
        // // then error handler just needs to return a RouterEvent type
        // while self.is_running() {
        //     self.location = match self.match_path() {
        //         Ok(next) => next,
        //         // TODO: IDEA: maybe instead of impl ErrorHandler for Router, I can use
        //         // TryFrom<impl Into<RouterError>> for RouterEvent?
        //         //         ^^^ Not sure this exactly will work, but could always first
        //         //             RouterError::from(err), then RouterEvent::from(that err)...
        //         Err(err) => match Router::<Routes>::handle_error(err, self.location) {
        //             Ok(next) => next,
        //             Err(RouterError::Exit(msg)) => self.exit(msg)?,
        //         },
        //     }
        // }

        // Ok(self.location)
    }
}

#[derive(Debug, PartialEq)]
enum RouterState<Routes> {
    Initializing,
    Idle,
    Running(RunningState<Routes>),
    Exiting,
}

#[derive(Debug, PartialEq)]
enum RunningState<Routes> {
    Listening,
    Navigating(Routes),
}

trait StateChange<R: Copy + std::fmt::Debug + Matcher + PartialEq>: Sized {
    fn on_entry(&self, router: &mut Router<R>) -> RouterResult;
    fn on_exit(&self, router: &mut Router<R>) -> RouterResult;
}

impl<R: Copy + std::fmt::Debug + Matcher + PartialEq> StateChange<R> for RouterState<R> {
    fn on_exit(&self, router: &mut Router<R>) -> RouterResult {
        todo!()
    }

    fn on_entry(&self, router: &mut Router<R>) -> RouterResult {
        todo! {}
    }
}

impl<R: Copy + std::fmt::Debug + Matcher + PartialEq> StateChange<R> for RunningState<R> {
    fn on_exit(&self, router: &mut Router<R>) -> RouterResult {
        todo!()
    }

    fn on_entry(&self, router: &mut Router<R>) -> RouterResult {
        todo! {}
    }
}

#[derive(Debug)]
pub enum RouterEvent<Routes> {
    StartListening,
    Goto(Routes),
    Cancel,
    StopListening,
    Error(RouterError),
    Exit(String),
}

impl<R: Copy + std::fmt::Debug> RouterState<R> {
    pub fn next_from_event(self, event: RouterEvent<R>) -> Result<Self, RouterError> {
        todo!()
    }
}

impl<E, R> From<E> for RouterEvent<R>
where
    E: Into<RouterError>,
{
    fn from(value: E) -> Self {
        match value.into() {
            // unhandled errors cancel navigation & keep router at current location?
            RouterError::Unhandled(err) => {
                error!("Unhandled error caught at router: {err:#?}");
                RouterEvent::Cancel
            }

            // TODO: irrecoverable errors cancel navigation & trigger program exit

            // program exit causes program exit
            RouterError::Exit(msg) => RouterEvent::Exit(msg),
        }
    }
}

// TODO: I'm sure there's a better way to do this, but need to read more on implementing errors &
// error handling better in Rust
#[derive(Debug)]
pub enum RouterError {
    Exit(String),
    Unhandled(anyhow::Error),
}

impl From<anyhow::Error> for RouterError {
    fn from(value: anyhow::Error) -> Self {
        RouterError::Unhandled(value)
    }
}

impl<Routes: Copy + Matcher> Matcher<Routes, Routes> for Router<Routes> {
    fn match_path(&self) -> anyhow::Result<Routes> {
        self.location.match_path()
    }
}

pub trait Matcher<Root = Self, Routes = Self> {
    fn match_path(&self) -> anyhow::Result<Root>;
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
        fn match_path(&self) -> anyhow::Result<RoutesTable> {
            match self {
                // these match arms should really call handlers described elsewhere
                // or just execute simple expressions in place
                Self::Somewhere => Ok(Self::AfterSomewhere),
                Self::AfterSomewhere => Ok(Self::GoingDeeper(NestedRoutes::WentDeep)),
                Self::GoingDeeper(deeper) => deeper.match_path(),
                Self::Exit => Err(anyhow!("Time to quit.")),
            }
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq)]
    enum NestedRoutes {
        WentDeep,
    }

    impl Matcher<RoutesTable> for NestedRoutes {
        fn match_path(&self) -> anyhow::Result<RoutesTable> {
            Ok(RoutesTable::Exit)
        }
    }

    // #[tokio::test]
    // async fn motivating_example() -> RouterResult {
    //     let mut router = Router::new(RoutesTable::Somewhere);

    //     let after_somewhere = router.init()?;
    //     assert_eq!(after_somewhere, RoutesTable::AfterSomewhere);

    //     let last_location = router.navigate(after_somewhere)?;
    //     assert_eq!(
    //         last_location,
    //         RoutesTable::GoingDeeper(NestedRoutes::WentDeep)
    //     );

    //     let should_be_exit = router.navigate(last_location)?;
    //     assert_eq!(should_be_exit, RoutesTable::Exit);

    //     Ok(())
    // }

    #[tokio::test]
    #[ignore]
    async fn router_processing_loop() {
        let router = Router::new(RoutesTable::Somewhere);

        match router.run() {
            Err(RouterError::Exit(s)) => assert_eq!(&s, "Time to quit."),
            Err(e) => panic!("Threw non-exit error: {e:?}"),
            Ok(v) => panic!("Ended as Ok({v:?}) when expected Err('Time to quit.')"),
        }
    }
}
