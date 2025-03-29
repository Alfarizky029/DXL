use actix_web::{web, App, HttpServer};
use actix_cors::Cors;

mod routes;
mod storage;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on http://0.0.0.0:5000");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .service(
                web::scope("/api")
                    .route("/resources", web::get().to(routes::get_resources))
                    .route("/learning-paths", web::get().to(routes::get_learning_paths))
                    .route("/assistant/query", web::post().to(routes::assistant_query))
            )
    })
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}

```

```rust
// routes.rs
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

// Models (These remain the same as in the original main.rs)
#[derive(Debug, Serialize, Deserialize)]
struct Resource {
    id: i32,
    title: String,
    r#type: String,
    description: String,
    modules: i32,
    duration: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LearningPath {
    id: i32,
    title: String,
    description: String,
    steps: Vec<PathStep>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PathStep {
    title: String,
    duration: String,
}

#[derive(Debug, Deserialize)]
struct AssistantQuery {
    query: String,
}


#[get("/resources")]
async fn get_resources() -> impl Responder {
    let resources = vec![
        Resource {
            id: 1,
            title: String::from("Web3 Fundamentals"),
            r#type: String::from("course"),
            description: String::from("Learn the core concepts of Web3"),
            modules: 8,
            duration: String::from("4 hours"),
        },
        Resource {
            id: 2,
            title: String::from("Blockchain Security"),
            r#type: String::from("course"),
            description: String::from("Explore security in blockchain"),
            modules: 10,
            duration: String::from("6 hours"),
        }
    ];

    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "resources": resources
    }))
}

#[get("/learning-paths")]
async fn get_learning_paths() -> impl Responder {
    let paths = vec![
        LearningPath {
            id: 1,
            title: String::from("Developer Path"),
            description: String::from("Become a Web3 developer"),
            steps: vec![
                PathStep {
                    title: String::from("Web3 Fundamentals"),
                    duration: String::from("4 hours"),
                },
                PathStep {
                    title: String::from("Smart Contract Development"),
                    duration: String::from("8 hours"),
                }
            ],
        }
    ];

    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "paths": paths
    }))
}

#[post("/assistant/query")]
async fn assistant_query(query: web::Json<AssistantQuery>) -> impl Responder {
    let response = match query.query.to_lowercase().as_str() {
        q if q.contains("blockchain") => "Blockchain is a distributed ledger technology",
        q if q.contains("web3") => "Web3 refers to the next generation of the internet",
        q if q.contains("smart contracts") => "Smart contracts are self-executing contracts with terms written in code",
        q if q.contains("nft") => "NFTs are unique digital assets representing ownership on the blockchain",
        q if q.contains("defi") => "DeFi refers to decentralized financial services on blockchain networks",
        _ => "I'm your Web3 learning assistant!"
    };

    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "response": response
    }))
}
```

```rust
// storage.rs
// This module can be expanded later to implement persistent storage