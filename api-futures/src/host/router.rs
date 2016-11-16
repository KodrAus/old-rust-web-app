use std::sync::Arc;
use futures::{ finished, Future };
use hyper::{self, Get as GetMethod, Post as PostMethod};
use hyper::server::{Service as HyperService, Request, Response};
use route_recognizer::Router as Recognizer;
use errors::*;
use super::{HttpFuture, Get, Post, Route};

type HttpRouter<T> = Recognizer<Box<T>>;

/// A `hyper` service that routes requests to child handlers.
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
    pub fn new() -> Self {
        RouterBuilder {
            get_router: HttpRouter::new(),
            post_router: HttpRouter::new(),
        }
    }

    pub fn get<H>(mut self, handler: H) -> Self
        where H: Get + Route + 'static
    {
        self.get_router.add(H::ROUTE, Box::new(handler));

        self
    }

    pub fn post<H>(mut self, handler: H) -> Self
        where H: Post + Route + 'static
    {
        self.post_router.add(H::ROUTE, Box::new(handler));

        self
    }

    pub fn build(self) -> Router {
        Router {
            routers: Arc::new(Box::new(
                Routers {
                    get_router: self.get_router,
                    post_router: self.post_router,
                }
            ))
        }
    }
}

impl HyperService for Router {
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
    fn get(&self, req: Request) -> <Self as HyperService>::Future {
        let path = req.path().unwrap_or("").to_owned();

        match self.routers.get_router.recognize(&path) {
            Ok(route) => {
                let handler = route.handler;
                let params = route.params;

                handler.call(params, req)
            },
            Err(_) => {
                box finished(ErrorKind::NoRouteMatch(path).into())
            }
        }
    }

    fn post(&self, req: Request) -> <Self as HyperService>::Future {
        let path = req.path().unwrap_or("").to_owned();
        
        match self.routers.post_router.recognize(&path) {
            Ok(route) => {
                let handler = route.handler;
                let params = route.params;

                handler.call(params, req)
            },
            Err(_) => {
                box finished(ErrorKind::NoRouteMatch(path).into())
            }
        }
    }
}

pub trait IntoResponse {
    fn into_response(self) -> HttpFuture;
}

impl <F> IntoResponse for F where
F: Future + 'static,
F::Item: Into<Response>,
F::Error: Into<Error> {
    fn into_response(self) -> HttpFuture {
        box self.then(|response| {
            finished(match response {
                Ok(response) => response.into(),
                Err(e) => e.into().into()
            })
        })
    }
}