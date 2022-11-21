use crate::stats::counts::*;
use std::collections::HashMap;

/*
This code is largely reimplemented from CPC2, details here:
https://github.com/gao-lab/CPC2_standalone/blob/master/bin/CPC2.py#L188

 */

lazy_static! {
    pub static ref POS_PROBS: HashMap<char, [f64; 10]> = HashMap::from_iter([
        (
            'A',
            [0.51, 0.55, 0.57, 0.52, 0.48, 0.58, 0.57, 0.54, 0.50, 0.36]
        ),
        (
            'C',
            [0.29, 0.44, 0.55, 0.49, 0.52, 0.60, 0.60, 0.56, 0.51, 0.38]
        ),
        (
            'G',
            [0.62, 0.67, 0.74, 0.65, 0.61, 0.62, 0.52, 0.41, 0.31, 0.17]
        ),
        (
            'U',
            [0.51, 0.60, 0.69, 0.64, 0.62, 0.67, 0.58, 0.48, 0.39, 0.24]
        )
    ]);
    pub static ref POS_WEIGHTS: HashMap<char, f64> =
        HashMap::from_iter([('A', 0.062), ('C', 0.093), ('G', 0.205), ('U', 0.154)]);
    pub static ref POS_PARA: [f64; 10] = [1.9, 1.8, 1.7, 1.6, 1.5, 1.4, 1.3, 1.2, 1.1, 0.0];
    pub static ref CONT_PROBS: HashMap<char, [f64; 10]> = HashMap::from_iter([
        (
            'A',
            [0.40, 0.55, 0.58, 0.58, 0.52, 0.48, 0.45, 0.45, 0.38, 0.19]
        ),
        (
            'C',
            [0.50, 0.63, 0.59, 0.50, 0.46, 0.45, 0.47, 0.56, 0.59, 0.33]
        ),
        (
            'G',
            [0.21, 0.40, 0.47, 0.50, 0.52, 0.56, 0.57, 0.52, 0.44, 0.23]
        ),
        (
            'U',
            [0.30, 0.49, 0.56, 0.53, 0.48, 0.48, 0.52, 0.57, 0.60, 0.51]
        )
    ]);
    pub static ref CONT_WEIGHTS: HashMap<char, f64> =
        HashMap::from_iter([('A', 0.084), ('C', 0.076), ('G', 0.081), ('U', 0.055)]);
    pub static ref CONT_PARA: [f64; 10] =
        [0.33, 0.31, 0.29, 0.27, 0.25, 0.23, 0.21, 0.19, 0.17, 0.0];
}

fn lookup_content_prob(base: char, val: f64) -> f64 {
    if val < 0.0 {
        return 0.0;
    }

    let base_specific_content_prob = CONT_PROBS.get(&base).unwrap();
    let base_specific_content_weight = CONT_WEIGHTS.get(&base).unwrap();

    for el in CONT_PARA.iter().zip(base_specific_content_prob.iter()) {
        let (para, prob) = el;
        if val >= *para {
            return *prob * *base_specific_content_weight;
        }
    }

    0.0
}

fn lookup_position_prob(base: char, val: f64) -> f64 {
    if val < 0.0 {
        return 0.0;
    }

    let base_specific_pos_prob = POS_PROBS.get(&base).unwrap();
    let base_specific_pos_weight = POS_WEIGHTS.get(&base).unwrap();

    for el in POS_PARA.iter().zip(base_specific_pos_prob.iter()) {
        let (para, prob) = el;
        if val >= *para {
            return *prob * *base_specific_pos_weight;
        }
    }

    0.0
}

