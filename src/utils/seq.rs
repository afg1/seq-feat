use regex::{Match, Regex};

lazy_static! {
    pub static ref SEQ_CHECK: Regex = Regex::new(r"[[:alpha:]&&[^ACTGU]]").unwrap();
}

/// Checks a sequence contains only uppercase AUCG characters
///
/// This is a simple check using regex to see if the sequence looks like
/// a valid RNA or not.
///
/// Note, this does not consider ambiguous bases yet (i.e N and co) so any
/// sequence with those in will fail.
///
/// # Examples
/// ```
/// # fn main() {
/// use utils::seq::*;
/// let good_seq = "AAAGGGUUUCCC"
/// let bad_seq  = "NNNNNNNNNNNN"
///
/// let good = is_ok(&good_seq); // true
/// let bad = is_ok(&bad_seq); //false
///
/// # }
/// ```
pub fn is_ok(seq: &str) -> bool {
    // Check a sequence is valid (i.e. has no disallowed characters in)
    let invalid_char_matches: Option<Match> = SEQ_CHECK.find(seq);
    match invalid_char_matches {
        Some(_x) => false,
        None => true,
    }
}

/// Cleans up a sequence passed by the user, ready for feature extraction
///
/// seq could come from anywhere, and eventually will be coming from e.g. a
/// html form and could therefore contain anything! This function will clean
/// the sequence by converting to uppercase, and converting DNA to RNA (i.e
/// changing T to U)
///
/// # Panics
/// The function will panic if the sequence does not satisfy the `is_ok`
/// function after cleaning. This is likely to be because of invalid
/// characters.
///
/// # Examples
/// ```
/// use utils::clean;
/// let seq = "aaagggtttccc";
/// let clean_seq = clean(&seq);
/// // clean_seq should be AAAGGGTTTCCC
/// ```
///
/// ```
/// use utils::clean;
/// let seq = "AGCTNNNTAG"
/// let clean_seq = clean(&seq);
/// // This will panic! We don't handle ambiguous bases yet
/// ```
pub fn clean(seq: &str) -> String {
    // Clean up the sequence to be only RNA and uppercase
    let uc_seq: String = seq.to_uppercase().replace('T', "U");
    if !is_ok(&uc_seq) {
        panic!("Invalid sequence passed to cleaner");
    }
    uc_seq
}
