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
}
