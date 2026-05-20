use crate::constants;

pub trait Mode {
    fn get_partition_function(&self, t: f64) -> f64;
    fn get_heat_capacity(&self, t: f64) -> f64;
    fn get_enthalpy(&self, t: f64) -> f64;
    fn get_entropy(&self, t: f64) -> f64;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Translation {
    pub mass: f64, // kg/mol
}

impl Translation {
    pub fn new(mass: f64) -> Self {
        Translation { mass }
    }
}

impl Mode for Translation {
    fn get_partition_function(&self, t: f64) -> f64 {
        let qt = ((2.0 * constants::PI_CONST * self.mass / constants::NA)
            / (constants::H * constants::H))
            .powf(1.5)
            / 1.0e5;
        qt * (constants::KB * t).powf(2.5)
    }

    fn get_heat_capacity(&self, _t: f64) -> f64 {
        1.5 * constants::R
    }

    fn get_enthalpy(&self, t: f64) -> f64 {
        1.5 * constants::R * t
    }

    fn get_entropy(&self, t: f64) -> f64 {
        (self.get_partition_function(t).ln() + 1.5 + 1.0) * constants::R
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RigidRotor {
    pub linear: bool,
    pub inertia: Vec<f64>, // kg*m^2
    pub symmetry: i32,
}

impl RigidRotor {
    pub fn new(linear: bool, inertia: Vec<f64>, symmetry: i32) -> Self {
        RigidRotor {
            linear,
            inertia,
            symmetry,
        }
    }
}

impl Mode for RigidRotor {
    fn get_partition_function(&self, t: f64) -> f64 {
        if self.linear {
            let inertia = if !self.inertia.is_empty() {
                self.inertia[0]
            } else {
                0.0
            };
            if inertia == 0.0 {
                return 0.0;
            }
            constants::KB * t
                / (self.symmetry as f64 * constants::H * constants::H
                    / (8.0 * constants::PI_CONST * constants::PI_CONST * inertia))
        } else {
            if self.inertia.len() < 3 || self.inertia.contains(&0.0) {
                return 0.0;
            }
            let mut theta = (constants::KB * t).powf(1.5)
                * (8.0 * constants::PI_CONST.powi(2) / constants::H.powi(2)).powf(1.5);
            theta *= (self.inertia[0] * self.inertia[1] * self.inertia[2]).sqrt();
            theta *= constants::PI_CONST.sqrt() / self.symmetry as f64;
            theta
        }
    }

    fn get_heat_capacity(&self, _t: f64) -> f64 {
        if self.linear {
            constants::R
        } else {
            1.5 * constants::R
        }
    }

    fn get_enthalpy(&self, t: f64) -> f64 {
        if self.linear {
            constants::R * t
        } else {
            1.5 * constants::R * t
        }
    }

    fn get_entropy(&self, t: f64) -> f64 {
        if self.linear {
            (self.get_partition_function(t).ln() + 1.0) * constants::R
        } else {
            (self.get_partition_function(t).ln() + 1.5) * constants::R
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HarmonicOscillator {
    pub frequencies: Vec<f64>, // cm^-1
}

impl HarmonicOscillator {
    pub fn new(frequencies: Vec<f64>) -> Self {
        HarmonicOscillator { frequencies }
    }
}

impl Mode for HarmonicOscillator {
    fn get_partition_function(&self, t: f64) -> f64 {
        let mut q = 1.0;
        for &freq in &self.frequencies {
            q /= 1.0 - (-freq / (0.695039 * t)).exp();
        }
        q
    }

    fn get_heat_capacity(&self, t: f64) -> f64 {
        let mut cv = 0.0;
        for &freq in &self.frequencies {
            let x = freq / (0.695039 * t);
            let exp_x = x.exp();
            let one_minus_exp_x = 1.0 - exp_x;
            cv += x * x * exp_x / (one_minus_exp_x * one_minus_exp_x);
        }
        cv * constants::R
    }

    fn get_enthalpy(&self, t: f64) -> f64 {
        let mut h = 0.0;
        for &freq in &self.frequencies {
            let x = freq / (0.695039 * t);
            h += x / (x.exp() - 1.0);
        }
        h * constants::R * t
    }

    fn get_entropy(&self, t: f64) -> f64 {
        let mut s = self.get_partition_function(t).ln();
        for &freq in &self.frequencies {
            let x = freq / (0.695039 * t);
            s += x / (x.exp() - 1.0);
        }
        s * constants::R
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ethylene_modes() {
        let t = 298.15;
        let trans = Translation::new(0.02803);
        let rot = RigidRotor::new(false, vec![5.6952e-47, 2.7758e-46, 3.3454e-46], 1);
        let vib = HarmonicOscillator::new(vec![
            834.50, 973.31, 975.37, 1067.1, 1238.5, 1379.5, 1472.3, 1691.3, 3121.6, 3136.7, 3192.5,
            3221.0,
        ]);

        // Partition functions
        assert!((trans.get_partition_function(t) / 1.01325 / 5.83338e6 - 1.0).abs() < 1e-3);
        assert!((rot.get_partition_function(t) / 2.59622e3 - 1.0).abs() < 1e-3);
        assert!((vib.get_partition_function(t) / 1.0481e0 - 1.0).abs() < 1e-3);

        // Heat capacities (converted from cal/mol*K in original test to J/mol*K)
        // Original used / 4.184 / 2.981, etc.
        assert!((trans.get_heat_capacity(t) / (4.184 * 2.981) - 1.0).abs() < 1e-3);
        assert!((rot.get_heat_capacity(t) / (4.184 * 2.981) - 1.0).abs() < 1e-3);
    }

    #[test]
    fn test_oxygen_modes() {
        let t = 298.15;
        let trans = Translation::new(0.03199);
        let rot = RigidRotor::new(true, vec![1.9271e-46], 2);
        let vib = HarmonicOscillator::new(vec![1637.9]);

        assert!((trans.get_partition_function(t) / 1.01325 / 7.11169e6 - 1.0).abs() < 1e-3);
        assert!((rot.get_partition_function(t) / 7.13316e1 - 1.0).abs() < 1e-3);
        assert!((vib.get_partition_function(t) / 1.000037e0 - 1.0).abs() < 1e-3);
    }
}

pub struct StatesModel {
    pub modes: Vec<Box<dyn Mode>>,
    pub spin_multiplicity: i32,
}

impl StatesModel {
    pub fn new(modes: Vec<Box<dyn Mode>>, spin_multiplicity: i32) -> Self {
        StatesModel {
            modes,
            spin_multiplicity,
        }
    }

    pub fn get_partition_function(&self, t: f64) -> f64 {
        let mut q = 1.0;
        for mode in &self.modes {
            q *= mode.get_partition_function(t);
        }
        q * self.spin_multiplicity as f64
    }

    pub fn get_heat_capacity(&self, t: f64) -> f64 {
        let mut cp = constants::R;
        for mode in &self.modes {
            cp += mode.get_heat_capacity(t);
        }
        cp
    }

    pub fn get_enthalpy(&self, t: f64) -> f64 {
        let mut h = constants::R * t;
        for mode in &self.modes {
            h += mode.get_enthalpy(t);
        }
        h
    }

    pub fn get_entropy(&self, t: f64) -> f64 {
        let mut s = 0.0;
        for mode in &self.modes {
            s += mode.get_entropy(t);
        }
        s
    }
}
