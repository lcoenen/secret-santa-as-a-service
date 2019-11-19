extern crate hyper;

use hyper::{Body, Request, Response};

pub fn handle(_req: Request<Body>) -> Response<Body> {
    Response::new(Body::from("Started"))
}
