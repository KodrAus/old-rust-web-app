use std::sync::Arc;
use futures;
use hyper::{ self, StatusCode, Get as GetMethod, Post as PostMethod };
use hyper::server::{ Service as HyperService, Request, Response };
use route_recognizer::Router as Recognizer;
use super::{ HttpFuture, Get, Post };

type HttpRouter<T> = Recognizer<Box<T>>;

#[derive(Clone)]
pub struct Router {
    get_router: Arc<HttpRouter<Get>>,
    post_router: Arc<HttpRouter<Post>>
}

pub struct RouterBuilder {
    get_router: HttpRouter<Get>,
    post_router: HttpRouter<Post>
}

impl RouterBuilder {
    pub fn new() -> Self {
        RouterBuilder {
            get_router: HttpRouter::new(),
            post_router: HttpRouter::new()
        }
    }

    pub fn get<H>(mut self, handler: H) -> Self where 
    H: Get + 'static {
        self.get_router.add(handler.route(), Box::new(handler));

        self
    }

    pub fn post<H>(mut self, handler: H) -> Self where 
    H: Post + 'static {
        self.post_router.add(handler.route(), Box::new(handler));

        self
    }

    pub fn build(self) -> Router {
        Router {
            get_router: Arc::new(self.get_router),
            post_router: Arc::new(self.post_router)
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
            _ => return box futures::finished(Response::new().status(StatusCode::MethodNotAllowed))
        }
    }
}

impl Router {
    fn get(&self, req: Request) -> <Self as HyperService>::Future {
        match self.get_router.recognize(req.path().unwrap()) {
            Ok(route) => {
                let handler = route.handler;
                let params = route.params;

                handler.call(params, req)
            },
            Err(_) => box futures::finished(Response::new().status(StatusCode::NotFound))
        }
    }

    fn post(&self, req: Request) -> <Self as HyperService>::Future {
        match self.post_router.recognize(req.path().unwrap()) {
            Ok(route) => {
                let handler = route.handler;
                let params = route.params;

                handler.call(params, req)
            },
            Err(_) => box futures::finished(Response::new().status(StatusCode::NotFound))
        }
    }
}