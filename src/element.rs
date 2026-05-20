use std::fmt;

/// A chemical element.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Element {
    /// The atomic number of the element
    pub number: u16,
    /// The symbol used for the element
    pub symbol: &'static str,
    /// The IUPAC name of the element
    pub name: &'static str,
    /// The mass of the element in kg/mol
    pub mass: f64,
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol)
    }
}

/// Return the `Element` object with attributes defined by the given parameters.
/// Only the parameters explicitly given will be used.
pub fn get_element(number: u16, symbol: &str) -> Option<&'static Element> {
    ELEMENT_LIST
        .iter()
        .find(|e| (number == 0 || e.number == number) && (symbol.is_empty() || e.symbol == symbol))
}

// Period 1
pub const H: Element = Element {
    number: 1,
    symbol: "H",
    name: "hydrogen",
    mass: 0.00100794,
};
pub const HE: Element = Element {
    number: 2,
    symbol: "He",
    name: "helium",
    mass: 0.004002602,
};

// Period 2
pub const LI: Element = Element {
    number: 3,
    symbol: "Li",
    name: "lithium",
    mass: 0.006941,
};
pub const BE: Element = Element {
    number: 4,
    symbol: "Be",
    name: "beryllium",
    mass: 0.009012182,
};
pub const B: Element = Element {
    number: 5,
    symbol: "B",
    name: "boron",
    mass: 0.010811,
};
pub const C: Element = Element {
    number: 6,
    symbol: "C",
    name: "carbon",
    mass: 0.0120107,
};
pub const N: Element = Element {
    number: 7,
    symbol: "N",
    name: "nitrogen",
    mass: 0.01400674,
};
pub const O: Element = Element {
    number: 8,
    symbol: "O",
    name: "oxygen",
    mass: 0.0159994,
};
pub const F: Element = Element {
    number: 9,
    symbol: "F",
    name: "fluorine",
    mass: 0.018998403,
};
pub const NE: Element = Element {
    number: 10,
    symbol: "Ne",
    name: "neon",
    mass: 0.0201797,
};

// Period 3
pub const NA: Element = Element {
    number: 11,
    symbol: "Na",
    name: "sodium",
    mass: 0.022989770,
};
pub const MG: Element = Element {
    number: 12,
    symbol: "Mg",
    name: "magnesium",
    mass: 0.0243050,
};
pub const AL: Element = Element {
    number: 13,
    symbol: "Al",
    name: "aluminium",
    mass: 0.026981538,
};
pub const SI: Element = Element {
    number: 14,
    symbol: "Si",
    name: "silicon",
    mass: 0.0280855,
};
pub const P: Element = Element {
    number: 15,
    symbol: "P",
    name: "phosphorus",
    mass: 0.030973761,
};
pub const S: Element = Element {
    number: 16,
    symbol: "S",
    name: "sulfur",
    mass: 0.032065,
};
pub const CL: Element = Element {
    number: 17,
    symbol: "Cl",
    name: "chlorine",
    mass: 0.035453,
};
pub const AR: Element = Element {
    number: 18,
    symbol: "Ar",
    name: "argon",
    mass: 0.039348,
};

