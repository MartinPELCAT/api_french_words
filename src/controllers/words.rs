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

#[derive(Deserialize)]
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
}

#[derive(Deserialize, Debug)]
pub struct MotusRequest {
    // Sequences
    s: Option<String>,

    // Length
    l: Option<usize>,

    // contains
    c: Option<String>,

    // Not sequence
    ns: Option<String>,

    // Not contains
    nc: Option<String>,
}

// /motus?q=[0:a],[5:er] = a....er%
#[get("/motus")]
pub async fn search_advanced(
    query: web::Query<MotusRequest>,
    ctx: Data<AppContext>,
) -> Result<impl Responder> {
    let seq_query = (&query.s).to_owned().unwrap_or("".to_string());
    let contains_query = (&query.c).to_owned().unwrap_or("".to_string());
    let length_query = (&query.l.unwrap_or(0)).to_owned();

    let not_seq_query = (&query.ns).to_owned().unwrap_or("".to_string());
    let not_contains_query = (&query.nc).to_owned().unwrap_or("".to_string());
    let words = &ctx.all_words;

    let mut sequences: Vec<(usize, String)> = vec![];

    if !seq_query.is_empty() {
        sequences = seq_query
            .split(",")
            .map(|seq| {
                let seq = seq.replace(&['[', ']'], "");
                let val = seq.split(":").map(|v| v.to_string()).collect::<Vec<_>>();
                let index = val.get(0).unwrap().parse::<usize>().unwrap();
                let value = val.get(1).unwrap();
                let tuple: (usize, String) = (index, value.clone());
                tuple
            })
            .collect::<Vec<_>>();
    }

    let mut not_sequences: Vec<(usize, String)> = vec![];
    if !not_seq_query.is_empty() {
        not_sequences = not_seq_query
            .split(",")
            .map(|seq| {
                let seq = seq.replace(&['[', ']'], "");
                let val = seq.split(":").map(|v| v.to_string()).collect::<Vec<_>>();
                let index = val.get(0).unwrap().parse::<usize>().unwrap();
                let value = val.get(1).unwrap();
                let tuple: (usize, String) = (index, value.clone());
                tuple
            })
            .collect::<Vec<_>>();
    }

    let not_contains_letters = not_contains_query.split(",").collect::<Vec<_>>();

    let contains_letters = contains_query.split(",").collect::<Vec<_>>();

    let seqs = sequences.clone();

    let found_words = words
        .iter()
        .filter(move |word| {
            let letters = &contains_letters;
            let not_contains_letters = &not_contains_letters;
            let seqs = &seqs[..];
            let not_sequences = &not_sequences[..];
            if length_query != 0 && word.len() != length_query {
                return false;
            }

            let word_chars = word.chars().collect::<Vec<_>>();

            for letter in letters {
                if !word.contains(letter) {
                    return false;
                }
            }

            for letter in not_contains_letters {
                if !letter.is_empty() {
                    if word.contains(letter) {
                        return false;
                    }
                }
            }

            for seq in seqs {
                let value = seq.1.clone();

                if !word.contains(&value) {
                    return false;
                }

                let first = seq.0.clone();
                let last = first + value.len();

                if word_chars.len() < last {
                    return false;
                }

                let extract = word_chars
                    .get(first..last)
                    .unwrap()
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join("");

                if !extract.eq(&value) {
                    return false;
                }
            }

            for seq in not_sequences {
                let value = seq.1.clone();

                let first = seq.0.clone();
                let last = first + value.len();

                if word_chars.len() < last {
                    return false;
                }

                let extract = word_chars
                    .get(first..last)
                    .unwrap()
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join("");

                if extract.eq(&value) {
                    return false;
                }
            }

            true
        })
        .map(|w| w.to_owned())
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(AllWordsResponse { words: found_words }))
}
