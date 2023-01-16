mod controllers;
mod services;

use std::time::Instant;

use actix_web::{web::Data, App, HttpServer};

#[derive(Debug, Clone)]
pub struct AppContext {
    all_words: Vec<String>,
    word_count: usize,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let start = Instant::now();
    let words = services::word_service::process_dictionary("dico_compact.csv").await;

    // services::word_service::reduce_dictonary("dictionary.csv").await;

    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);

    let context = AppContext {
        all_words: words.to_owned(),
        word_count: words.len(),
    };

    println!("Server started at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(context.clone()))
            .service(controllers::words::get_all)
            .service(controllers::words::get_random)
            .service(controllers::words::search_word)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
