use crate::utils::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name=gc_content)]
pub fn gc_content(seq: &str) -> f64 {
    let clean_seq = seq::clean(seq);
    let num_c = c_content(&clean_seq);
    let num_g = g_content(&clean_seq);

    (num_c + num_g) / clean_seq.len() as f64
}

pub fn a_content(seq: &str) -> f64 {
    let clean_seq: String = seq::clean(seq);
    clean_seq.matches('A').count() as f64
}

pub fn c_content(seq: &str) -> f64 {
    let clean_seq: String = seq::clean(seq);
    clean_seq.matches('C').count() as f64
}

pub fn g_content(seq: &str) -> f64 {
    let clean_seq: String = seq::clean(seq);
    clean_seq.matches('G').count() as f64
}

pub fn u_content(seq: &str) -> f64 {
    let clean_seq: String = seq::clean(seq);
    clean_seq.matches('U').count() as f64
}

pub fn get_phased_counts(seq: &str, phase: usize) -> (f64, f64, f64, f64) {
    let phase_a: f64 = seq
        .chars()
        .skip(phase)
        .step_by(3)
        .map(|x| (x == 'A') as i64 as f64)
        .sum();
    let phase_c: f64 = seq
        .chars()
        .skip(phase)
        .step_by(3)
        .map(|x| (x == 'C') as i64 as f64)
        .sum();
    let phase_g: f64 = seq
        .chars()
        .skip(phase)
        .step_by(3)
        .map(|x| (x == 'G') as i64 as f64)
        .sum();
    let phase_u: f64 = seq
        .chars()
        .skip(phase)
        .step_by(3)
        .map(|x| (x == 'U') as i64 as f64)
        .sum();

    (phase_a, phase_c, phase_g, phase_u)
}
