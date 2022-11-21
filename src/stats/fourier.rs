use realfft::{FftError, RealFftPlanner};
use rustfft::num_complex::Complex;

pub fn seq_2_binary(seq: &str) -> (Vec<i64>, Vec<i64>, Vec<i64>, Vec<i64>) {
    let len = seq.len();
    let mut binary_a: Vec<i64> = Vec::new();
    let mut binary_c: Vec<i64> = Vec::new();
    let mut binary_g: Vec<i64> = Vec::new();
    let mut binary_u: Vec<i64> = Vec::new();

    binary_a.resize(len, 0);
    binary_c.resize(len, 0);
    binary_g.resize(len, 0);
    binary_u.resize(len, 0);

    seq.match_indices('A')
        .into_iter()
        .for_each(|(idx, _)| binary_a[idx] = 1);
    seq.match_indices('C')
        .into_iter()
        .for_each(|(idx, _)| binary_c[idx] = 1);
    seq.match_indices('G')
        .into_iter()
        .for_each(|(idx, _)| binary_g[idx] = 1);
    seq.match_indices('U')
        .into_iter()
        .for_each(|(idx, _)| binary_u[idx] = 1);

    (binary_a, binary_c, binary_g, binary_u)
}

pub fn seq_2_integer(seq: &str) -> Vec<i64> {
    let len = seq.len();
    let mut integer_seq: Vec<i64> = Vec::new();
    integer_seq.resize(len, 0);

    seq.match_indices('A')
        .into_iter()
        .for_each(|(idx, _)| integer_seq[idx] = 1);
    seq.match_indices('C')
        .into_iter()
        .for_each(|(idx, _)| integer_seq[idx] = 2);
    seq.match_indices('G')
        .into_iter()
        .for_each(|(idx, _)| integer_seq[idx] = 3);
    seq.match_indices('U')
        .into_iter()
        .for_each(|(idx, _)| integer_seq[idx] = 4);

    integer_seq
}

pub fn seq_2_real(seq: &str) -> Vec<f64> {
    let len = seq.len();
    let mut float_seq: Vec<f64> = Vec::new();
    float_seq.resize(len, 0.0);

    seq.match_indices('A')
        .into_iter()
        .for_each(|(idx, _)| float_seq[idx] = -1.5);
    seq.match_indices('C')
        .into_iter()
        .for_each(|(idx, _)| float_seq[idx] = 0.5);
    seq.match_indices('G')
        .into_iter()
        .for_each(|(idx, _)| float_seq[idx] = -0.5);
    seq.match_indices('U')
        .into_iter()
        .for_each(|(idx, _)| float_seq[idx] = 1.5);

    float_seq
}

pub fn fft_sequence<
    T: Copy
        + realfft::num_traits::FromPrimitive
        + realfft::num_traits::Signed
        + std::marker::Send
        + std::marker::Sync
        + std::fmt::Debug
        + 'static,
>(
    seq: &Vec<T>,
) -> Result<Vec<Complex<T>>, FftError> {
    let mut real_planner = RealFftPlanner::<T>::new();

    let r2c = real_planner.plan_fft_forward(seq.len());

    let mut indata = seq.clone();
    let mut spectrum = r2c.make_output_vec();

    r2c.process(&mut indata, &mut spectrum)?;

    // println!("{:?}", spectrum.iter().map(|x| x.powu(2).re ).collect::<Vec<T>>());

    Ok(spectrum)
}

#[cfg(test)]
mod test {
    use crate::stats::fourier;
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

        let mut spectrum = fourier::fft_sequence(&b_a).unwrap();
        let pow_spectrum: Vec<i64> = spectrum.iter_mut().map(|x| x.powu(2).re.abs()).collect();

        println!("{:?}", pow_spectrum);
    }

    #[test]
    fn test_simple_fft_integer() {
        let seq = "AAAAAAGGGGGGGGAAAAAAGGGGGGGGGAAAAAAAGGGGGGGGGGGAAAAAAAAAGGGG";
        let b_a = fourier::seq_2_integer(&seq);

        let mut spectrum = fourier::fft_sequence(&b_a).unwrap();
        let pow_spectrum: Vec<i64> = spectrum.iter_mut().map(|x| x.powu(2).re.abs()).collect();

        println!("{:?}", pow_spectrum);
    }

    #[test]
    fn test_simple_fft_float() {
        let seq = "AGAGAAGAGAAGAGAAGAGAAGAGAAGAGAAGAGAAGAGAAGAGAAGAGAAGAGAAGAGA";
        let b_a = fourier::seq_2_real(&seq);

        let mut spectrum = fourier::fft_sequence(&b_a).unwrap();
        let pow_spectrum: Vec<f64> = spectrum.iter_mut().map(|x| x.powu(2).re.abs()).collect();

        println!("{:?}", pow_spectrum);
    }
}
