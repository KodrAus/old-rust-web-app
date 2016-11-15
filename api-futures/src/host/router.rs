use std::sync::Arc;
use futures;
use hyper::{ self, Get, Post, StatusCode };
use hyper::server::{ Service as HyperService, Request, Response };
use route_recognizer::Router as Recognizer;
use super::{ HttpFuture, Service };

type HttpRouter = Recognizer<Arc<Box<Service>>>;

#[derive(Clone)]
pub struct Router {
    get_router: HttpRouter,
    post_router: HttpRouter
}

impl Router {
    pub fn new() -> Self {
        Router {
            get_router: HttpRouter::new(),
            post_router: HttpRouter::new()
        }
    }

    pub fn get<H>(&mut self, handler: H) where 
    H: Service + 'static {
        self.get_router.add(handler.route(), Arc::new(Box::new(handler)))
    }

    pub fn post<H>(&mut self, handler: H) where 
    H: Service + 'static {
        self.post_router.add(handler.route(), Arc::new(Box::new(handler)))
    }
}

impl HyperService for Router {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = HttpFuture;

    fn call(&self, req: Request) -> Self::Future {
        let router = match *req.method() {
            Get => &self.get_router,
            Post => &self.post_router,
            _ => return box futures::finished(Response::new().status(StatusCode::MethodNotAllowed))
        };
        
        match router.recognize(req.path().unwrap()) {
            Ok(route) => {
                let handler = route.handler;
                let params = route.params;

                handler.call(params, req)
            },
            Err(_) => box futures::finished(Response::new().status(StatusCode::NotFound))
        }
    }
}