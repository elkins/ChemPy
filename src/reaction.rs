use crate::constants;
use crate::kinetics::KineticsModel;
use crate::species::{Species, TransitionState};
use std::sync::Arc;

pub struct Reaction {
    pub index: i32,
    pub reactants: Vec<Arc<Species>>,
    pub products: Vec<Arc<Species>>,
    pub kinetics: Option<Box<dyn KineticsModel>>,
    pub reversible: bool,
    pub transition_state: Option<TransitionState>,
}

impl Reaction {
    pub fn new(reactants: Vec<Arc<Species>>, products: Vec<Arc<Species>>) -> Self {
        Reaction {
            index: -1,
            reactants,
            products,
            kinetics: None,
            reversible: true,
            transition_state: None,
        }
    }

    pub fn get_enthalpy_of_reaction(&self, t: f64) -> f64 {
        let mut dh_rxn = 0.0;
        for reactant in &self.reactants {
            if let Some(thermo) = &reactant.thermo {
                dh_rxn -= thermo.get_enthalpy(t);
            }
        }
        for product in &self.products {
            if let Some(thermo) = &product.thermo {
                dh_rxn += thermo.get_enthalpy(t);
            }
        }
        dh_rxn
    }

    pub fn get_entropy_of_reaction(&self, t: f64) -> f64 {
        let mut ds_rxn = 0.0;
        for reactant in &self.reactants {
            if let Some(thermo) = &reactant.thermo {
                ds_rxn -= thermo.get_entropy(t);
            }
        }
        for product in &self.products {
            if let Some(thermo) = &product.thermo {
                ds_rxn += thermo.get_entropy(t);
            }
        }
        ds_rxn
    }

    pub fn get_free_energy_of_reaction(&self, t: f64) -> f64 {
        let mut dg_rxn = 0.0;
        for reactant in &self.reactants {
            if let Some(thermo) = &reactant.thermo {
                dg_rxn -= thermo.get_free_energy(t);
            }
        }
        for product in &self.products {
            if let Some(thermo) = &product.thermo {
                dg_rxn += thermo.get_free_energy(t);
            }
        }
        dg_rxn
    }

    pub fn get_equilibrium_constant(&self, t: f64, k_type: &str) -> f64 {
        let dg_rxn = self.get_free_energy_of_reaction(t);
        let mut k = (-dg_rxn / constants::R / t).exp();

        let p0 = 1.0e5;
        match k_type {
            "Kc" => {
                let c0 = p0 / constants::R / t;
                k *= c0.powi(self.products.len() as i32 - self.reactants.len() as i32);
            }
            "Kp" => {
                k *= p0.powi(self.products.len() as i32 - self.reactants.len() as i32);
            }
            _ => {}
        }
        k
    }

    pub fn get_rate_coefficient(&self, t: f64, p: f64) -> f64 {
        if let Some(kinetics) = &self.kinetics {
            kinetics.get_rate_coefficient(t, p)
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::thermo::NASAPolynomial;
    use crate::species::Species;
    use std::sync::Arc;

    #[test]
    fn test_reaction_thermo() {
        // 2 H2 + O2 -> 2 H2O
        let h2_nasa = NASAPolynomial::new(200.0, 1000.0, [3.29812431E+00, 8.24944177E-04, -8.14301529E-07, -9.47543410E-11, 4.13487234E-13, -1.01252083E+03, -3.29405039E+00]);
        let o2_nasa = NASAPolynomial::new(200.0, 1000.0, [3.21293640E+00, 1.12748635E-03, -5.75615047E-07, 1.31387723E-09, -8.76855392E-13, -1.00524902E+03, 3.61111620E+00]);
        let h2o_nasa = NASAPolynomial::new(200.0, 1000.0, [3.38684249E+00, 3.47498246E-03, -6.35469633E-06, 6.96858127E-09, -2.50658847E-12, -3.02081133E+04, 2.59023285E+00]);

        let mut h2 = Species::new("H2"); h2.thermo = Some(Box::new(h2_nasa));
        let mut o2 = Species::new("O2"); o2.thermo = Some(Box::new(o2_nasa));
        let mut h2o = Species::new("H2O"); h2o.thermo = Some(Box::new(h2o_nasa));

        let h2_arc = Arc::new(h2);
        let o2_arc = Arc::new(o2);
        let h2o_arc = Arc::new(h2o);

        let reaction = Reaction::new(
            vec![h2_arc.clone(), h2_arc, o2_arc],
            vec![h2o_arc.clone(), h2o_arc],
        );

        let t = 298.15;
        let dh = reaction.get_enthalpy_of_reaction(t);
        let ds = reaction.get_entropy_of_reaction(t);

        // Expected values for H2 + O2 -> H2O at 298.15 K
        // This is a simple test, values are approximate
        assert!(dh < 0.0); // Exothermic
        assert!(ds < 0.0); // Entropy decreases
    }
}