// Period 4
pub const K: Element = Element {
    number: 19,
    symbol: "K",
    name: "potassium",
    mass: 0.0390983,
};
pub const CA: Element = Element {
    number: 20,
    symbol: "Ca",
    name: "calcium",
    mass: 0.040078,
};
pub const SC: Element = Element {
    number: 21,
    symbol: "Sc",
    name: "scandium",
    mass: 0.044955910,
};
pub const TI: Element = Element {
    number: 22,
    symbol: "Ti",
    name: "titanium",
    mass: 0.047867,
};
pub const V: Element = Element {
    number: 23,
    symbol: "V",
    name: "vanadium",
    mass: 0.0509415,
};
pub const CR: Element = Element {
    number: 24,
    symbol: "Cr",
    name: "chromium",
    mass: 0.0519961,
};
pub const MN: Element = Element {
    number: 25,
    symbol: "Mn",
    name: "manganese",
    mass: 0.054938049,
};
pub const FE: Element = Element {
    number: 26,
    symbol: "Fe",
    name: "iron",
    mass: 0.055845,
};
pub const CO: Element = Element {
    number: 27,
    symbol: "Co",
    name: "cobalt",
    mass: 0.058933200,
};
pub const NI: Element = Element {
    number: 28,
    symbol: "Ni",
    name: "nickel",
    mass: 0.0586934,
};
pub const CU: Element = Element {
    number: 29,
    symbol: "Cu",
    name: "copper",
    mass: 0.063546,
};
pub const ZN: Element = Element {
    number: 30,
    symbol: "Zn",
    name: "zinc",
    mass: 0.065409,
};
pub const GA: Element = Element {
    number: 31,
    symbol: "Ga",
    name: "gallium",
    mass: 0.069723,
};
pub const GE: Element = Element {
    number: 32,
    symbol: "Ge",
    name: "germanium",
    mass: 0.07264,
};
pub const AS: Element = Element {
    number: 33,
    symbol: "As",
    name: "arsenic",
    mass: 0.07492160,
};
pub const SE: Element = Element {
    number: 34,
    symbol: "Se",
    name: "selenium",
    mass: 0.07896,
};
pub const BR: Element = Element {
    number: 35,
    symbol: "Br",
    name: "bromine",
    mass: 0.079904,
};
pub const KR: Element = Element {
    number: 36,
    symbol: "Kr",
    name: "krypton",
    mass: 0.083798,
};

// Period 5
pub const RB: Element = Element {
    number: 37,
    symbol: "Rb",
    name: "rubidium",
    mass: 0.0854678,
};
pub const SR: Element = Element {
    number: 38,
    symbol: "Sr",
    name: "strontium",
    mass: 0.08762,
};
pub const Y: Element = Element {
    number: 39,
    symbol: "Y",
    name: "yttrium",
    mass: 0.08890585,
};
pub const ZR: Element = Element {
    number: 40,
    symbol: "Zr",
    name: "zirconium",
    mass: 0.091224,
};
pub const NB: Element = Element {
    number: 41,
    symbol: "Nb",
    name: "niobium",
    mass: 0.09290638,
};
pub const MO: Element = Element {
    number: 42,
    symbol: "Mo",
    name: "molybdenum",
    mass: 0.09594,
};
pub const TC: Element = Element {
    number: 43,
    symbol: "Tc",
    name: "technetium",
    mass: 0.098,
};
pub const RU: Element = Element {
    number: 44,
    symbol: "Ru",
    name: "ruthenium",
    mass: 0.10107,
};
pub const RH: Element = Element {
    number: 45,
    symbol: "Rh",
    name: "rhodium",
    mass: 0.10290550,
};
pub const PD: Element = Element {
    number: 46,
    symbol: "Pd",
    name: "palladium",
    mass: 0.10642,
};
pub const AG: Element = Element {
    number: 47,
    symbol: "Ag",
    name: "silver",
    mass: 0.1078682,
};
pub const CD: Element = Element {
    number: 48,
    symbol: "Cd",
    name: "cadmium",
    mass: 0.112411,
};
pub const IN: Element = Element {
    number: 49,
    symbol: "In",
    name: "indium",
    mass: 0.114818,
};
pub const SN: Element = Element {
    number: 50,
    symbol: "Sn",
    name: "tin",
    mass: 0.118710,
};
pub const SB: Element = Element {
    number: 51,
    symbol: "Sb",
    name: "antimony",
    mass: 0.121760,
};
pub const TE: Element = Element {
    number: 52,
    symbol: "Te",
    name: "tellurium",
    mass: 0.12760,
};
pub const I: Element = Element {
    number: 53,
    symbol: "I",
    name: "iodine",
    mass: 0.12690447,
};
pub const XE: Element = Element {
    number: 54,
    symbol: "Xe",
    name: "xenon",
    mass: 0.131293,
};

