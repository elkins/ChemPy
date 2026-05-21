use crate::constants;
use crate::thermo::{NASAModel, NASAPolynomial, WilhoitModel, ThermoModel};
use nalgebra::{DMatrix, DVector};

pub fn convert_wilhoit_to_nasa(
    wilhoit: &WilhoitModel,
    t_min: f64,
    t_max: f64,
    t_int: f64,
    fixed_t_int: bool,
    weighting: bool,
    continuity: usize,
) -> NASAModel {
    // Scale temperatures to kK
    let t_min_k = t_min / 1000.0;
    let t_int_k = t_int / 1000.0;
    let t_max_k = t_max / 1000.0;

    // Create scaled Wilhoit model (Cp/R, B in kK)
    let mut wilhoit_scaled = wilhoit.clone();
    wilhoit_scaled.cp0 /= constants::R;
    wilhoit_scaled.cp_inf /= constants::R;
    wilhoit_scaled.b /= 1000.0;

    let (mut nasa_low, mut nasa_high) = if fixed_t_int {
        wilhoit_to_nasa(&wilhoit_scaled, t_min_k, t_max_k, t_int_k, weighting, continuity)
    } else {
        // For now, only fixed Tint is implemented
        // In a full impl, we would use an optimizer here
        wilhoit_to_nasa(&wilhoit_scaled, t_min_k, t_max_k, t_int_k, weighting, continuity)
    };

    // Restore units
    let t_int_final = t_int_k * 1000.0;
    
    nasa_low.t_min = t_min;
    nasa_low.t_max = t_int_final;
    nasa_high.t_min = t_int_final;
    nasa_high.t_max = t_max;

    // Rescale coefficients from kK basis to K basis
    // Cp/R = a1 + a2*T + a3*T^2 + a4*T^3 + a5*T^4
    // In kK basis: Cp/R = b1 + b2*(T/1000) + b3*(T/1000)^2 + ...
    // So: a1 = b1, a2 = b2/1000, a3 = b3/1000000, etc.
    nasa_low.coeffs[1] /= 1000.0;
    nasa_low.coeffs[2] /= 1_000_000.0;
    nasa_low.coeffs[3] /= 1_000_000_000.0;
    nasa_low.coeffs[4] /= 1_000_000_000_000.0;

    nasa_high.coeffs[1] /= 1000.0;
    nasa_high.coeffs[2] /= 1_000_000.0;
    nasa_high.coeffs[3] /= 1_000_000_000.0;
    nasa_high.coeffs[4] /= 1_000_000_000_000.0;

    // Match Wilhoit H, S at 298.15 K for low polynomial
    let t_ref = 298.15;
    let h_low = (wilhoit.get_enthalpy(t_ref) - nasa_low.get_enthalpy(t_ref)) / constants::R;
    let s_low = (wilhoit.get_entropy(t_ref) - nasa_low.get_entropy(t_ref)) / constants::R;
    nasa_low.coeffs[5] = h_low;
    nasa_low.coeffs[6] = s_low;

    // Match low polynomial H, S at Tint for high polynomial
    let h_high = (nasa_low.get_enthalpy(t_int_final) - nasa_high.get_enthalpy(t_int_final)) / constants::R;
    let s_high = (nasa_low.get_entropy(t_int_final) - nasa_high.get_entropy(t_int_final)) / constants::R;
    nasa_high.coeffs[5] = h_high;
    nasa_high.coeffs[6] = s_high;

    NASAModel::new(t_min, t_max, vec![nasa_low, nasa_high], "Fitted from Wilhoit".to_string())
}

