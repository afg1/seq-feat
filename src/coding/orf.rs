use crate::utils::*;
use regex::Regex;
use wasm_bindgen::prelude::*;

lazy_static! {
    pub static ref AUG_START: Regex = Regex::new(r"AUG").unwrap();
    pub static ref TAA_STOP: Regex = Regex::new(r"UAA").unwrap();
    pub static ref TAG_STOP: Regex = Regex::new(r"UAG").unwrap();
    pub static ref TGA_STOP: Regex = Regex::new(r"UGA").unwrap();
}

#[wasm_bindgen(js_name=orf_length)]
pub fn length(seq: &str) -> Option<usize> {
    let clean_seq = seq::clean(seq);
    let stops: Vec<&Regex> = vec![&TAA_STOP, &TAG_STOP, &TGA_STOP];
    let start_matches = AUG_START.find(&clean_seq);

    let start_idx: Option<usize> = start_matches.map(|x| x.start());

    let end_idx: Option<usize> = stops
        .into_iter()
        .map(|x| x.find(&clean_seq))
        .filter_map(|x| x.map(|y| y.end()))
        .collect::<Vec<usize>>()
        .into_iter()
        .max();
    println!("{:?} to {:?}", start_idx, end_idx);

    // No start codon, or no stop = no orf
    if start_idx.is_none() || end_idx.is_none() {
        return None;
    }

    // Stop before start = no orf
    if end_idx < start_idx {
        return None;
    }

    Some(end_idx.unwrap() - start_idx.unwrap())
}

#[cfg(test)]
mod test {
    use crate::coding::orf;
    #[test]
    fn test_no_orf() {
        // Test that the orf finder doesn't find an orf in a simple sequence
        assert_eq!(orf::length("AAAAAAAA"), None);
        assert_eq!(orf::length("CCCCCCCCC"), None);
    }

    #[test]
    fn test_orf_exists() {
        // Test some sequences from RNAcentral that had potential ORFs annotated by cpat
        // The lengths are what this code determined, I need to run separate checks to be sure they're right
        assert_eq!(
            orf::length(
                "GGCAUGGAGUCCUGUGGUAUCCACGAGAUCACCUUCAACUCCAUCAUGAAGUGUGAUGUGGAUAUCCGCAAAGACCUGUAUGCC"
            ),
            Some(46)
        );
        assert_eq!(orf::length("AUGAGUGAUCAGCAGUUGGACUAUGCCUUAGACCUAAUGAGGCACCUACCUCCACAGCAAAUUGAGAAAAAGCUCAGCAACCUGAUUGACCUGAUCCCUCAUCUAUGUGAAGAUCUCUUGCCUUCUGUUAAUCAGAUAAUGAAAAUUGCCAGAGACAAGGAAGUGGGAAAGGAUUACCUUUUGUGUGACUGCAACAGAGAU")
        , Some(37));
    }
}
