use std::sync::Arc;
use futures::{finished, Future};
use hyper::{self, Get as GetMethod, Post as PostMethod};
use hyper::server::{Service, Request, Response};
use route_recognizer::{Router as Recognizer};
use errors::*;
use super::{HttpFuture, Get, Post, Route};

type HttpRouter<T> = Recognizer<Box<T>>;

/// A `hyper` service that routes requests to child handlers.
///
/// This structure is relatively cheap to clone; it only needs to
/// increment a single [`Arc`]() pointer.
#[derive(Clone)]
pub struct Router {
    routers: Arc<Box<Routers>>,
}

struct Routers {
    get_router: HttpRouter<Get>,
    post_router: HttpRouter<Post>,
}

pub struct RouterBuilder {
    get_router: HttpRouter<Get>,
    post_router: HttpRouter<Post>,
}

impl RouterBuilder {
    /// Create a new router builder.
    pub fn new() -> Self {
        RouterBuilder {
            get_router: HttpRouter::new(),
            post_router: HttpRouter::new(),
        }
    }

    /// Add a new handler for a `GET` request.
    pub fn get<H>(mut self, handler: H) -> Self
        where H: Get + Route + 'static
    {
        self.get_router.add(H::ROUTE, Box::new(handler));

        self
    }

    /// Add a new handler for a `POST` request.
    pub fn post<H>(mut self, handler: H) -> Self
        where H: Post + Route + 'static
    {
        self.post_router.add(H::ROUTE, Box::new(handler));

        self
    }

    /// Build a `Router`.
    ///
    /// This function consumes the builder and returns a new
    /// immutable router with the given handlers.
    pub fn build(self) -> Router {
        Router {
            routers: Arc::new(Box::new(Routers {
                get_router: self.get_router,
                post_router: self.post_router,
            })),
        }
    }
}

impl Service for Router {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = HttpFuture;

    fn call(&self, req: Request) -> Self::Future {
        match *req.method() {
            GetMethod => self.get(req),
            PostMethod => self.post(req),
            _ => box finished(ErrorKind::MethodNotSupported.into()),
        }
    }
}

impl Router {
    fn get(&self, req: Request) -> <Self as Service>::Future {
        let route = {
            let path = req.path().unwrap_or("");
            self.routers.get_router.recognize(path)
        };

        match route {
            Ok(route) => {
                let handler = route.handler;
                let params = route.params;

                handler.call(params, req)
            }
            Err(_) => box finished(ErrorKind::NoRouteMatch.into()),
        }
    }

    fn post(&self, req: Request) -> <Self as Service>::Future {
        let route = {
            let path = req.path().unwrap_or("");
            self.routers.post_router.recognize(path)
        };

        match route {
            Ok(route) => {
                let handler = route.handler;
                let params = route.params;

                handler.call(params, req)
            }
            Err(_) => box finished(ErrorKind::NoRouteMatch.into()),
        }
    }
}

/// A conversion trait for handler futures.
///
/// This is a convenience trait for taking any future with a `Response`
/// or application `Error` and converting it into a future of a `Response`
/// or `hyper::Error`.
/// A failed future is converted into a successful one, but with an appropriate
/// HTTP status code derived from the error.
///
/// This particular trait is quite effective because it allows a wide range of
/// input values, and Rust's type inference will hide the details away for us.
/// It also helps avoid boxing response futures multiple times.
///
/// You can read the implementors section as _implement `IntoHttpFuture` for all
/// types `F`, where `F` is a `Future`, and `F`'s `Item` type can be converted into
/// a `Response`, and `F`'s `Error` type can be converted into an `Error`_.
pub trait IntoHttpFuture {
    fn into_http_future(self) -> HttpFuture;
}

impl<F> IntoHttpFuture for F
    where F: Future + 'static,
          F::Item: Into<Response>,
          F::Error: Into<Error>
{
    fn into_http_future(self) -> HttpFuture {
        box self.then(|response| {
            finished(match response {
                Ok(response) => response.into(),
                Err(e) => e.into().into(),
            })
        })
    }
}
