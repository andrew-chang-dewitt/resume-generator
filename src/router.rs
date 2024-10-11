//! A simple, generalized, & nestable routing solution, capable of sending a given Request to the
//! appropriate Route (or sub-Router) for handling & of processing the result returned.

use std::{error::Error, fmt::write, future::Future, marker::PhantomData};

pub struct Router<Routes: Matcher, ErrHandler = DefaultErrorHandler> {
    location: Routes,
    err_handler: PhantomData<ErrHandler>,
}

// TODO: I'm sure there's a better way to do this, but need to read more on implementing errors &
// error handling better in Rust
#[derive(Debug)]
enum RouterError {
    Exit(String),
    Unhandled(anyhow::Error),
}

impl From<anyhow::Error> for RouterError {
    fn from(value: anyhow::Error) -> Self {
        RouterError::Unhandled(value)
    }
}

trait ErrHandler<R> {
    fn handle(err: RouterError) -> Result<R, RouterError>;
}

struct DefaultErrorHandler;
impl<R> ErrHandler<R> for DefaultErrorHandler {
    fn handle(err: RouterError) -> Result<R, RouterError> {
        todo!()
    }
}

impl<Routes: Matcher> Router<Routes> {
    fn new(initial_path: Routes) -> Self {
        Self {
            location: initial_path,
            err_handler: PhantomData,
        }
    }

    fn init(&self) -> anyhow::Result<Routes> {
        self.match_path()
    }

    fn navigate(&mut self, path: Routes) -> anyhow::Result<Routes> {
        self.location = path;
        self.match_path()
    }

    pub fn run(initial_path: Routes) -> Result<Routes, RouterError> {
        let mut r = Router::new(initial_path);

        loop {
            match r.match_path() {
                Ok(next) => r.location = next,
                Err(err) => {
                    r.location = <DefaultErrorHandler as ErrHandler<Routes>>::handle(err.into())?
                }
            }
        }
    }
}

impl<Routes: Matcher> Matcher<Routes, Routes> for Router<Routes> {
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

    #[derive(Debug, PartialEq)]
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

    #[derive(Debug, PartialEq)]
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
        match Router::run(RoutesTable::Somewhere) {
            Err(RouterError::Exit(s)) => assert_eq!(&s, "Time to quit."),
            Err(e) => panic!("Threw non-exit error: {e:?}"),
            Ok(v) => panic!("Ended as Ok({v:?}) when expected Err('Time to quit.')"),
        }
    }
}
