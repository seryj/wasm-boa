extern crate zmq;

use std::{thread, time};
use serde_json;
use serde::{Serialize, Deserialize};
use serde_repr::*;
use zmq::Message;
use std::str::FromStr;


fn main() {
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();
    assert!(responder.bind("tcp://*:5555").is_ok());

    loop {
        let msg_maybe = responder.recv_string(0);
        match msg_maybe {
            // UTF 8 conform message
            Ok(Ok(msg)) => {
                eprintln!("received message: {:?}", msg);
                let message_type: MessageTypes = msg[0..1].into();
                let msg_data = &msg[1..];
                match message_type {
                    MessageTypes::Move => {
                        let usermove_maybe: Result<UserMove, _> = serde_json::from_str(msg_data);
                        match usermove_maybe {
                            Ok(players_move) => {
                                eprintln!("Received move: {:?}", &players_move);
                                ()
                            },
                            Err(e) => {eprintln!("something went completely wrong while extracting the move: {:?}", e)}
                        }
                    },
                    MessageTypes::GetAvailablePlayers => {
                        // responder.send_multipart(vec![vec![message_type.value()], vec![0,1,2,3,4,5]], 0);
                    },

                    MessageTypes::WannaPlay => {
                        // responder.send_multipart(vec![vec![message_type.value()], vec![1,2,3]], 0);
                    },

                    MessageTypes::Unknown => {

                    }
                }
                responder.send(1, 0); // ACK
            },
            Ok(Err(e)) => {
                eprintln!("something went wrong while extracting string from message, {:?}", e);
                responder.send(0, 0); // NACK
            }
            Err(e) => {
                eprintln!("something went completely wrong while getting message, {:?}", e);
                responder.send(100, 0); // NACK
            }
        }

        thread::sleep(time::Duration::from_millis(100)); // do not consume all CPU...
    }
}