//! A simple, generalized, & nestable routing solution, capable of sending a given Request to the
//! appropriate Route (or sub-Router) for handling & of processing the result returned.

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

    pub fn init(&self) -> anyhow::Result<Routes> {
        self.match_path()
    }

    pub fn navigate(&mut self, path: Routes) -> anyhow::Result<Routes> {
        self.location = path;
        self.match_path()
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

    pub fn run(mut self) -> Result<Routes, RouterError> {
        self.state = RouterState::Running(RunningState::Listening);

        // TODO: instead of trying to coerce everything to return a Route, even when it doesn't
        // make sense, why not also impl an event dispatcher on the Router?
        // then this can call self.dispatch on each loop iteration, issuing a `Navigate(next)` on
        // successful routings or successful error recovery, and a `Exit` on Program exit?
        // then error handler just needs to return a RouterEvent type
        while self.is_running() {
            self.location = match self.match_path() {
                Ok(next) => next,
                // TODO: IDEA: maybe instead of impl ErrorHandler for Router, I can use
                // TryFrom<impl Into<RouterError>> for RouterEvent?
                //         ^^^ Not sure this exactly will work, but could always first
                //             RouterError::from(err), then RouterEvent::from(that err)...
                Err(err) => match Router::<Routes>::handle_error(err, self.location) {
                    Ok(next) => next,
                    Err(RouterError::Exit(msg)) => self.exit(msg)?,
                },
            }
        }

        Ok(self.location)
    }
}

enum RouterState<Routes> {
    Initializing,
    Idle,
    Running(RunningState<Routes>),
    Exiting,
}

enum RunningState<Routes> {
    Listening,
    Navigating(Routes),
}

pub enum RouterEvent<Routes> {
    Goto(Routes),
    Cancel,
    Error(RouterError),
    Exit(String),
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

pub trait ErrHandler<R, L = R> {
    fn handle_error(err: impl Into<RouterError>, current_location: L) -> Result<R, RouterError>;
}

impl<Routes: Copy + Matcher> ErrHandler<Routes> for Router<Routes> {
    fn handle_error(
        err: impl Into<RouterError>,
        current_location: Routes,
    ) -> Result<Routes, RouterError> {
        match err.into() {
            // unhandled errors cancel navigation & keep router at current location?
            RouterError::Unhandled(err) => {
                error!("Unhandled error caught at router: {err:#?}");
                Ok(current_location)
            }

            // TODO: irrecoverable errors cancel navigation & trigger program exit

            // program exit causes program exit
            RouterError::Exit(msg) => Err(RouterError::Exit(msg)),
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

    #[tokio::test]
    async fn motivating_example() -> anyhow::Result<()> {
        let mut router = Router::new(RoutesTable::Somewhere);

        let after_somewhere = router.init()?;
        assert_eq!(after_somewhere, RoutesTable::AfterSomewhere);

        let last_location = router.navigate(after_somewhere)?;
        assert_eq!(
            last_location,
            RoutesTable::GoingDeeper(NestedRoutes::WentDeep)
        );

        let should_be_exit = router.navigate(last_location)?;
        assert_eq!(should_be_exit, RoutesTable::Exit);

        Ok(())
    }

    #[tokio::test]
    async fn router_processing_loop() {
        let router = Router::new(RoutesTable::Somewhere);

        match router.run() {
            Err(RouterError::Exit(s)) => assert_eq!(&s, "Time to quit."),
            Err(e) => panic!("Threw non-exit error: {e:?}"),
            Ok(v) => panic!("Ended as Ok({v:?}) when expected Err('Time to quit.')"),
        }
    }
}
