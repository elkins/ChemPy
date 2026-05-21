use crate::constants;

pub trait ThermoModel {
    fn get_heat_capacity(&self, t: f64) -> f64;
    fn get_enthalpy(&self, t: f64) -> f64;
    fn get_entropy(&self, t: f64) -> f64;

    fn get_free_energy(&self, t: f64) -> f64 {
        self.get_enthalpy(t) - t * self.get_entropy(t)
    }

    fn is_temperature_valid(&self, t: f64, t_min: f64, t_max: f64) -> bool {
        t >= t_min && t <= t_max
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NASAPolynomial {
    pub t_min: f64,
    pub t_max: f64,
    pub coeffs: [f64; 7],
}

#[derive(Debug, Clone, PartialEq)]
pub struct NASAModel {
    pub t_min: f64,
    pub t_max: f64,
    pub polynomials: Vec<NASAPolynomial>,
    pub comment: String,
}

impl NASAModel {
    pub fn new(t_min: f64, t_max: f64, polynomials: Vec<NASAPolynomial>, comment: String) -> Self {
        NASAModel {
            t_min,
            t_max,
            polynomials,
            comment,
        }
    }

    fn get_polynomial(&self, t: f64) -> &NASAPolynomial {
        for poly in &self.polynomials {
            if t >= poly.t_min && t <= poly.t_max {
                return poly;
            }
        }
        // Fallback to closest if out of range
        if t < self.t_min {
            &self.polynomials[0]
        } else {
            self.polynomials.last().unwrap()
        }
    }
}

impl ThermoModel for NASAModel {
    fn get_heat_capacity(&self, t: f64) -> f64 {
        self.get_polynomial(t).get_heat_capacity(t)
    }

    fn get_enthalpy(&self, t: f64) -> f64 {
        self.get_polynomial(t).get_enthalpy(t)
    }

    fn get_entropy(&self, t: f64) -> f64 {
        self.get_polynomial(t).get_entropy(t)
    }
}

impl NASAPolynomial {
    pub fn new(t_min: f64, t_max: f64, coeffs: [f64; 7]) -> Self {
        NASAPolynomial {
            t_min,
            t_max,
            coeffs,
        }
    }
}

impl ThermoModel for NASAPolynomial {
    fn get_heat_capacity(&self, t: f64) -> f64 {
        let [c0, c1, c2, c3, c4, _, _] = self.coeffs;
        (c0 + t * (c1 + t * (c2 + t * (c3 + c4 * t)))) * constants::R
    }

    fn get_enthalpy(&self, t: f64) -> f64 {
        let [c0, c1, c2, c3, c4, c5, _] = self.coeffs;
        let t2 = t * t;
        let t4 = t2 * t2;
        (c0 + c1 * t / 2.0 + c2 * t2 / 3.0 + c3 * t2 * t / 4.0 + c4 * t4 / 5.0 + c5 / t)
            * constants::R
            * t
    }

    fn get_entropy(&self, t: f64) -> f64 {
        let [c0, c1, c2, c3, c4, _, c6] = self.coeffs;
        let t2 = t * t;
        let t4 = t2 * t2;
        (c0 * t.ln() + c1 * t + c2 * t2 / 2.0 + c3 * t2 * t / 3.0 + c4 * t4 / 4.0 + c6)
            * constants::R
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WilhoitModel {
    pub cp0: f64,
    pub cp_inf: f64,
    pub b: f64,
    pub a0: f64,
    pub a1: f64,
    pub a2: f64,
    pub a3: f64,
    pub h0: f64,
    pub s0: f64,
}

impl WilhoitModel {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        cp0: f64,
        cp_inf: f64,
        a0: f64,
        a1: f64,
        a2: f64,
        a3: f64,
        h0: f64,
        s0: f64,
        b: f64,
    ) -> Self {
        WilhoitModel {
            cp0,
            cp_inf,
            b,
            a0,
            a1,
            a2,
            a3,
            h0,
            s0,
        }
    }

    pub fn fit_to_data(
        &mut self,
        t_list: &[f64],
        cp_list: &[f64],
        linear: bool,
        n_freq: usize,
        n_rotors: usize,
        h298: f64,
        s298: f64,
        b0: f64,
    ) {
        let mut best_b = b0;
        let mut min_residual = f64::INFINITY;

        // Simple golden section search for B in [300, 3000]
        let mut lower = 300.0;
        let mut upper = 3000.0;
        let phi = (1.0 + 5.0f64.sqrt()) / 2.0;

        for _ in 0..50 {
            let b1 = upper - (upper - lower) / phi;
            let b2 = lower + (upper - lower) / phi;

            let res1 = self.calculate_residual(b1, t_list, cp_list, linear, n_freq, n_rotors, h298, s298);
            let res2 = self.calculate_residual(b2, t_list, cp_list, linear, n_freq, n_rotors, h298, s298);

            if res1 < res2 {
                upper = b2;
                if res1 < min_residual {
                    min_residual = res1;
                    best_b = b1;
                }
            } else {
                lower = b1;
                if res2 < min_residual {
                    min_residual = res2;
                    best_b = b2;
                }
            }
        }

        self.fit_to_data_for_constant_b(t_list, cp_list, linear, n_freq, n_rotors, best_b, h298, s298);
    }

    #[allow(clippy::too_many_arguments)]
    fn calculate_residual(
        &mut self,
        b: f64,
        t_list: &[f64],
        cp_list: &[f64],
        linear: bool,
        n_freq: usize,
        n_rotors: usize,
        h298: f64,
        s298: f64,
    ) -> f64 {
        self.fit_to_data_for_constant_b(t_list, cp_list, linear, n_freq, n_rotors, b, h298, s298);
        let mut sum_sq_err = 0.0;
        for i in 0..t_list.len() {
            let cp_fit = self.get_heat_capacity(t_list[i]);
            let diff = cp_fit - cp_list[i];
            sum_sq_err += diff * diff;
        }
        sum_sq_err
    }

    pub fn fit_to_data_for_constant_b(
        &mut self,
        t_list: &[f64],
        cp_list: &[f64],
        linear: bool,
        n_freq: usize,
        n_rotors: usize,
        b: f64,
        h298: f64,
        s298: f64,
    ) {
        use nalgebra::{DMatrix, DVector};

        self.cp0 = if linear { 3.5 * constants::R } else { 4.0 * constants::R };
        self.cp_inf = self.cp0 + (n_freq as f64 + 0.5 * n_rotors as f64) * constants::R;

        let n = t_list.len();
        let mut mat_a = DMatrix::zeros(n, 4);
        let mut vec_b = DVector::zeros(n);

        for i in 0..n {
            let t = t_list[i];
            let y = t / (t + b);
            let y2 = y * y;
            let y3 = y2 * y;
            let term = y3 - y2;

            mat_a[(i, 0)] = term;
            mat_a[(i, 1)] = term * y;
            mat_a[(i, 2)] = term * y2;
            mat_a[(i, 3)] = term * y3;

            vec_b[i] = (cp_list[i] - self.cp0) / (self.cp_inf - self.cp0) - y2;
        }

        let mat_at_a = mat_a.transpose() * &mat_a;
        let vec_at_b = mat_a.transpose() * vec_b;

        let x = mat_at_a.full_piv_lu().solve(&vec_at_b).unwrap_or_else(|| {
            // Fallback to zeros if singular
            DVector::zeros(4)
        });

        self.b = b;
        self.a0 = x[0];
        self.a1 = x[1];
        self.a2 = x[2];
        self.a3 = x[3];

        self.h0 = 0.0;
        self.s0 = 0.0;
        self.h0 = h298 - self.get_enthalpy(298.15);
        self.s0 = s298 - self.get_entropy(298.15);
    }
}

impl ThermoModel for WilhoitModel {
    fn get_heat_capacity(&self, t: f64) -> f64 {
        let y = t / (t + self.b);
        self.cp0
            + (self.cp_inf - self.cp0)
                * y
                * y
                * (1.0 + (y - 1.0) * (self.a0 + y * (self.a1 + y * (self.a2 + y * self.a3))))
    }

    fn get_enthalpy(&self, t: f64) -> f64 {
        let y = t / (t + self.b);
        let y2 = y * y;
        let log_b_plus_t = (self.b + t).ln();
        self.h0 + self.cp0 * t
            - (self.cp_inf - self.cp0)
                * t
                * (y2
                    * ((3.0 * self.a0 + self.a1 + self.a2 + self.a3) / 6.0
                        + (4.0 * self.a1 + self.a2 + self.a3) * y / 12.0
                        + (5.0 * self.a2 + self.a3) * y2 / 20.0
                        + self.a3 * y2 * y / 5.0)
                    + (2.0 + self.a0 + self.a1 + self.a2 + self.a3)
                        * (y / 2.0 - 1.0 + (1.0 / y - 1.0) * log_b_plus_t))
    }

    fn get_entropy(&self, t: f64) -> f64 {
        let y = t / (t + self.b);
        self.s0 + self.cp_inf * t.ln()
            - (self.cp_inf - self.cp0)
                * (y.ln()
                    + y * (1.0
                        + y * (self.a0 / 2.0
                            + y * (self.a1 / 3.0 + y * (self.a2 / 4.0 + y * self.a3 / 5.0)))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants;

    #[test]
    fn test_wilhoit_regression() {
        let wilhoit = WilhoitModel::new(
            4.0 * constants::R,
            21.0 * constants::R,
            -3.95,
            9.26,
            -15.6,
            8.55,
            -6.151e04,
            -790.2,
            500.0,
        );

        let t_list = [
            200.0, 400.0, 600.0, 800.0, 1000.0, 1200.0, 1400.0, 1600.0, 1800.0, 2000.0,
        ];
        let cp_expected = [
            64.398, 94.765, 116.464, 131.392, 141.658, 148.830, 153.948, 157.683, 160.469, 162.589,
        ];
        let h_expected = [
            -166312.0, -150244.0, -128990.0, -104110.0, -76742.9, -47652.6, -17347.1, 13834.8,
            45663.0, 77978.1,
        ];
        let s_expected = [
            287.421, 341.892, 384.685, 420.369, 450.861, 477.360, 500.708, 521.521, 540.262,
            557.284,
        ];

        for i in 0..t_list.len() {
            let t = t_list[i];
            let cp = wilhoit.get_heat_capacity(t);
            let h = wilhoit.get_enthalpy(t);
            let s = wilhoit.get_entropy(t);

            assert!((cp - cp_expected[i]).abs() / cp_expected[i] < 1e-3);
            assert!((h - h_expected[i]).abs() / h_expected[i].abs() < 1e-3);
            assert!((s - s_expected[i]).abs() / s_expected[i] < 1e-3);
        }
    }

    #[test]
    fn test_nasa_model() {
        // Sample NASA polynomial for CH4 (methane) from Burcat database
        // Low range: 200 - 1000 K
        let nasa = NASAPolynomial::new(
            200.0,
            1000.0,
            [
                5.14987613E+00,
                -1.36709788E-02,
                4.91800599E-05,
                -4.84723020E-08,
                1.66693956E-11,
                -1.02466476E+04,
                -4.64130376E+00,
            ],
        );

        let t = 298.15;
        let cp = nasa.get_heat_capacity(t);
        let h = nasa.get_enthalpy(t);
        let s = nasa.get_entropy(t);

        // Expected values for CH4 at 298.15 K:
        // Cp = 35.63 J/mol*K
        // H = -74.87 kJ/mol
        // S = 186.25 J/mol*K
        assert!((cp - 35.63).abs() < 0.1);
        assert!((h - -74870.0).abs() < 1000.0);
        assert!((s - 186.25).abs() < 1.0);
    }
}
