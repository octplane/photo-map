use websocket::sync::Server;
use websocket::Message;
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};
use shared::*;

pub fn start_ws(ip_port: &str, to_coordinator: Sender<CoordinatorMessage>) {
	let ws_server = Server::bind(ip_port).unwrap();
	println!("Serving Websocket on: {}", ip_port);

    for connection in ws_server.filter_map(Result::ok) {
		let toc = to_coordinator.clone();
		thread::spawn(move|| {
			let (sender, receiver) = channel::<String>();
			toc.send(CoordinatorMessage::NewClient(sender)).unwrap();

			let mut client = connection.use_protocol("rust-websocket").accept().unwrap();

			let ip = client.peer_addr().unwrap();
			println!("Connection from {}", ip);

			loop {
				let message = receiver.recv().unwrap();
				let ws_message = Message::text(message.clone());
				println!("Received {}", message);
				client.send_message(&ws_message).unwrap();
			}
		});
    }
}
