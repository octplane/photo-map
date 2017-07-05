extern crate websocket;
extern crate hyper;
use hyper::server::{Server, Request, Response};
use websocket::Message;
use websocket::sync::server::upgrade::IntoWs;
use websocket::sync::server::upgrade::HyperRequest;
fn main() {
    Server::http("0.0.0.0:80").unwrap().handle(move |req: Request, res: Response| {
        match HyperRequest(req).into_ws() {
            Ok(upgrade) => {
                // `accept` sends a successful handshake, no need to worry about res
                let mut client = match upgrade.accept() {
                    Ok(c) => c,
                    Err(_) => panic!(),
                };

                client.send_message(&Message::text("its free real estate"));
            },

            Err((request, err)) => {
                // continue using the request as normal, "echo uri"
                res.send(b"Try connecting over ws instead.").unwrap();
            },
        };
    })
    .unwrap();
}