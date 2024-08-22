#![allow(dead_code, unused_imports)]

use actix_web::{
    get, http::header::ContentType, post, web, web::Path, App, HttpResponse, HttpServer, Responder
};

use std::collections::HashMap;
use validator::Validate;
use actix_cors::Cors;

mod database;
mod models;

// I will work on validations later

#[get("/")]
async fn display_all_tasks() -> impl Responder {
    let all_tasks: Vec<models::Task> = database::fetch_tasks().unwrap();
    let serialized_tasks = serde_json::to_string_pretty(&all_tasks);
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serialized_tasks.unwrap())
}

#[get("/task/{task_id}")]
async fn get_single_task(task_id: Path<u32>) -> impl Responder {
    let task_id = task_id.into_inner();
    let selected_task = database::get_single_task(task_id).unwrap();

    let task: String = match selected_task {
        Some(task) => serde_json::to_string_pretty(&task).unwrap(),
        None => "".to_string(),
    };

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(task)
}

#[post("/")]
async fn create_task(task: web::Json<models::Task>) -> impl Responder {
    let _ = database::create_task(&task);

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string_pretty(&task).unwrap())
}

#[get("/toggle_complete/{task_id}")]
async fn toggle_task_complete(task_id: Path<u32>) -> impl Responder {
    let task_id: u32 = task_id.into_inner();
    let set_task = database::get_single_task(task_id).unwrap();

    match &set_task {
        Some(set_task) => {
            let _ = database::set_task_is_complete(task_id, !set_task.is_complete);
        }, 
        None => {}
    };

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string_pretty(&set_task.unwrap()).unwrap())
}

#[get("undo_task/<id>")]
async fn undo_task(task_id: Path<u32>) -> impl Responder {
    let task_id: u32 = task_id.into_inner();
    let _ = database::set_task_is_complete(task_id, false);
    let set_task = database::get_single_task(task_id).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string_pretty(&set_task.unwrap()).unwrap())
}

#[get("/delete/{task_id}")]
async fn delete_task(task_id: Path<u32>) -> impl Responder {
    let task_id: u32 = task_id.into_inner();

    // Checking whether the task to delete exists
    let selected_task: Option<models::Task> = database::get_single_task(task_id).unwrap();

    let status: String = match selected_task {
        Some(_) => "true".to_string(),
        None => "false".to_string(),
    };

    let _ = database::delete_task(task_id);
    let mut response: HashMap<String, String> = HashMap::new();
    response.insert("status".to_string(), status);
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string_pretty(&response).unwrap())
}

#[post("/update/{task_id}")]
async fn update_task(task_id: Path<u32>, task: web::Json<models::Task>) -> impl Responder {
    let task_id: u32 = task_id.into_inner();
    let status = database::update_task(task_id, &task).unwrap();
    let mut status_hash_map: HashMap<String, bool> = HashMap::new();
    status_hash_map.insert("status".to_string(), status);

    HttpResponse::Ok().body(serde_json::to_string(&status_hash_map).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server started on 127.0.0.1:80");
    HttpServer::new(|| {
        // Setting up cors 
        let cors = Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header()
        .max_age(3600);

        App::new()
            .wrap(cors)
            .service(display_all_tasks)
            .service(get_single_task)
            .service(create_task)
            .service(delete_task)
            .service(update_task)
            .service(toggle_task_complete)
            .service(undo_task)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
