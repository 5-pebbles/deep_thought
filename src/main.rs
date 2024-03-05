use rocket::serde::{Deserialize, Serialize};
use rocket::{
    get,
    http::{ContentType, Status},
    launch, routes,
    serde::json::Json,
};
use std::{fs::File, io::Write, path::Path};
use utoipa::{OpenApi, ToSchema};

// Api
#[derive(Deserialize, Serialize, ToSchema)]
#[serde(crate = "rocket::serde")]
struct Answer {
    pub question: String,
    pub answer: u8,
}

#[utoipa::path(
    get,
    path = "/answer",
    responses(
        (status = 200, description = "Success", body = Answer),
    ),
    params(
        ("question" = String, Query, description = "The question to answer"),
    )
)]
#[get("/answer?<question>")]
fn get_answer(question: Option<String>) -> Json<Answer> {
    Json(Answer {
        question: question.unwrap_or("Life, the Universe, and Everything".to_string()),
        answer: 42,
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(generate_docs())
        .mount("/", routes![get_answer, docs_yaml])
}

// Docs
#[derive(OpenApi)]
#[openapi(paths(get_answer, docs_yaml), components(schemas(Answer)))]
struct ApiDoc;

fn generate_docs() -> Result<(), String> {
    let openapi = ApiDoc::openapi();
    let yaml =
        serde_yaml::to_string(&openapi).map_err(|e| format!("Failed to serialize docs: {}", e))?;

    let path = Path::new("docs/openapi.yaml");
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }
    let mut file = File::create(path).map_err(|e| format!("Failed to create file: {}", e))?;
    file.write_all(yaml.as_bytes())
        .map_err(|e| format!("Failed to write to file: {}", e))?;

    Ok(())
}

#[utoipa::path(
    get,
    path = "/docs.yaml",
    responses((
        status = 200,
        description = "Success",
        content_type = "application/x-yaml",
        body = String,
    ))
)]
#[get("/docs.yaml")]
fn docs_yaml() -> Result<(ContentType, String), (Status, String)> {
    let yaml = std::fs::read_to_string("docs/openapi.yaml").map_err(|e| {
        (
            Status::InternalServerError,
            format!("Failed to read docs: {}", e),
        )
    })?;
    Ok((ContentType::new("application", "x-yaml"), yaml))
}
