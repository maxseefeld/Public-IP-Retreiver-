use std::io::{self, Write};
use std::net::{Ipv4Addr, SocketAddrV4};
use hyper::{Body, Request, Response, Server};
use hyper::rt::{self, Future};

fn main() {
    let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 3000);

    let server = Server::bind(&addr.into())
        .serve(|| {
            hyper::service::service_fn_ok(|_req: Request<Body>| {
                Response::new(Body::from(get_public_ip()))
            })
        })
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);
    rt::run(server);
}

fn get_public_ip() -> String {
    let resp = reqwest::blocking::get("https://api.ipify.org").unwrap();
    let body = resp.text().unwrap();
    body
}
