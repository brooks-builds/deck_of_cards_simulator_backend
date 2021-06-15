use rocket::async_stream::yielder::Sender;
use rocket::response;
use rocket::response::content::Html;
use rocket::response::stream::EventStream;
use rocket::tokio::time::{self, Duration};
use rocket::Shutdown;
use rocket::State;
use rocket::{
    form::{Form, Lenient, Strict},
    http::{ContentType, Status},
    response::{
        content::Json,
        stream::{Event, ReaderStream, TextStream},
    },
    tokio::net::TcpStream,
    Response,
};
use serde::{Deserialize, Serialize};
use std::{io, net::SocketAddr};

#[macro_use]
extern crate rocket;

#[derive(FromForm, Debug)]
struct Data {
    message: Strict<String>,
    id: Option<u32>,
}

#[derive(Serialize, Responder)]
struct JsonResponse {
    data: String,
}

#[derive(Debug, Serialize)]
struct Game {
    id: u32,
    name: String,
}

#[get("/")]
fn index() -> Html<&'static str> {
    Html("<html><body><h1>Hello</h1></body></html>")
}

#[post("/games", data = "<data>")]
fn create_game(data: Form<Data>) -> Json<JsonResponse> {
    dbg!(&data);
    let game = Game {
        id: data.id.unwrap_or(0),
        name: data.message.clone(),
    };
    Json(JsonResponse {
        data: serde_json::to_string(&game).unwrap(),
    })
}

#[get("/stream")]
async fn stream() -> io::Result<ReaderStream![TcpStream]> {
    let address = SocketAddr::from(([127, 0, 0, 1], 9999));
    let stream = TcpStream::connect(address).await?;
    Ok(ReaderStream::one(stream))
}

/// Produce an infinite series of `"hello"`s, one per second.
#[get("/infinite-hellos")]
async fn infinite_hello() -> TextStream![&'static str] {
    TextStream! {
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            yield "hello";
            interval.tick().await;
        }
    }
}

#[get("/infinite_events")]
async fn infinite_events() -> EventStream![] {
    EventStream! {
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            yield Event::data("hello");
            interval.tick().await;
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![index, create_game, stream, infinite_hello, infinite_events],
    )
}
