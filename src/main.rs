use std::future::{self, Ready};

use actix_session::Session;
use actix_web::{
    dev::{self, Body, Payload},
    get,
    http::StatusCode,
    post,
    web::{self, Form},
    App, Error, FromRequest, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use futures_util::future::ok;
use serde::Deserialize;

#[get("/")]
async fn index(session: Session, request: HttpRequest) -> Result<HttpResponse> {
    dbg!(request);
    Ok(HttpResponse::build(StatusCode::OK).body("hello from new actix_web!!!!!".to_owned()))
}

#[get("/games")]
async fn games(session: Session, request: HttpRequest) -> Result<HttpResponse> {
    dbg!(request);
    Ok(HttpResponse::build(StatusCode::OK).body("games!!!!!".to_owned()))
}

#[derive(Debug, Deserialize)]
struct GameData {
    pub message: String,
}

impl FromRequest for GameData {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut dev::Payload) -> Self::Future {
        match payload {
            Payload::None => {
                dbg!("none");
            }
            Payload::H1(h1) => {
                dbg!(h1.readany());
            }
            Payload::H2(h2) => {
                dbg!("h2");
            }
            Payload::Stream(stream) => {
                dbg!("stream");
            }
        }
        future::ready(Ok(GameData {
            message: "wat".to_owned(),
        }))
    }
}

#[post("/games")]
async fn create_game(supplied_thing: Result<GameData>) -> Result<HttpResponse> {
    dbg!(supplied_thing);
    Ok(HttpResponse::build(StatusCode::OK).body("posting games!!!!!".to_owned()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(games)
            .service(create_game)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
