//! A simple, generalized, & nestable routing solution, capable of sending a given Request to the
//! appropriate Route (or sub-Router) for handling & of processing the result returned.

use std::future::Future;

#[cfg(test)]
#[tokio::test]
async fn motivating_example() -> anyhow::Result<()> {
    use anyhow::anyhow;

    #[derive(Debug, PartialEq)]
    enum RoutesTable {
        Somewhere,
        AfterSomewhere,
        GoingDeeper(ANestedRouter),
        Exit,
    }

    // TODO: should this be a builder pattern using a HashMap<RouteEnum, RouteHandler>?
    // if not, there's no point to all the boxing I feel like if my intention is simply to execute
    // the fn provided in the match arm higher up...I might as well just exec it here & require
    // that Matchers return a <Route> in each match arm instead and call my handlers in the match
    // arms, even if they're defined elsewhere...
    impl Matcher for RoutesTable {
        fn match_path(&self) -> RouteHandler<RoutesTable> {
            match self {
                Self::Somewhere => RouteHandler::new(|| async { Ok(Self::AfterSomewhere) }),
                Self::AfterSomewhere => {
                    RouteHandler::new(|| async { Ok(Self::GoingDeeper(ANestedRouter::WentDeep)) })
                }
                Self::GoingDeeper(deeper) => deeper.match_path(),
                Self::Exit => RouteHandler::new(|| async { Err(anyhow!("Time to quit")) }),
            }
        }
    }

    #[derive(Debug, PartialEq)]
    enum ANestedRouter {
        WentDeep,
    }

    impl Matcher<RoutesTable> for ANestedRouter {
        fn match_path(&self) -> RouteHandler<RoutesTable> {
            RouteHandler::new(|| async { Ok(RoutesTable::Exit) })
        }
    }

    let mut router = Router::new(RoutesTable::Somewhere);

    let somewhere_handler = router.init();
    let after_somewhere = somewhere_handler();
    assert_eq!(after_somewhere, RoutesTable::AfterSomewhere);

    let after_somewhere_handler = router.navigate(after_somewhere);
    let last_location = after_somewhere_handler();
    assert_eq!(
        last_location,
        RoutesTable::GoingDeeper(ANestedRouter::WentDeep)
    );

    let last_handler = router.navigate(last_location);
    let should_be_exit = last_handler();
    assert_eq!(should_be_exit, RoutesTable::Exit);

    Ok(())
}

// Okay let's pause a moment to think about this:
//
// A Router is the core feature holding a ref to the routes table
// Routes tables can be nested, if desired
// A routes table handles a route request by matching itself to the handler associated with the
// variant it is
//
// A routes table is told to match itself when a Router receives some sort of Request & the
// RoutesTable variant's handler should return (& bubble up) a Response to the appropriate
// Router/RoutesTable?
// ...not sure exactly what this looks like yet, but maybe something where the Response is first
// checked if matched by the RoutesTable the original Request was handled, then bubbled up if no
// match...
// A Router has no need to know a current location?

pub struct Router<Routes: Matcher> {
    location: Routes,
}

impl<Routes: Matcher> Router<Routes> {
    fn new(initial_path: Routes) -> Self {
        Self {
            location: initial_path,
        }
    }

    fn init(&self) -> RouteHandler<Routes> {
        self.match_path()
    }

    fn navigate(&mut self, path: Routes) -> RouteHandler<Routes> {
        self.location = path;
        self.match_path()
    }
}

impl<Routes: Matcher> Matcher<Routes, Routes> for Router<Routes> {
    fn match_path(&self) -> RouteHandler<Routes> {
        self.location.match_path()
    }
}

pub trait Matcher<Root = Self, Routes = Self> {
    fn match_path(&self) -> RouteHandler<Root>;
}

type Handler<R> = Box<dyn Fn() -> HandlerReturn<R> + Send + Sync + 'static>;
type HandlerReturn<R> = Box<dyn Future<Output = anyhow::Result<R>> + Send + 'static>;

pub struct RouteHandler<ReturnType> {
    handler: Handler<ReturnType>,
}
impl<ReturnType> RouteHandler<ReturnType> {
    pub fn new<H, R>(handler: H) -> Self
    where
        H: Fn() -> R + Send + Sync + 'static,
        R: Future<Output = anyhow::Result<ReturnType>> + Send + 'static,
    {
        Self {
            handler: Box::new(move || Box::new(handler())),
        }
    }
}

// #[async_trait]
// pub trait Handler<Next: Matcher> {
//     async fn handle(self) -> anyhow::Result<Next>;
// }

// #[async_trait]
// pub trait HandleRoute<Route> {
//     async fn handle(&self, path: Route) -> anyhow::Result<()>;
// }
//
// /// All Route Matchers are also Route Handlers, making nested Routers easier
// #[async_trait]
// impl<M: Matcher + Send + Sync> HandleRoute<M> for M {
//     async fn handle(path: M) -> anyhow::Result<()> {
//         self.match_path(path).await
//     }
// }
