// use boa;
use wasm_bindgen::{JsValue};
use crate::GameState;
// use web_sys::{WebSocket, ErrorEvent, MessageEvent};
// use wasm_bindgen::closure::Closure;
use serde_json;
use serde::{Deserialize, Serialize};
use serde_repr;
use std::str::FromStr;

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum MessageTypes {
    Move=0,
    WannaPlay=1,
    GetAvailablePlayers=2,
    Unknown = 0xFF,
}

impl MessageTypes {
    fn value(&self) -> u8 {
        match *self {
            MessageTypes::Unknown => 0xFF,
            MessageTypes::Move => 0,
            MessageTypes::WannaPlay => 1,
            MessageTypes::GetAvailablePlayers => 2,
        }
    }
}

impl From<u8> for MessageTypes {
    fn from(orig: u8) -> Self {
        match orig {
            0 => MessageTypes::Move,
            1 => MessageTypes::WannaPlay,
            2 => MessageTypes::GetAvailablePlayers,
            _ => MessageTypes::Unknown
        }
    }
}

impl From<&str> for MessageTypes {
    fn from(orig: &str) -> Self {
        let id = u8::from_str(orig);
        match id {
            Ok(0) => MessageTypes::Move,
            Ok(1) => MessageTypes::WannaPlay,
            Ok(2) => MessageTypes::GetAvailablePlayers,
            _ => MessageTypes::Unknown
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct UserMove {
    tempid: u128,
    move_pos: u8,
}


pub enum ClientState {
    FREE_TO_PLAY,
    ACTIVELY_PLAYING,
    AWAITING_RESPONSE,
}


#[derive(Serialize)]
pub struct MoveMessage {
    user_id: u128,
    player_move: u8
}



pub struct ConnectionManager {
    user_id: u128,
    last_state: GameState,

    /// Web socket for communication with the backend when two human players are playing
    zmq_socket: Option<zmq::Socket>,
}

impl ConnectionManager {
    /// Send the move to the server and wait for acknowledgement. If ACK arrives, make the move
    /// locally and return the new state.
    pub fn make_move(&mut self, move_position: u8) -> Result<GameState, JsValue> {
        let move_msg = MoveMessage {
            user_id: self.user_id,
            player_move: move_position
            };

        let mut response_buffer: [u8;1];
        if let Some(zmq_socket) = &self.zmq_socket {
            zmq_socket.send(format!("{}{}", MessageTypes::Move.value(), serde_json::to_string(&move_msg).unwrap()).as_str(), 0);
            let response = zmq_socket.recv_into(&mut response_buffer, 0);
            match response {
                // everything is fine
                Ok(1) => {
                    // first, make the move and then return updated state to the UI
                    let new_state_maybe = self.last_state.make_move(move_position as usize);
                    return match new_state_maybe {
                        Ok(new_state) => {
                            self.last_state = new_state.clone();
                            Ok(self.last_state.clone())
                        },
                        Err(e) => Err(JsValue::from_str(format!("Error while making move locally: {:?}", e).as_str()))
                    }
                },
                // error
                _ => return Err(JsValue::from_str("Error while sending the move to the server.")),
            }
        }

        Err(JsValue::from_str("No connection to the server."))
    }

    pub fn get_last_state(&self) -> GameState {
        self.last_state.clone()
    }

    // From here, the part with functions for communication over websocket starts
    pub fn connect_to_server(&mut self, url: &str) -> Result<(), JsValue> {
        let context = zmq::Context::new();
        let requester = context.socket(zmq::REQ).unwrap();
        match requester.connect("tcp://localhost:5555") {
            Ok(_) => (),
            Err(e) => return Err(JsValue::from_str(format!("Could not connect to the server {}. Error: {:?}", url, e).as_str()))
        }
        self.zmq_socket = Some(requester);
        Ok(())
    }



}