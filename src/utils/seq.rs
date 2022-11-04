use regex::{Match, Regex};
lazy_static! {
    pub static ref SEQ_CHECK: Regex = Regex::new(r"[[:alpha:]&&[^ACTGU]]").unwrap();
}

pub fn is_ok(seq: &str) -> bool {
    // Check a sequence is valid (i.e. has no disallowed characters in)
    let invalid_char_matches: Option<Match> = SEQ_CHECK.find(seq);
    match invalid_char_matches {
        Some(_x) => false,
        None => true,
    }
}

pub fn clean(seq: &str) -> String {
    // Clean up the sequence to be only RNA and uppercase
    let uc_seq: String = seq.to_uppercase().replace('T', "U");
    if !is_ok(&uc_seq) {
        panic!("Invalid sequence passed to cleaner");
    }
    uc_seq
}