fn wilhoit_to_nasa(
    wilhoit: &WilhoitModel,
    t_min: f64,
    t_max: f64,
    t_int: f64,
    weighting: bool,
    continuity: usize,
) -> (NASAPolynomial, NASAPolynomial) {
    let size = 10 + continuity;
    let mut a = DMatrix::zeros(size, size);
    let mut b = DVector::zeros(size);

    if weighting {
        a[(0, 0)] = 2.0 * (t_int / t_min).ln();
        a[(0, 1)] = 2.0 * (t_int - t_min);
        a[(0, 2)] = t_int * t_int - t_min * t_min;
        a[(0, 3)] = 2.0 * (t_int.powi(3) - t_min.powi(3)) / 3.0;
        a[(0, 4)] = (t_int.powi(4) - t_min.powi(4)) / 2.0;
        a[(1, 4)] = 2.0 * (t_int.powi(5) - t_min.powi(5)) / 5.0;
        a[(2, 4)] = (t_int.powi(6) - t_min.powi(6)) / 3.0;
        a[(3, 4)] = 2.0 * (t_int.powi(7) - t_min.powi(7)) / 7.0;
        a[(4, 4)] = (t_int.powi(8) - t_min.powi(8)) / 4.0;
    } else {
        a[(0, 0)] = 2.0 * (t_int - t_min);
        a[(0, 1)] = t_int * t_int - t_min * t_min;
        a[(0, 2)] = 2.0 * (t_int.powi(3) - t_min.powi(3)) / 3.0;
        a[(0, 3)] = (t_int.powi(4) - t_min.powi(4)) / 2.0;
        a[(0, 4)] = 2.0 * (t_int.powi(5) - t_min.powi(5)) / 5.0;
        a[(1, 4)] = (t_int.powi(6) - t_min.powi(6)) / 3.0;
        a[(2, 4)] = 2.0 * (t_int.powi(7) - t_min.powi(7)) / 7.0;
        a[(3, 4)] = (t_int.powi(8) - t_min.powi(8)) / 4.0;
        a[(4, 4)] = 2.0 * (t_int.powi(9) - t_min.powi(9)) / 9.0;
    }
    a[(1, 1)] = a[(0, 2)];
    a[(1, 2)] = a[(0, 3)];
    a[(1, 3)] = a[(0, 4)];
    a[(2, 2)] = a[(0, 4)];
    a[(2, 3)] = a[(1, 4)];
    a[(3, 3)] = a[(2, 4)];

    // Symmetric parts for low range
    for i in 1..5 {
        for j in 0..i {
            a[(i, j)] = a[(j, i)];
        }
    }

    if weighting {
        a[(5, 5)] = 2.0 * (t_max / t_int).ln();
        a[(5, 6)] = 2.0 * (t_max - t_int);
        a[(5, 7)] = t_max * t_max - t_int * t_int;
        a[(5, 8)] = 2.0 * (t_max.powi(3) - t_int.powi(3)) / 3.0;
        a[(5, 9)] = (t_max.powi(4) - t_int.powi(4)) / 2.0;
        a[(6, 9)] = 2.0 * (t_max.powi(5) - t_int.powi(5)) / 5.0;
        a[(7, 9)] = (t_max.powi(6) - t_int.powi(6)) / 3.0;
        a[(8, 9)] = 2.0 * (t_max.powi(7) - t_int.powi(7)) / 7.0;
        a[(9, 9)] = (t_max.powi(8) - t_int.powi(8)) / 4.0;
    } else {
        a[(5, 5)] = 2.0 * (t_max - t_int);
        a[(5, 6)] = t_max * t_max - t_int * t_int;
        a[(5, 7)] = 2.0 * (t_max.powi(3) - t_int.powi(3)) / 3.0;
        a[(5, 8)] = (t_max.powi(4) - t_int.powi(4)) / 2.0;
        a[(5, 9)] = 2.0 * (t_max.powi(5) - t_int.powi(5)) / 5.0;
        a[(6, 9)] = (t_max.powi(6) - t_int.powi(6)) / 3.0;
        a[(7, 9)] = 2.0 * (t_max.powi(7) - t_int.powi(7)) / 7.0;
        a[(8, 9)] = (t_max.powi(8) - t_int.powi(8)) / 4.0;
        a[(9, 9)] = 2.0 * (t_max.powi(9) - t_int.powi(9)) / 9.0;
    }
    a[(6, 6)] = a[(5, 7)];
    a[(6, 7)] = a[(5, 8)];
    a[(6, 8)] = a[(5, 9)];
    a[(7, 7)] = a[(5, 9)];
    a[(7, 8)] = a[(6, 9)];
    a[(8, 8)] = a[(7, 9)];

    // Symmetric parts for high range
    for i in 6..10 {
        for j in 5..i {
            a[(i, j)] = a[(j, i)];
        }
    }

    // Continuity constraints
    if continuity > 0 {
        a[(0, 10)] = 1.0;
        a[(1, 10)] = t_int;
        a[(2, 10)] = t_int * t_int;
        a[(3, 10)] = a[(2, 10)] * t_int;
        a[(4, 10)] = a[(3, 10)] * t_int;
        a[(5, 10)] = -1.0;
        a[(6, 10)] = -t_int;
        a[(7, 10)] = -t_int * t_int;
        a[(8, 10)] = -t_int * t_int * t_int;
        a[(9, 10)] = -t_int * t_int * t_int * t_int;
        
        if continuity > 1 {
            a[(1, 11)] = 1.0;
            a[(2, 11)] = 2.0 * t_int;
            a[(3, 11)] = 3.0 * t_int * t_int;
            a[(4, 11)] = 4.0 * t_int * t_int * t_int;
            a[(6, 11)] = -1.0;
            a[(7, 11)] = -2.0 * t_int;
            a[(8, 11)] = -3.0 * t_int * t_int;
            a[(9, 11)] = -4.0 * t_int * t_int * t_int;
        }
        if continuity > 2 {
            a[(2, 12)] = 2.0;
            a[(3, 12)] = 6.0 * t_int;
            a[(4, 12)] = 12.0 * t_int * t_int;
            a[(7, 12)] = -2.0;
            a[(8, 12)] = -6.0 * t_int;
            a[(9, 12)] = -12.0 * t_int * t_int;
        }
    }

    // Symmetric constraints
    for i in 10..size {
        for j in 0..i {
            a[(i, j)] = a[(j, i)];
        }
    }

    // Construct b vector
    let w0int = wilhoit_integral_t0(wilhoit, t_int);
    let w1int = wilhoit_integral_t1(wilhoit, t_int);
    let w2int = wilhoit_integral_t2(wilhoit, t_int);
    let w3int = wilhoit_integral_t3(wilhoit, t_int);
    let w0min = wilhoit_integral_t0(wilhoit, t_min);
    let w1min = wilhoit_integral_t1(wilhoit, t_min);
    let w2min = wilhoit_integral_t2(wilhoit, t_min);
    let w3min = wilhoit_integral_t3(wilhoit, t_min);
    let w0max = wilhoit_integral_t0(wilhoit, t_max);
    let w1max = wilhoit_integral_t1(wilhoit, t_max);
    let w2max = wilhoit_integral_t2(wilhoit, t_max);
    let w3max = wilhoit_integral_t3(wilhoit, t_max);

    if weighting {
        let wm1int = wilhoit_integral_tm1(wilhoit, t_int);
        let wm1min = wilhoit_integral_tm1(wilhoit, t_min);
        let wm1max = wilhoit_integral_tm1(wilhoit, t_max);
        
        b[0] = 2.0 * (wm1int - wm1min);
        b[1] = 2.0 * (w0int - w0min);
        b[2] = 2.0 * (w1int - w1min);
        b[3] = 2.0 * (w2int - w2min);
        b[4] = 2.0 * (w3int - w3min);
        b[5] = 2.0 * (wm1max - wm1int);
        b[6] = 2.0 * (w0max - w0int);
        b[7] = 2.0 * (w1max - w1int);
        b[8] = 2.0 * (w2max - w2int);
        b[9] = 2.0 * (w3max - w3int);
    } else {
        let w4int = wilhoit_integral_t4(wilhoit, t_int);
        let w4min = wilhoit_integral_t4(wilhoit, t_min);
        let w4max = wilhoit_integral_t4(wilhoit, t_max);

        b[0] = 2.0 * (w0int - w0min);
        b[1] = 2.0 * (w1int - w1min);
        b[2] = 2.0 * (w2int - w2min);
        b[3] = 2.0 * (w3int - w3min);
        b[4] = 2.0 * (w4int - w4min);
        b[5] = 2.0 * (w0max - w0int);
        b[6] = 2.0 * (w1max - w1int);
        b[7] = 2.0 * (w2max - w2int);
        b[8] = 2.0 * (w3max - w3int);
        b[9] = 2.0 * (w4max - w4int);
    }

    // Solve Ax = b
    let x = a.full_piv_lu().solve(&b).expect("Linear system solver failed");

    let nasa_low = NASAPolynomial::new(0.0, 0.0, [x[0], x[1], x[2], x[3], x[4], 0.0, 0.0]);
    let nasa_high = NASAPolynomial::new(0.0, 0.0, [x[5], x[6], x[7], x[8], x[9], 0.0, 0.0]);

    (nasa_low, nasa_high)
}