pub fn score(seq: &str) -> f64 {
    let mut fickett_score: f64 = 0.0;

    let (phase_0_a, phase_0_c, phase_0_g, phase_0_u) = get_phased_counts(seq, 0);
    let (phase_1_a, phase_1_c, phase_1_g, phase_1_u) = get_phased_counts(seq, 1);
    let (phase_2_a, phase_2_c, phase_2_g, phase_2_u) = get_phased_counts(seq, 2);

    let a_content = (phase_0_a + phase_1_a + phase_2_a) / seq.len() as f64;
    let c_content: f64 = (phase_0_c + phase_1_c + phase_2_c) / seq.len() as f64;
    let g_content: f64 = (phase_0_g + phase_1_g + phase_2_g) / seq.len() as f64;
    let u_content: f64 = (phase_0_u + phase_1_u + phase_2_u) / seq.len() as f64;

    let pos_a = [phase_0_a, phase_1_a, phase_2_a]
        .into_iter()
        .reduce(f64::max)
        .unwrap()
        / [phase_0_a, phase_1_a, phase_2_a]
            .into_iter()
            .reduce(f64::min)
            .unwrap();
    let pos_c = [phase_0_c, phase_1_c, phase_2_c]
        .into_iter()
        .reduce(f64::max)
        .unwrap()
        / [phase_0_c, phase_1_c, phase_2_c]
            .into_iter()
            .reduce(f64::min)
            .unwrap();
    let pos_g = [phase_0_g, phase_1_g, phase_2_g]
        .into_iter()
        .reduce(f64::max)
        .unwrap()
        / [phase_0_g, phase_1_g, phase_2_g]
            .into_iter()
            .reduce(f64::min)
            .unwrap();
    let pos_u = [phase_0_u, phase_1_u, phase_2_u]
        .into_iter()
        .reduce(f64::max)
        .unwrap()
        / [phase_0_u, phase_1_u, phase_2_u]
            .into_iter()
            .reduce(f64::min)
            .unwrap();

    println!("{} {} {} {}", pos_a, pos_c, pos_g, pos_u);

    // In the CPC2 code, they sum the phased nt counts, but that would just equal the same as counting in unphased
    // I don't think I need to redo the content counting

    fickett_score += lookup_content_prob('A', a_content);
    fickett_score += lookup_content_prob('C', c_content);
    fickett_score += lookup_content_prob('G', g_content);
    fickett_score += lookup_content_prob('U', u_content);

    fickett_score += lookup_position_prob('A', pos_a);
    fickett_score += lookup_position_prob('C', pos_c);
    fickett_score += lookup_position_prob('G', pos_g);
    fickett_score += lookup_position_prob('U', pos_u);

    fickett_score
}

