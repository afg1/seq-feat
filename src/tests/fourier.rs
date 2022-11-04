use crate::stats::fourier::fft_sequence;
#[cfg(test)]
use crate::stats::*;
#[test]
fn test_all_as_binary() {
    let seq = "AAAAAAAAAA";

    let (b_a, b_c, b_g, b_u) = fourier::seq_2_binary(&seq);

    assert_eq!(b_a.iter().sum::<i64>(), 10);
    assert_eq!(b_c.iter().sum::<i64>(), 0);
    assert_eq!(b_g.iter().sum::<i64>(), 0);
    assert_eq!(b_u.iter().sum::<i64>(), 0);
}

#[test]
fn test_5as_5us_binary() {
    let seq = "AUAUAUAUAU";

    let (b_a, b_c, b_g, b_u) = fourier::seq_2_binary(&seq);

    assert_eq!(b_a.iter().sum::<i64>(), 5);
    assert_eq!(b_c.iter().sum::<i64>(), 0);
    assert_eq!(b_g.iter().sum::<i64>(), 0);
    assert_eq!(b_u.iter().sum::<i64>(), 5);
}

#[test]
fn test_all_as_integer() {
    let seq = "AAAAAAAAAA";

    let res = fourier::seq_2_integer(&seq);

    assert_eq!(res.iter().sum::<i64>(), 10);
}

#[test]
fn test_5as_5us_integer() {
    let seq = "AUAUAUAUAU";

    let res = fourier::seq_2_integer(&seq);

    assert_eq!(res.iter().sum::<i64>(), 25);
}

#[test]
fn test_all_as_float() {
    let seq = "AAAAAAAAAA";

    let res = fourier::seq_2_real(&seq);

    assert_eq!(res.iter().sum::<f64>(), -15.0);
}

#[test]
fn test_5as_5us_float() {
    let seq = "AUAUAUAUAU";

    let res = fourier::seq_2_real(&seq);

    assert_eq!(res.iter().sum::<f64>(), 0.0);
}

#[test]
fn test_simple_fft_binary() {
    let seq = "AGAGAAGAGAAGAGAAGAGAAGAGAAGAGAAGAGAAGAGAAGAGAAGAGAAGAGAAGAGA";
    let (b_a, _b_c, _b_g, _b_u) = fourier::seq_2_binary(&seq);

    let mut spectrum = fft_sequence(&b_a).unwrap();
    let pow_spectrum: Vec<i64> = spectrum.iter_mut().map(|x| x.powu(2).re.abs()).collect();

    println!("{:?}", pow_spectrum);
}

#[test]
fn test_simple_fft_integer() {
    let seq = "AAAAAAGGGGGGGGAAAAAAGGGGGGGGGAAAAAAAGGGGGGGGGGGAAAAAAAAAGGGG";
    let b_a = fourier::seq_2_integer(&seq);

    let mut spectrum = fft_sequence(&b_a).unwrap();
    let pow_spectrum: Vec<i64> = spectrum.iter_mut().map(|x| x.powu(2).re.abs()).collect();

    println!("{:?}", pow_spectrum);
}

#[test]
fn test_simple_fft_float() {
    let seq = "AGAGAAGAGAAGAGAAGAGAAGAGAAGAGAAGAGAAGAGAAGAGAAGAGAAGAGAAGAGA";
    let b_a = fourier::seq_2_real(&seq);

    let mut spectrum = fft_sequence(&b_a).unwrap();
    let pow_spectrum: Vec<f64> = spectrum.iter_mut().map(|x| x.powu(2).re.abs()).collect();

    println!("{:?}", pow_spectrum);
}
