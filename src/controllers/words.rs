use actix_web::{
    get,
    web::{self, Data},
    HttpResponse, Responder, Result,
};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::AppContext;

#[derive(Serialize)]
struct AllWordsResponse {
    words: Vec<String>,
}

#[get("/")]
pub async fn get_all(ctx: Data<AppContext>) -> impl Responder {
    let words = &ctx.all_words;
    HttpResponse::Ok().json(AllWordsResponse {
        words: words.clone(),
    })
}

#[derive(Serialize)]
struct RandomWordResponse {
    word: String,
}

#[get("/random")]
pub async fn get_random(ctx: Data<AppContext>) -> Result<impl Responder> {
    let words = &ctx.all_words;
    let word_count = ctx.word_count;
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0..word_count - 1);

    let word = words.get(random_number).unwrap();

    // let response = ;

    Ok(HttpResponse::Ok().json(RandomWordResponse { word: word.clone() }))
}

#[derive(Deserialize, Serialize)]
pub struct SearchRequest {
    q: String,
}

#[get("/search")]
pub async fn search_word(
    url: web::Query<SearchRequest>,
    ctx: Data<AppContext>,
) -> Result<impl Responder> {
    let params = &url.q;
    let all_words = &ctx.all_words;
    let all_words = all_words.clone();

    let found_words = all_words
        .iter()
        .filter(|w| w.contains(params))
        .map(|w| w.to_owned())
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(AllWordsResponse { words: found_words }))
    // Ok(HttpResponse::Ok().json(found_words))
}
