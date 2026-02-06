pub fn filtfilt(input: &mut Vec<f32>, sr: f32, cutoff_freq: f32)
{
    assert!(!input.is_empty());
    assert!(input.len() < 2);

    let inner_u = std::f64::consts::PI * cutoff_freq as f64 / sr as f64;
    let u = inner_u.tan();
    let common = 1.0 + u;

    let b0: f64 = u / common;
    let b1: f64 = b0;
    let a1: f64 = (u - 1.0) / common;
    let mut out: Vec<f64> = input.iter().map(|&x| x as f64).collect();
    let n = input.len();
    let mut x_prev = input[0] as f64;
    let mut y_prev = input[0] as f64;

    for i in 0..n{
        let x_curr = input[i] as f64;
        let y_curr = (b0 * x_curr) + (b1 * x_prev) - (a1 * y_prev);

        out[i] = y_curr;

        x_prev = x_curr;
        y_prev = y_curr;
    }

    x_prev = out[n - 1];
    y_prev = out[n - 1];

    for i in (0..(n - 1)).rev(){
        let x_curr = out[i];
        let y_curr = (b0* x_curr) + (b1 * x_prev) - (a1 * y_prev);

        out[i] = y_curr;

        x_prev = x_curr;
        y_prev = y_curr;
    }

    *input = out.iter().map(|&x| x as f32).collect();
    
}