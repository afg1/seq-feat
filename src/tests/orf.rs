#[cfg(test)]
use crate::coding::*;

#[test]
fn test_no_orf() {
    // Test that the orf finder doesn't find an orf in a simple sequence
    assert_eq!(orf::length("AAAAAAAA"), -1.0);
    assert_eq!(orf::length("CCCCCCCCC"), -1.0);
}

#[test]
fn test_orf_exists() {
    // Test some sequences from RNAcentral that had potential ORFs annotated by cpat
    // The lengths are what this code determined, I need to run separate checks to be sure they're right
    assert_eq!(
        orf::length(
            "GGCAUGGAGUCCUGUGGUAUCCACGAGAUCACCUUCAACUCCAUCAUGAAGUGUGAUGUGGAUAUCCGCAAAGACCUGUAUGCC"
        ),
        46.0
    );
    assert_eq!(orf::length("AUGAGUGAUCAGCAGUUGGACUAUGCCUUAGACCUAAUGAGGCACCUACCUCCACAGCAAAUUGAGAAAAAGCUCAGCAACCUGAUUGACCUGAUCCCUCAUCUAUGUGAAGAUCUCUUGCCUUCUGUUAAUCAGAUAAUGAAAAUUGCCAGAGACAAGGAAGUGGGAAAGGAUUACCUUUUGUGUGACUGCAACAGAGAU"), 37.0);
}
