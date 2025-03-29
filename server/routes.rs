
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Resource {
    id: i32,
    title: String,
    r#type: String,
    description: String,
    modules: i32,
    duration: String,
}

#[derive(Serialize)]
struct LearningPath {
    id: i32,
    title: String,
    description: String,
    steps: Vec<PathStep>,
}

#[derive(Serialize)]
struct PathStep {
    title: String,
    duration: String,
}

#[derive(Deserialize)]
pub struct AssistantQuery {
    query: String,
}

pub async fn get_resources() -> impl Responder {
    let resources = vec![
        Resource {
            id: 1,
            title: String::from("Web3 Fundamentals"),
            r#type: String::from("course"),
            description: String::from("Learn the core concepts of Web3 and understand how it differs from the traditional internet."),
            modules: 8,
            duration: String::from("4 hours"),
        },
        Resource {
            id: 2,
            title: String::from("Blockchain Security"),
            r#type: String::from("course"),
            description: String::from("Explore security considerations for blockchain projects and learn how to protect your assets."),
            modules: 10,
            duration: String::from("6 hours"),
        },
    ];

    HttpResponse::Ok().json(resources)
}

pub async fn get_learning_paths() -> impl Responder {
    let paths = vec![
        LearningPath {
            id: 1,
            title: String::from("Developer Path"),
            description: String::from("Become a Web3 developer with this structured learning path."),
            steps: vec![
                PathStep {
                    title: String::from("Web3 Fundamentals"),
                    duration: String::from("4 hours"),
                },
                PathStep {
                    title: String::from("Smart Contract Development"),
                    duration: String::from("8 hours"),
                },
            ],
        },
    ];

    HttpResponse::Ok().json(paths)
}

pub async fn assistant_query(query: web::Json<AssistantQuery>) -> impl Responder {
    let response = match query.query.to_lowercase().as_str() {
        q if q.contains("blockchain") => "Blockchain is a distributed ledger technology",
        q if q.contains("web3") => "Web3 refers to the next generation of the internet",
        _ => "I'm your Web3 learning assistant!",
    };

    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "response": response
    }))
}
