use crate::molecule::Molecule;
use crate::states::StatesModel;
use crate::thermo::ThermoModel;

/// A chemical species.
pub struct Species {
    pub index: i32,
    pub label: String,
    pub thermo: Option<Box<dyn ThermoModel>>,
    pub states: Option<StatesModel>,
    pub molecule: Vec<Molecule>,
    pub e0: f64,
    pub molecular_weight: f64,
    pub reactive: bool,
}

impl Species {
    pub fn new(label: &str) -> Self {
        Species {
            index: -1,
            label: label.to_string(),
            thermo: None,
            states: None,
            molecule: Vec::new(),
            e0: 0.0,
            molecular_weight: 0.0,
            reactive: true,
        }
    }
}

pub struct TransitionState {
    pub label: String,
    pub states: Option<StatesModel>,
    pub e0: f64,
    pub frequency: f64,
    pub degeneracy: i32,
}

impl TransitionState {
    pub fn new(label: &str) -> Self {
        TransitionState {
            label: label.to_string(),
            states: None,
            e0: 0.0,
            frequency: 0.0,
            degeneracy: 1,
        }
    }
}