// Analytical integrals for Wilhoit model
// These assume scaled parameters (Cp/R, B in kK)

fn wilhoit_integral_t0(wilhoit: &WilhoitModel, t: f64) -> f64 {
    let cp0 = wilhoit.cp0;
    let cp_inf = wilhoit.cp_inf;
    let b = wilhoit.b;
    let a0 = wilhoit.a0;
    let a1 = wilhoit.a1;
    let a2 = wilhoit.a2;
    let a3 = wilhoit.a3;

    let y = t / (t + b);
    let y2 = y * y;
    let log_b_plus_t = (b + t).ln();
    
    cp0 * t - (cp_inf - cp0) * t * (
        y2 * (
            (3.0 * a0 + a1 + a2 + a3) / 6.0
            + (4.0 * a1 + a2 + a3) * y / 12.0
            + (5.0 * a2 + a3) * y2 / 20.0
            + a3 * y2 * y / 5.0
        )
        + (2.0 + a0 + a1 + a2 + a3) * (y / 2.0 - 1.0 + (1.0 / y - 1.0) * log_b_plus_t)
    )
}

fn wilhoit_integral_tm1(wilhoit: &WilhoitModel, t: f64) -> f64 {
    let cp0 = wilhoit.cp0;
    let cp_inf = wilhoit.cp_inf;
    let b = wilhoit.b;
    let a0 = wilhoit.a0;
    let a1 = wilhoit.a1;
    let a2 = wilhoit.a2;
    let a3 = wilhoit.a3;

    let y = t / (t + b);
    cp_inf * t.ln() - (cp_inf - cp0) * (
        y.ln() + y * (1.0 + y * (a0 / 2.0 + y * (a1 / 3.0 + y * (a2 / 4.0 + y * a3 / 5.0))))
    )
}

