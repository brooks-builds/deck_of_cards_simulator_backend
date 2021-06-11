use rocket::{
    form::{Form, Lenient, Strict},
    http::{ContentType, Status},
    response::content::Json,
    Response,
};
use serde::{Deserialize, Serialize};

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
fn index() -> &'static str {
    "Hello world"
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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, create_game])
}
