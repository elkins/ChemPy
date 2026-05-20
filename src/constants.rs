//! Physical constants used throughout ChemPy.
//! All constants are in SI units (m, s, kg, mol, etc.).
//! Values are taken from NIST.

use std::f64::consts::PI;

/// The Avogadro constant (particles/mol)
pub const NA: f64 = 6.02214179e23;

/// The Boltzmann constant (J/K)
pub const KB: f64 = 1.3806504e-23;

/// The gas law constant (J/(mol·K))
pub const R: f64 = 8.314472;

/// The Planck constant (J·s)
pub const H: f64 = 6.62606896e-34;

/// The speed of light in a vacuum (m/s)
pub const C: f64 = 299792458.0;

/// pi (dimensionless)
pub const PI_CONST: f64 = PI;