fn wilhoit_integral_t1(wilhoit: &WilhoitModel, t: f64) -> f64 {
    let cp0 = wilhoit.cp0;
    let cp_inf = wilhoit.cp_inf;
    let b = wilhoit.b;
    let a0 = wilhoit.a0;
    let a1 = wilhoit.a1;
    let a2 = wilhoit.a2;
    let a3 = wilhoit.a3;

    let log_b_plus_t = (b + t).ln();
    (2.0 + a0 + a1 + a2 + a3) * b * (cp0 - cp_inf) * t
    + (cp_inf * t * t) / 2.0
    + (a3 * b.powi(7) * (cp_inf - cp0)) / (5.0 * (b + t).powi(5))
    + ((a2 + 6.0 * a3) * b.powi(6) * (cp0 - cp_inf)) / (4.0 * (b + t).powi(4))
    - ((a1 + 5.0 * (a2 + 3.0 * a3)) * b.powi(5) * (cp0 - cp_inf)) / (3.0 * (b + t).powi(3))
    + ((a0 + 4.0 * a1 + 10.0 * (a2 + 2.0 * a3)) * b.powi(4) * (cp0 - cp_inf)) / (2.0 * (b + t).powi(2))
    - ((1.0 + 3.0 * a0 + 6.0 * a1 + 10.0 * a2 + 15.0 * a3) * b.powi(3) * (cp0 - cp_inf)) / (b + t)
    - (3.0 + 3.0 * a0 + 4.0 * a1 + 5.0 * a2 + 6.0 * a3) * b * b * (cp0 - cp_inf) * log_b_plus_t
}

