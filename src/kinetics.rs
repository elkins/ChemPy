use crate::constants;

pub trait KineticsModel {
    fn get_rate_coefficient(&self, t: f64, p: f64) -> f64;

    fn is_temperature_valid(&self, t: f64, t_min: f64, t_max: f64) -> bool {
        t >= t_min && t <= t_max
    }

    fn is_pressure_valid(&self, p: f64, p_min: f64, p_max: f64) -> bool {
        p >= p_min && p <= p_max
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrheniusModel {
    pub a: f64,
    pub n: f64,
    pub ea: f64,
    pub t0: f64,
    pub t_min: f64,
    pub t_max: f64,
}

impl ArrheniusModel {
    pub fn new(a: f64, n: f64, ea: f64, t0: f64) -> Self {
        ArrheniusModel {
            a,
            n,
            ea,
            t0,
            t_min: 0.0,
            t_max: 1.0e10,
        }
    }
}

impl KineticsModel for ArrheniusModel {
    fn get_rate_coefficient(&self, t: f64, _p: f64) -> f64 {
        self.a * (t / self.t0).powf(self.n) * (-self.ea / constants::R / t).exp()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arrhenius_rate_coefficient() {
        let model = ArrheniusModel::new(1.0e10, 0.0, 50000.0, 1.0);
        let t = 1000.0;
        let k_expected = 1.0e10 * (-50000.0 / (constants::R * t)).exp();
        let k_actual = model.get_rate_coefficient(t, 1.0e5);
        assert!((k_actual - k_expected).abs() < 1e-10 * k_expected);
    }
}