// Period 6
pub const CS: Element = Element {
    number: 55,
    symbol: "Cs",
    name: "caesium",
    mass: 0.13290545,
};
pub const BA: Element = Element {
    number: 56,
    symbol: "Ba",
    name: "barium",
    mass: 0.137327,
};
pub const LA: Element = Element {
    number: 57,
    symbol: "La",
    name: "lanthanum",
    mass: 0.1389055,
};
pub const CE: Element = Element {
    number: 58,
    symbol: "Ce",
    name: "cerium",
    mass: 0.140116,
};
pub const PR: Element = Element {
    number: 59,
    symbol: "Pr",
    name: "praesodymium",
    mass: 0.14090765,
};
pub const ND: Element = Element {
    number: 60,
    symbol: "Nd",
    name: "neodymium",
    mass: 0.14424,
};
pub const PM: Element = Element {
    number: 61,
    symbol: "Pm",
    name: "promethium",
    mass: 0.145,
};
pub const SM: Element = Element {
    number: 62,
    symbol: "Sm",
    name: "samarium",
    mass: 0.15036,
};
pub const EU: Element = Element {
    number: 63,
    symbol: "Eu",
    name: "europium",
    mass: 0.151964,
};
pub const GD: Element = Element {
    number: 64,
    symbol: "Gd",
    name: "gadolinium",
    mass: 0.15725,
};
pub const TB: Element = Element {
    number: 65,
    symbol: "Tb",
    name: "terbium",
    mass: 0.15892534,
};
pub const DY: Element = Element {
    number: 66,
    symbol: "Dy",
    name: "dysprosium",
    mass: 0.162500,
};
pub const HO: Element = Element {
    number: 67,
    symbol: "Ho",
    name: "holmium",
    mass: 0.16493032,
};
pub const ER: Element = Element {
    number: 68,
    symbol: "Er",
    name: "erbium",
    mass: 0.167259,
};
pub const TM: Element = Element {
    number: 69,
    symbol: "Tm",
    name: "thulium",
    mass: 0.16893421,
};
pub const YB: Element = Element {
    number: 70,
    symbol: "Yb",
    name: "ytterbium",
    mass: 0.17304,
};
pub const LU: Element = Element {
    number: 71,
    symbol: "Lu",
    name: "lutetium",
    mass: 0.174967,
};
pub const HF: Element = Element {
    number: 72,
    symbol: "Hf",
    name: "hafnium",
    mass: 0.17849,
};
pub const TA: Element = Element {
    number: 73,
    symbol: "Ta",
    name: "tantalum",
    mass: 0.1809479,
};
pub const W: Element = Element {
    number: 74,
    symbol: "W",
    name: "tungsten",
    mass: 0.18384,
};
pub const RE: Element = Element {
    number: 75,
    symbol: "Re",
    name: "rhenium",
    mass: 0.186207,
};
pub const OS: Element = Element {
    number: 76,
    symbol: "Os",
    name: "osmium",
    mass: 0.19023,
};
pub const IR: Element = Element {
    number: 77,
    symbol: "Ir",
    name: "iridium",
    mass: 0.192217,
};
pub const PT: Element = Element {
    number: 78,
    symbol: "Pt",
    name: "platinum",
    mass: 0.195078,
};
pub const AU: Element = Element {
    number: 79,
    symbol: "Au",
    name: "gold",
    mass: 0.19696655,
};
pub const HG: Element = Element {
    number: 80,
    symbol: "Hg",
    name: "mercury",
    mass: 0.20059,
};
pub const TL: Element = Element {
    number: 81,
    symbol: "Tl",
    name: "thallium",
    mass: 0.2043833,
};
pub const PB: Element = Element {
    number: 82,
    symbol: "Pb",
    name: "lead",
    mass: 0.2072,
};
pub const BI: Element = Element {
    number: 83,
    symbol: "Bi",
    name: "bismuth",
    mass: 0.20898038,
};
pub const PO: Element = Element {
    number: 84,
    symbol: "Po",
    name: "polonium",
    mass: 0.209,
};
pub const AT: Element = Element {
    number: 85,
    symbol: "At",
    name: "astatine",
    mass: 0.210,
};
pub const RN: Element = Element {
    number: 86,
    symbol: "Rn",
    name: "radon",
    mass: 0.222,
};

