// #[cfg(test)]
use crate::utils::*;

#[test]
fn test_seq_ok() {
    assert_eq!(seq::is_ok("ACTGUAAA"), true);
    assert_eq!(seq::is_ok("WTFLOL"), false);
}

#[test]
fn test_seq_clean() {
    assert_eq!(seq::clean("actccc"), "ACUCCC");
}
