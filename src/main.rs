extern crate websocket;

use std::thread;
use websocket::sync::Server;
use websocket::OwnedMessage;

fn main() {
  let server = Server::bind("127.0.0.1:3000").unwrap();

  for request in server.filter_map(Result::ok) {
    thread::spawn(|| {
      let mut client = request.use_protocol("rust-websocket").accept().unwrap();
      let ip = client.peer_addr().unwrap();

      println!("New connection from {}", ip);

      let outmsg = OwnedMessage::Text("Test".to_string());
      client.send_message(&outmsg).unwrap();

      let (mut receiver, mut sender) = client.split().unwrap();

			for message in receiver.incoming_messages() {
				let message = message.unwrap();

				match message {
					OwnedMessage::Close(_) => {
						let message = OwnedMessage::Close(None);
						sender.send_message(&message).unwrap();
						println!("Client {} disconnected", ip);
						return;
					}
					OwnedMessage::Ping(ping) => {
						let message = OwnedMessage::Pong(ping);
						sender.send_message(&message).unwrap();
					}
          OwnedMessage::Text(t) => {
            println!("Message: {}", t);
          }
          _ => println!("Unhandled message type"),
				}
			}
    });
  }
}
