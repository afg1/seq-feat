//! Functions for the calculation of entropy in sequences
use itertools::Itertools;
use std::collections::HashMap;

/// Get the probability distributionn of kmers of given length
///
/// Iterates over the sequence in chinks of the given length and
/// calculated the probability of observing each kmer. The kmer
/// probabilities are returned as a hash map with the key being
/// the kmer string itself, and the value the probability of
/// observing it.
///
/// This function is used downstream in the entropy calculators
///
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

/// Calculate the Shannon entropy for a sequence, using given kmer length range
///
/// This function will repeatedly calculate the kmer probability distribution
/// and then calculate the shannon entropy from it. You can end up with a
/// number of entropies in the resulting Vec<f64>
///
/// Bear in mind that the kmer_max parameter will have a strong impact on
/// the runtime of this function, probably factorial.
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

/// Calculate the Tsallis entropy for a sequence, using given kmer length range
///
/// This function will repeatedly calculate the kmer probability distribution
/// and then calculate the tsallis entropy from it. You can end up with a
/// number of entropies in the resulting Vec<f64>
///
/// Bear in mind that the kmer_max parameter will have a strong impact on
/// the runtime of this function, probably factorial.
pub fn tsallis(seq: &str, kmer_max: u64) -> Vec<f64> {
    let mut entropy_list: Vec<f64> = Vec::new();
    let q = 2.0;

    for kmer in 1..kmer_max + 1 {
        let kmer_probs = kmer_probabilities(seq, kmer);
        let entropy =
            (1.0 / (q - 1.0)) * (1.0 - kmer_probs.iter().fold(0.0, |acc, x| acc + (x.1.powf(q))));
        entropy_list.push(entropy);
    }

    entropy_list
}
