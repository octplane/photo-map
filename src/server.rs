extern crate websocket;
extern crate iron;
extern crate staticfile;
extern crate mount;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::{thread, time};

mod shared;
mod http_server;
mod websocket_server;

use http_server::start_http;
use websocket_server::start_ws;
use shared::*;



fn fake_emitter(sender: Sender<CoordinatorMessage>, receiver: Receiver<CoordinatorMessage>) {

    thread::spawn(move|| {
        loop {
            sender.send(CoordinatorMessage::Timeout()).unwrap();
            thread::sleep(time::Duration::from_secs(1));
        }
    });
    thread::spawn(move|| {
        let mut clients: Vec<Sender<String>> = vec![];
        loop {
            match receiver.recv() {
                Ok(CoordinatorMessage::Timeout()) => { println!("Time to send some data!") }
                Ok(CoordinatorMessage::NewClient(s)) => {
                    println!("A new client !");
                    clients.push(s);
                    }
                Err(e) => { panic!("An error occured: {:?}", e)}
            }
        }
    });
}


fn main() {
    let (to_coord, coord_receiver) = channel::<CoordinatorMessage>();
    fake_emitter(to_coord.clone(), coord_receiver);
    let _listening = start_http("0.0.0.0:8080");
    start_ws("0.0.0.0:8081", to_coord);
}