fn wilhoit_integral_t2(wilhoit: &WilhoitModel, t: f64) -> f64 {
    let cp0 = wilhoit.cp0;
    let cp_inf = wilhoit.cp_inf;
    let b = wilhoit.b;
    let a0 = wilhoit.a0;
    let a1 = wilhoit.a1;
    let a2 = wilhoit.a2;
    let a3 = wilhoit.a3;

    let log_b_plus_t = (b + t).ln();
    -((3.0 + 3.0 * a0 + 4.0 * a1 + 5.0 * a2 + 6.0 * a3) * b * b * (cp0 - cp_inf) * t)
    + ((2.0 + a0 + a1 + a2 + a3) * b * (cp0 - cp_inf) * t * t) / 2.0
    + (cp_inf * t.powi(3)) / 3.0
    + (a3 * b.powi(8) * (cp0 - cp_inf)) / (5.0 * (b + t).powi(5))
    - ((a2 + 7.0 * a3) * b.powi(7) * (cp0 - cp_inf)) / (4.0 * (b + t).powi(4))
    + ((a1 + 6.0 * a2 + 21.0 * a3) * b.powi(6) * (cp0 - cp_inf)) / (3.0 * (b + t).powi(3))
    - ((a0 + 5.0 * (a1 + 3.0 * a2 + 7.0 * a3)) * b.powi(5) * (cp0 - cp_inf)) / (2.0 * (b + t).powi(2))
    + ((1.0 + 4.0 * a0 + 10.0 * a1 + 20.0 * a2 + 35.0 * a3) * b.powi(4) * (cp0 - cp_inf)) / (b + t)
    + (4.0 + 6.0 * a0 + 10.0 * a1 + 15.0 * a2 + 21.0 * a3) * b.powi(3) * (cp0 - cp_inf) * log_b_plus_t
}

fn wilhoit_integral_t3(wilhoit: &WilhoitModel, t: f64) -> f64 {
    let cp0 = wilhoit.cp0;
    let cp_inf = wilhoit.cp_inf;
    let b = wilhoit.b;
    let a0 = wilhoit.a0;
    let a1 = wilhoit.a1;
    let a2 = wilhoit.a2;
    let a3 = wilhoit.a3;

    let log_b_plus_t = (b + t).ln();
    (4.0 + 6.0 * a0 + 10.0 * a1 + 15.0 * a2 + 21.0 * a3) * b.powi(3) * (cp0 - cp_inf) * t
    + ((3.0 + 3.0 * a0 + 4.0 * a1 + 5.0 * a2 + 6.0 * a3) * b * b * (cp_inf - cp0) * t * t) / 2.0
    + ((2.0 + a0 + a1 + a2 + a3) * b * (cp0 - cp_inf) * t.powi(3)) / 3.0
    + (cp_inf * t.powi(4)) / 4.0
    + (a3 * b.powi(9) * (cp_inf - cp0)) / (5.0 * (b + t).powi(5))
    + ((a2 + 8.0 * a3) * b.powi(8) * (cp0 - cp_inf)) / (4.0 * (b + t).powi(4))
    - ((a1 + 7.0 * (a2 + 4.0 * a3)) * b.powi(7) * (cp0 - cp_inf)) / (3.0 * (b + t).powi(3))
    + ((a0 + 6.0 * a1 + 21.0 * a2 + 56.0 * a3) * b.powi(6) * (cp0 - cp_inf)) / (2.0 * (b + t).powi(2))
    - ((1.0 + 5.0 * a0 + 15.0 * a1 + 35.0 * a2 + 70.0 * a3) * b.powi(5) * (cp0 - cp_inf)) / (b + t)
    - (5.0 + 10.0 * a0 + 20.0 * a1 + 35.0 * a2 + 56.0 * a3) * b.powi(4) * (cp0 - cp_inf) * log_b_plus_t
}

