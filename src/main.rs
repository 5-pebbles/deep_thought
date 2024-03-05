use rocket::serde::{Deserialize, Serialize};
use rocket::{get, launch, routes, serde::json::Json};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Answer {
    pub question: String,
    pub answer: u8,
}

#[get("/answer?<question>")]
fn get_answer(question: Option<String>) -> Json<Answer> {
    Json(Answer {
        question: question.unwrap_or("Life, the Universe, and Everything".to_string()),
        answer: 42,
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_answer])
}
