extern crate hyper;

use hyper::{Body, Request, Response};
use crate::hyper::rt::Stream;
use std::str;
use crate::hyper::rt::Future;

pub fn handle(_req: Request<Body>) -> Response<Body> {
    let (_parts, body) = _req.into_parts();
    let chunks = body.collect().wait().unwrap(); 
    let strings: Vec<String> = chunks.into_iter().map(|chunk| {

        let buf: Vec<u8> = chunk.into_bytes().into_iter().collect();
        String::from(str::from_utf8(&buf).unwrap())
    }).collect();
    let request_string = String::from(strings.concat());
    print!("Request is {}", request_string);

    Response::new(Body::from("Subscribed"))
}
