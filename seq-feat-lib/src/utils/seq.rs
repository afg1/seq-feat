use anyhow::{anyhow, Result};
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
/// use seq_feat::utils::seq::is_ok;
/// let good_seq = "AAAGGGUUUCCC";
/// let bad_seq  = "NNNNNNNNNNNN";
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
/// use seq_feat::utils::seq::clean;
/// let seq = "aaagggtttccc";
/// let clean_seq = clean(&seq);
/// // clean_seq should be AAAGGGTTTCCC
/// ```
///
/// ```
/// use seq_feat::utils::seq::clean;
/// let seq = "AGCTNNNTAG";
/// let clean_seq = clean(&seq);
/// // This will give an error! We don't handle ambiguous bases yet
/// ```
pub fn clean(seq: &str) -> Result<String> {
    // Clean up the sequence to be only RNA and uppercase
    let uc_seq: String = seq.to_uppercase().replace('T', "U");
    if !is_ok(&uc_seq) {
        Err(anyhow!("Invalid characters found in sequence"))
    } else {
        Ok(uc_seq)
    }
}

#[cfg(test)]
mod test {
    use crate::utils::seq;
    #[test]
    fn test_seq_ok() {
        assert_eq!(seq::is_ok("ACTGUAAA"), true);
        assert_eq!(seq::is_ok("WTFLOL"), false);
    }

    #[test]
    fn test_seq_clean() {
        assert_eq!(seq::clean("actccc").unwrap(), "ACUCCC");
    }

    /// Make sure the cleaner gives an error when finding invalid sequences
    #[test]
    fn test_seq_clean_invalid() {
        assert_eq!(
            seq::clean("actnnccc").err().unwrap().to_string(),
            "Invalid characters found in sequence"
        );
    }
}
