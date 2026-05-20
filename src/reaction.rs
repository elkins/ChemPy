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
