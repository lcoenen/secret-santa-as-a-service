#![deny(warnings)]
extern crate hyper;

mod subscribe;
mod start;
mod check;

use hyper::{Body, Request, Response, Server};
use hyper::service::service_fn_ok;
use hyper::rt::{self, Future};

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr)
        .serve(|| {
            service_fn_ok(move |req: Request<Body>| {

                println!("Request is {} {}", req.method(), req.uri().path());

                if req.uri().path() == "/subscribe" {
                    return subscribe::handle(req);
                }
                else if req.uri().path() == "/start" {
                    return start::handle(req);
                }
                else if req.uri().path() == "/check" {
                    return check::handle(req);
                }

                Response::new(Body::from("404"))
            })
        })
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    rt::run(server);
}
