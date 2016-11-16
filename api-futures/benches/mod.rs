#![feature(test, associated_consts)]
extern crate test;
use test::Bencher;

extern crate hyper;
extern crate webapp_demo;

use hyper::server::Request;
use webapp_demo::host::*;

macro_rules! service {
    ($route:expr => $ident:ident) => (
        struct $ident;
        impl Route for $ident {
            const ROUTE: &'static str = $route;
        }

        impl Get for $ident {
            fn call(&self, _: &Params, _: Request) -> HttpFuture {
                unimplemented!()
            }
        }

        impl Post for $ident {
            fn call(&self, _: &Params, _: Request) -> HttpFuture {
                unimplemented!()
            }
        }
    )
}

service!("/" => Root);
service!("/a" => A);
service!("/a/b" => B);
service!("/a/b/c" => C);
service!("/a/b/c/d" => D);

#[bench]
fn clone_router_0(b: &mut Bencher) {
    let router = RouterBuilder::new().build();

    b.iter(|| {
        test::black_box(router.clone())
    })
}

#[bench]
fn clone_router_6(b: &mut Bencher) {
    let router = RouterBuilder::new()
        .get(Root)
        .get(A)
        .get(B)
        .post(Root)
        .post(A)
        .post(B)
        .build();

    b.iter(|| {
        test::black_box(router.clone())
    })
}

#[bench]
fn clone_router_10(b: &mut Bencher) {
    let router = RouterBuilder::new()
        .get(Root)
        .get(A)
        .get(B)
        .get(C)
        .get(D)
        .post(Root)
        .post(A)
        .post(B)
        .post(C)
        .post(D)
        .build();

    b.iter(|| {
        test::black_box(router.clone())
    })
}