// Period 7
pub const FR: Element = Element {
    number: 87,
    symbol: "Fr",
    name: "francium",
    mass: 0.223,
};
pub const RA: Element = Element {
    number: 88,
    symbol: "Ra",
    name: "radium",
    mass: 0.226,
};
pub const AC: Element = Element {
    number: 89,
    symbol: "Ac",
    name: "actinum",
    mass: 0.227,
};
pub const TH: Element = Element {
    number: 90,
    symbol: "Th",
    name: "thorium",
    mass: 0.2320381,
};
pub const PA: Element = Element {
    number: 91,
    symbol: "Pa",
    name: "protactinum",
    mass: 0.23103588,
};
pub const U: Element = Element {
    number: 92,
    symbol: "U",
    name: "uranium",
    mass: 0.23802891,
};
pub const NP: Element = Element {
    number: 93,
    symbol: "Np",
    name: "neptunium",
    mass: 0.237,
};
pub const PU: Element = Element {
    number: 94,
    symbol: "Pu",
    name: "plutonium",
    mass: 0.244,
};
pub const AM: Element = Element {
    number: 95,
    symbol: "Am",
    name: "americium",
    mass: 0.243,
};
pub const CM: Element = Element {
    number: 96,
    symbol: "Cm",
    name: "curium",
    mass: 0.247,
};
pub const BK: Element = Element {
    number: 97,
    symbol: "Bk",
    name: "berkelium",
    mass: 0.247,
};
pub const CF: Element = Element {
    number: 98,
    symbol: "Cf",
    name: "californium",
    mass: 0.251,
};
pub const ES: Element = Element {
    number: 99,
    symbol: "Es",
    name: "einsteinium",
    mass: 0.252,
};
pub const FM: Element = Element {
    number: 100,
    symbol: "Fm",
    name: "fermium",
    mass: 0.257,
};
pub const MD: Element = Element {
    number: 101,
    symbol: "Md",
    name: "mendelevium",
    mass: 0.258,
};
pub const NO: Element = Element {
    number: 102,
    symbol: "No",
    name: "nobelium",
    mass: 0.259,
};
pub const LR: Element = Element {
    number: 103,
    symbol: "Lr",
    name: "lawrencium",
    mass: 0.262,
};
pub const RF: Element = Element {
    number: 104,
    symbol: "Rf",
    name: "rutherfordium",
    mass: 0.261,
};
pub const DB: Element = Element {
    number: 105,
    symbol: "Db",
    name: "dubnium",
    mass: 0.262,
};
pub const SG: Element = Element {
    number: 106,
    symbol: "Sg",
    name: "seaborgium",
    mass: 0.266,
};
pub const BH: Element = Element {
    number: 107,
    symbol: "Bh",
    name: "bohrium",
    mass: 0.264,
};
pub const HS: Element = Element {
    number: 108,
    symbol: "Hs",
    name: "hassium",
    mass: 0.277,
};
pub const MT: Element = Element {
    number: 109,
    symbol: "Mt",
    name: "meitnerium",
    mass: 0.268,
};
pub const DS: Element = Element {
    number: 110,
    symbol: "Ds",
    name: "darmstadtium",
    mass: 0.281,
};
pub const RG: Element = Element {
    number: 111,
    symbol: "Rg",
    name: "roentgenium",
    mass: 0.272,
};
pub const CN: Element = Element {
    number: 112,
    symbol: "Cn",
    name: "copernicum",
    mass: 0.285,
};

pub const ELEMENT_LIST: [Element; 112] = [
    H, HE, LI, BE, B, C, N, O, F, NE, NA, MG, AL, SI, P, S, CL, AR, K, CA, SC, TI, V, CR, MN, FE,
    CO, NI, CU, ZN, GA, GE, AS, SE, BR, KR, RB, SR, Y, ZR, NB, MO, TC, RU, RH, PD, AG, CD, IN, SN,
    SB, TE, I, XE, CS, BA, LA, CE, PR, ND, PM, SM, EU, GD, TB, DY, HO, ER, TM, YB, LU, HF, TA, W,
    RE, OS, IR, PT, AU, HG, TL, PB, BI, PO, AT, RN, FR, RA, AC, TH, PA, U, NP, PU, AM, CM, BK, CF,
    ES, FM, MD, NO, LR, RF, DB, SG, BH, HS, MT, DS, RG, CN,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_element() {
        let carbon = get_element(6, "").unwrap();
        assert_eq!(carbon.symbol, "C");
        assert_eq!(carbon.name, "carbon");

        let hydrogen = get_element(0, "H").unwrap();
        assert_eq!(hydrogen.number, 1);

        let none = get_element(999, "");
        assert!(none.is_none());
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", C), "C");
    }
}
