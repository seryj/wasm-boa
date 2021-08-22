extern crate zmq;

use std::{thread, time};

fn main() {
    println!("Connecting to hello world server...");
    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();
    assert!(requester.connect("tcp://localhost:5555").is_ok());

    let msg_type = "0";
    for request_nbr in 0..10 {
        println!("Trying to send request number: {}", request_nbr);
        // requester.send_multipart(vec![vec![0], format!("{{\"tempid\": 123, \"move_pos\": {}}}", request_nbr).as_bytes().to_vec()], 0);

        requester.send(format!("{}{{\"tempid\": 123, \"move_pos\": {}}}", msg_type, request_nbr).as_str(), 0).expect("Could not send the message :(");
        let response = requester.recv_string(0);
        match response {
            Ok(Ok(r)) => assert_eq!(r, "ACK"),
            e => {eprintln!("Something went wrong: {:?}", e)}
        }
        println!("Request sent");
        thread::sleep(time::Duration::from_millis(1000)); // do not consume all CPU...
    }
}