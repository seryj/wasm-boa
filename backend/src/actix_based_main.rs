use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, http};
use actix_web_actors::ws;

use boalib;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct User {
    temp_id: u128,
    username: String,
}

type UserGames = Arc<Mutex<HashMap<User, boalib::GameState>>>;

/// Define HTTP actor
struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}
/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        println!("Something connected");
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) =>{
                // ctx.text(text)
                eprintln!("{}", text)
            },
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn websocket(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}

async fn login(req: HttpRequest) -> Result<HttpResponse, Error> {
    let resp = actix_web::HttpResponse::new(http::StatusCode::ACCEPTED);
    println!("{:?}", &resp);
    Ok(resp)
}

async fn wanna_play_with(req: HttpRequest) -> Result<HttpResponse, Error> {
    let resp = actix_web::HttpResponse::new(http::StatusCode::ACCEPTED);
    println!("{:?}", &resp);
    Ok(resp)
}

async fn online_users(req: HttpRequest) -> Result<HttpResponse, Error> {
    let resp = actix_web::HttpResponse::new(http::StatusCode::ACCEPTED);
    println!("{:?}", &resp);
    Ok(resp)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("will start server...");
    HttpServer::new(||
            App::new()
                .route("/ws/", web::get().to(websocket))
                .route("/login/", web::get().to(login))
                .route("/wannaplaywith/{tid}", web::get().to(wanna_play_with))
                .route("/onlineusers/", web::get().to(online_users))
    )
        .bind("127.0.0.1:18080")?
        .run()
        .await
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