fn wilhoit_integral_t4(wilhoit: &WilhoitModel, t: f64) -> f64 {
    let cp0 = wilhoit.cp0;
    let cp_inf = wilhoit.cp_inf;
    let b = wilhoit.b;
    let a0 = wilhoit.a0;
    let a1 = wilhoit.a1;
    let a2 = wilhoit.a2;
    let a3 = wilhoit.a3;

    let log_b_plus_t = (b + t).ln();
    -((5.0 + 10.0 * a0 + 20.0 * a1 + 35.0 * a2 + 56.0 * a3) * b.powi(4) * (cp0 - cp_inf) * t)
    + ((4.0 + 6.0 * a0 + 10.0 * a1 + 15.0 * a2 + 21.0 * a3) * b.powi(3) * (cp0 - cp_inf) * t * t) / 2.0
    + ((3.0 + 3.0 * a0 + 4.0 * a1 + 5.0 * a2 + 6.0 * a3) * b * b * (cp_inf - cp0) * t.powi(3)) / 3.0
    + ((2.0 + a0 + a1 + a2 + a3) * b * (cp0 - cp_inf) * t.powi(4)) / 4.0
    + (cp_inf * t.powi(5)) / 5.0
    + (a3 * b.powi(10) * (cp0 - cp_inf)) / (5.0 * (b + t).powi(5))
    - ((a2 + 9.0 * a3) * b.powi(9) * (cp0 - cp_inf)) / (4.0 * (b + t).powi(4))
    + ((a1 + 8.0 * a2 + 36.0 * a3) * b.powi(8) * (cp0 - cp_inf)) / (3.0 * (b + t).powi(3))
    - ((a0 + 7.0 * (a1 + 4.0 * (a2 + 3.0 * a3))) * b.powi(7) * (cp0 - cp_inf)) / (2.0 * (b + t).powi(2))
    + ((1.0 + 6.0 * a0 + 21.0 * a1 + 56.0 * a2 + 126.0 * a3) * b.powi(6) * (cp0 - cp_inf)) / (b + t)
    + (6.0 + 15.0 * a0 + 35.0 * a1 + 70.0 * a2 + 126.0 * a3) * b.powi(5) * (cp0 - cp_inf) * log_b_plus_t
}

pub fn convert_ga_to_wilhoit(
    t_data: &[f64],
    cp_data: &[f64],
    atoms: usize,
    rotors: usize,
    linear: bool,
    h298: f64,
    s298: f64,
    b0: f64,
) -> WilhoitModel {
    let mut wilhoit = WilhoitModel::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 500.0);
    let freq = 3 * atoms - (if linear { 5 } else { 6 }) - rotors;
    wilhoit.fit_to_data(t_data, cp_data, linear, freq, rotors, h298, s298, b0);
    wilhoit
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants;
    use crate::thermo::ThermoModel;

    #[test]
    fn test_wilhoit_to_nasa() {
        let wilhoit = WilhoitModel::new(
            4.0 * constants::R,
            20.0 * constants::R,
            0.0,
            0.0,
            0.0,
            0.0,
            100000.0,
            200.0,
            500.0,
        );

        let nasa = convert_wilhoit_to_nasa(&wilhoit, 300.0, 3000.0, 1000.0, true, true, 3);
        
        for t in [500.0, 1000.0, 1500.0, 2000.0] {
            let cp_w = wilhoit.get_heat_capacity(t);
            let cp_n = nasa.get_heat_capacity(t);
            assert!((cp_w / cp_n - 1.0).abs() < 0.05);
            
            let h_w = wilhoit.get_enthalpy(t);
            let h_n = nasa.get_enthalpy(t);
            assert!((h_w / h_n - 1.0).abs() < 0.05);

            let s_w = wilhoit.get_entropy(t);
            let s_n = nasa.get_entropy(t);
            assert!((s_w / s_n - 1.0).abs() < 0.05);
        }
    }

    #[test]
    fn test_ga_to_wilhoit() {
        // Ethane data
        let t_data = vec![300.0, 400.0, 500.0, 600.0, 800.0, 1000.0, 1500.0];
        let cp_data = vec![52.4, 65.2, 77.8, 89.1, 107.5, 122.2, 146.4];
        let h298 = -84.0 * 1000.0;
        let s298 = 229.5;
        
        let wilhoit = convert_ga_to_wilhoit(&t_data, &cp_data, 8, 1, false, h298, s298, 500.0);
        
        for i in 0..t_data.len() {
            let cp_w = wilhoit.get_heat_capacity(t_data[i]);
            assert!((cp_w / cp_data[i] - 1.0).abs() < 0.02);
        }
        
        assert!((wilhoit.get_enthalpy(298.15) / h298 - 1.0).abs() < 0.01);
        assert!((wilhoit.get_entropy(298.15) / s298 - 1.0).abs() < 0.01);
    }
}
