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
