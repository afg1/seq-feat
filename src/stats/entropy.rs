use itertools::Itertools;
use std::collections::HashMap;

pub fn kmer_probabilities(seq: &str, kmer: u64) -> HashMap<String, f64> {
    let total_windows = (seq.len() as u64 - kmer + 1) as f64;
    let mut kmer_probs: HashMap<String, f64> = HashMap::new();

    for subseq in &seq.chars().chunks(kmer as usize) {
        let kmer_string = subseq.collect::<String>();
        if let Some(c) = kmer_probs.get_mut(&kmer_string) {
            *c += 1.0;
        } else {
            kmer_probs.insert(kmer_string, 1.0);
        }
    }
    for entry in &mut kmer_probs {
        *entry.1 /= total_windows;
    }

    kmer_probs
}

pub fn shannon(seq: &str, kmer_max: u64) -> Vec<f64> {
    let mut entropy_list: Vec<f64> = Vec::new();

    for kmer in 1..kmer_max + 1 {
        let kmer_probs = kmer_probabilities(seq, kmer);
        let entropy = -kmer_probs
            .iter()
            .fold(0.0, |acc, x| acc + (x.1 * x.1.log2()));
        entropy_list.push(entropy);
    }

    entropy_list
}

pub fn tsallis(seq: &str, kmer_max: u64) -> Vec<f64> {
    let mut entropy_list: Vec<f64> = Vec::new();
    let q = 2.0;

    for kmer in 1..kmer_max + 1 {
        let kmer_probs = kmer_probabilities(seq, kmer);
        let entropy =
            (1.0 / (1.0 - q)) * (1.0 - kmer_probs.iter().fold(0.0, |acc, x| acc + (x.1.powf(q))));
        entropy_list.push(entropy);
    }

    entropy_list
}
