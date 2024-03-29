use wasm_bindgen::prelude::*;

/// Get the fraction of the sequence that is either a G or C
///
/// GC content is a fairly well established feature.
/// This function simply counts the ocurrences of each
/// character and divides by the total sequence length.
#[wasm_bindgen(js_name=gc_content)]
pub fn gc_content(clean_seq: &str) -> f64 {
    let num_c = c_content(clean_seq);
    let num_g = g_content(clean_seq);

    (num_c + num_g) / clean_seq.len() as f64
}

/// Calculate the number of A nucleotides in a sequence
pub fn a_content(clean_seq: &str) -> f64 {
    clean_seq.matches('A').count() as f64
}

/// Calculate the number of C nucleotides in a sequence
pub fn c_content(clean_seq: &str) -> f64 {
    clean_seq.matches('C').count() as f64
}

/// Calculate the number of G nucleotides in a sequence
pub fn g_content(clean_seq: &str) -> f64 {
    clean_seq.matches('G').count() as f64
}

/// Calculate the number of U nucleotides in a sequence
pub fn u_content(clean_seq: &str) -> f64 {
    clean_seq.matches('U').count() as f64
}

// Get nucleotide counts at given phase
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

#[cfg(test)]
mod test {
    use crate::stats::counts;
    #[test]
    fn test_gc_content() {
        assert_eq!(counts::gc_content("GGGGGGCCCCCC"), 1.0);
        assert_eq!(counts::gc_content("GGGGGGAAAAAA"), 0.5);
    }
}
