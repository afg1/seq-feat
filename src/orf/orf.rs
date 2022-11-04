use crate::utils::*;
use regex::{Match, Regex};
use wasm_bindgen::prelude::*;

lazy_static! {
    pub static ref AUG_START: Regex = Regex::new(r"AUG").unwrap();
    pub static ref TAA_STOP: Regex = Regex::new(r"UAA").unwrap();
    pub static ref TAG_STOP: Regex = Regex::new(r"UAG").unwrap();
    pub static ref TGA_STOP: Regex = Regex::new(r"UGA").unwrap();
}

#[wasm_bindgen(js_name=orf_length)]
pub fn length(seq: &str) -> f64 {
    let clean_seq = seq::clean(seq);
    let stops: Vec<&Regex> = vec![&TAA_STOP, &TAG_STOP, &TGA_STOP];
    let start_matches = AUG_START.find(&clean_seq);
    let stop_matches = stops
        .into_iter()
        .map(|x| x.find(&clean_seq))
        .collect::<Vec<Option<Match>>>();

    let start_idx = match start_matches {
        Some(x) => x.start() as i64,
        None => -1,
    };

    let mut end_idx: i64 = -1;
    for sm in stop_matches.iter() {
        let end_idx_i: i64 = match sm {
            Some(x) => x.end() as i64,
            None => -1,
        };
        if end_idx_i > end_idx {
            end_idx = end_idx_i;
        }
    }

    // No start codon = no orf
    if start_idx < 0 {
        return -1.0;
    }

    // Stop before start, or no stop = no orf
    if end_idx < start_idx {
        return -1.0;
    }

    let length = end_idx - start_idx;
    println!("{} {}", length, length % 3);
    println!("{}  {}", start_idx, end_idx);

    length as f64
}