#[cfg(test)]
mod test {
    use crate::stats::fickett;
    #[test]
    fn test_fickett() {
        let seq = "CCUCCAGGCCCUGCCUUCUGCCUGCACAUUCUGCCCUGAUUUCCGGAACCUGGAAGCCUAGGCAGGCAGUGGGGAACUCUGACUCGCCUGUGCUCUGGAGCUUGAUCCGAAAGCUUCCACAGUGAGGACUGCUCCGUGGGGGUAAGAGAGCACCAGGCACUGAGGCCUGGGAGUUCCACAGACCAACACCCCUGCUCCUGGCGGCUCCCACCCGGGACUUAGACCCUCAGGUCCCUAAUAUCCCGGAGGUGCUCUCAAUCAGAAAGGUCCUGCUCCGCUUCGCAGUGGAAUGGAACGGAUUUAGAAGCCUGCAGUAGGGGAGUGGGGAGUGGAGAGAGGGAGCCCAGAGUUACAGACGGCGGCGAGAGGAAGGAGGGGCGUCUUUAUUUUUUUAAGGCCCCAAAGAGUCUGAUGUUUACAAGACCAGAAAUGCCACGGCCGCGUCCUGGCAGAGAAAAGGCUGAAAUGGAGGACCGGCGCCUUCCUUAUAAGUAUGCACAUUGGCGAGAGAAGUGCUGCAACCUAAACCAGCAAUUACACCCAAGCUCGUUGGGGCCUAAGCCAGUACCGACCUGGUAGAAAAAGCAACCACGAAGCUAGAGAGAGAGCCAGAGGAGGGAAGAGAGCGCCAGACGAAGGUGAAAGCGAACCACGCAGAGAAAUGCAGGCAAGGGAGCAAGGCGGCAGUUCCCGGAACAAACGUGGCAGAGGGCAAGACGGGCACUCACAGACAGAGGUUUAUGUAUUUUUAUUUUUUAAAAUCUGAUUUGGUGUUCCAUGAGGAAAAGGGAAAAUCUAGGGAACGGGAGUACAGAGAGAAUAAUCCGGGUCCUAGCUCGCCACAUGAACGCCCAGAGAACGCUGGAAAAACCUGAGCGGGUGCCGGGGCAGCACCCGGCUCGGGUCAGCCACUGCCCCACACCGGGCCCACCAAGCCCCGCCCCUCGCGGCCACCGGGGCUUCCUUGCUCUUCUUAUCAUCUCCAUCUUUAUGAUGAGGCUUGUUAACAAGACCAGAGAGCUGGCCAAGCACCUCUAUCUCAGCCGCGCCCGCUCAGCCGAGCAGCGGUCGGUGGGGGGACUGGGAGGCGCUAAUUAAUUGAUUCCUUUGGACUGUAAAAUAUGGCGGCGUCUACACGGAACCCAUGGACUCAUAAACAAUAUAUCUGUUGGGCGUGAGUGCACUGUCUCUCAAAUAAUUUUUCCAUAGGCAAAUGUCAGAGGGUUCUGGAUUUUUAGUUGCUAAGGAAAGAUCCAAAUGGGACCAAUUUUAGGAGGCCCAAACAGAGUCCGUUCAGUGUCAGAAAAUGCUUCCCCAAAGGGGUUGGGAGUGUGUUUUGUUGGAAAAAAGCUUGGGUUAUAGGAAAGCCUUUCCCUGCUACUUGUGUAGACCCAGCCCAAUUUAAGAAUUACAAGGAAGCGAAGGGGUUGUGUAGGCCGGAAGCCUCUCUGUCCCGGCUGGAUGCAGGGGACUUGAGCUGCUCCGGAAUUUGAGAGGAACAUAGAAGCAAAGGUCCAGCCUUUGCUUCGUGCUGAUUCCUAGACUUAAGAUUCAAAAACAAAUUUUUAAAAGUGAAACCAGCCCUAGCCUUUGGAAGCUCUUGAAGGUUCAGCACCCACCCAGGAAUCCACCUGCCUGUUACACGCCUCUCCAAGACACAGUGGCACCGCUUUUCUAACUGGCAGCACAGAGCAACUCUAUAAUAUGCUUAUAUUAGGUCUAGAAGAAUGCAUCUUGAGACACAUGGGUAACCUAAUUAUAUAAUGCUUGUUCCAUACAGGAGUGAUUAUGCAGUGGGACCCUGCUGCAAACGGGACUUUGCACUCUAAAUAUAGACCCCAGCUUGGGACAAAAGUUGCAGUAGAAAAAUAGACAUAGGAGAACACUUAAAUAAGUGAUGCAUGUAGACACAGAAGGGGUAUUUAAAAGACAGAAAUAAUAGAAGUACAGAAGAACAGAAAAAAAAUCAGCAGAUGGAGAUUACCAUUCCCAAUGCCUGAACUUCCUCCUGCUAUUAAGAUUGCUAGAGAAUUGUGUCUUAAACAGUUCAUGAACCCAGAAGAAUGCAAUUUCAAUGUAUUUAGUACACACACAGUAUGUAUAUAAACACAACUCACAGAAUAUAUUUUCCAUACAUUGGGUAGGUAUGCACUUUGUGUAUAUAUAAUAAUGUAUUUUCCAUGCAGUUUUAAAAUGUAGAUAUAUUAAUAUCUGGAUGCAUUUUCUGUGCACUGGUUUUAUAUGCCUUAUGGAGUAUAUACUCACAUGUAGCUAAAUAGACUCAGGACUGCACAUUCCUUGUGUAGGUUGUGUGUGUGUGGUGGUUUUAUGCAUAAAUAAAGUUUUACAUGUGGUGAAUAUAAA";
        println!("{}", fickett::score(seq));
    }
}
