extern crate ws;
extern crate iron;
extern crate staticfile;
extern crate mount;

mod http_server;

use http_server::start_service;

fn main() {
    start_service("0.0.0.0:8080");
}
