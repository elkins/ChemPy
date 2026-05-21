use crate::constants;

#[derive(Debug, Clone, PartialEq)]
pub struct Geometry {
    pub coordinates: Vec<[f64; 3]>, // m
    pub mass: Vec<f64>,            // kg/mol
}

impl Geometry {
    pub fn new(coordinates: Vec<[f64; 3]>, mass: Vec<f64>) -> Self {
        Geometry { coordinates, mass }
    }

    pub fn get_total_mass(&self, atoms: Option<&[usize]>) -> f64 {
        match atoms {
            Some(indices) => indices.iter().map(|&i| self.mass[i]).sum(),
            None => self.mass.iter().sum(),
        }
    }

    pub fn get_center_of_mass(&self, atoms: Option<&[usize]>) -> [f64; 3] {
        let indices = match atoms {
            Some(indices) => indices.to_vec(),
            None => (0..self.mass.len()).collect(),
        };

        let mut center = [0.0, 0.0, 0.0];
        let mut total_mass = 0.0;

        for &i in &indices {
            let m = self.mass[i];
            let c = self.coordinates[i];
            center[0] += m * c[0];
            center[1] += m * c[1];
            center[2] += m * c[2];
            total_mass += m;
        }

        if total_mass > 0.0 {
            center[0] /= total_mass;
            center[1] /= total_mass;
            center[2] /= total_mass;
        }

        center
    }

    pub fn get_moment_of_inertia_tensor(&self) -> [[f64; 3]; 3] {
        let mut tensor = [[0.0; 3]; 3];
        let center_of_mass = self.get_center_of_mass(None);

        for (i, &coord0) in self.coordinates.iter().enumerate() {
            let mass = self.mass[i] / constants::NA;
            let coord = [
                coord0[0] - center_of_mass[0],
                coord0[1] - center_of_mass[1],
                coord0[2] - center_of_mass[2],
            ];

            tensor[0][0] += mass * (coord[1] * coord[1] + coord[2] * coord[2]);
            tensor[1][1] += mass * (coord[0] * coord[0] + coord[2] * coord[2]);
            tensor[2][2] += mass * (coord[0] * coord[0] + coord[1] * coord[1]);
            tensor[0][1] -= mass * coord[0] * coord[1];
            tensor[0][2] -= mass * coord[0] * coord[2];
            tensor[1][2] -= mass * coord[1] * coord[2];
        }

        tensor[1][0] = tensor[0][1];
        tensor[2][0] = tensor[0][2];
        tensor[2][1] = tensor[1][2];

        tensor
    }

    pub fn get_internal_reduced_moment_of_inertia(&self, pivots: [usize; 2], top1: &[usize]) -> f64 {
        let n_atoms = self.mass.len();

        // Check that exactly one pivot atom is in the specified top
        let pivot0_in_top1 = top1.contains(&pivots[0]);
        let pivot1_in_top1 = top1.contains(&pivots[1]);

        if !pivot0_in_top1 && !pivot1_in_top1 {
            panic!("No pivot atom included in top");
        } else if pivot0_in_top1 && pivot1_in_top1 {
            panic!("Both pivot atoms included in top");
        }

        // Determine atoms in other top
        let top2: Vec<usize> = (0..n_atoms).filter(|i| !top1.contains(i)).collect();

        // Determine centers of mass of each top
        let top1_com = self.get_center_of_mass(Some(top1));
        let top2_com = self.get_center_of_mass(Some(&top2));

        // Determine axis of rotation
        let mut axis = [
            top1_com[0] - top2_com[0],
            top1_com[1] - top2_com[1],
            top1_com[2] - top2_com[2],
        ];
        let axis_norm = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
        if axis_norm > 0.0 {
            axis[0] /= axis_norm;
            axis[1] /= axis_norm;
            axis[2] /= axis_norm;
        }

        // Determine moments of inertia of each top
        let mut i1 = 0.0;
        for &atom in top1 {
            let r1 = [
                self.coordinates[atom][0] - top1_com[0],
                self.coordinates[atom][1] - top1_com[1],
                self.coordinates[atom][2] - top1_com[2],
            ];
            let dot = r1[0] * axis[0] + r1[1] * axis[1] + r1[2] * axis[2];
            let r1_perp = [
                r1[0] - dot * axis[0],
                r1[1] - dot * axis[1],
                r1[2] - dot * axis[2],
            ];
            let r1_perp_norm_sq =
                r1_perp[0] * r1_perp[0] + r1_perp[1] * r1_perp[1] + r1_perp[2] * r1_perp[2];
            i1 += (self.mass[atom] / constants::NA) * r1_perp_norm_sq;
        }

        let mut i2 = 0.0;
        for &atom in &top2 {
            let r2 = [
                self.coordinates[atom][0] - top2_com[0],
                self.coordinates[atom][1] - top2_com[1],
                self.coordinates[atom][2] - top2_com[2],
            ];
            let dot = r2[0] * axis[0] + r2[1] * axis[1] + r2[2] * axis[2];
            let r2_perp = [
                r2[0] - dot * axis[0],
                r2[1] - dot * axis[1],
                r2[2] - dot * axis[2],
            ];
            let r2_perp_norm_sq =
                r2_perp[0] * r2_perp[0] + r2_perp[1] * r2_perp[1] + r2_perp[2] * r2_perp[2];
            i2 += (self.mass[atom] / constants::NA) * r2_perp_norm_sq;
        }

        1.0 / (1.0 / i1 + 1.0 / i2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_center_of_mass() {
        let coordinates = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]];
        let mass = vec![1.0, 1.0];
        let geom = Geometry::new(coordinates, mass);
        let com = geom.get_center_of_mass(None);
        assert_eq!(com, [0.5, 0.0, 0.0]);
    }

    #[test]
    fn test_moment_of_inertia_tensor() {
        // Simple linear molecule H2 at [0,0,0] and [1e-10, 0, 0]
        let coordinates = vec![[0.0, 0.0, 0.0], [1.0e-10, 0.0, 0.0]];
        let mass = vec![1.008, 1.008];
        let geom = Geometry::new(coordinates, mass);
        let tensor = geom.get_moment_of_inertia_tensor();

        // mass_h = 1.008 / Na
        // r = 0.5e-10
        // Ixx = 0
        // Iyy = 2 * mass_h * r^2
        // Izz = 2 * mass_h * r^2
        let mass_h = 1.008 / constants::NA;
        let r = 0.5e-10;
        let i_expected = 2.0 * mass_h * r * r;

        assert_eq!(tensor[0][0], 0.0);
        assert!((tensor[1][1] - i_expected).abs() < 1e-40);
        assert!((tensor[2][2] - i_expected).abs() < 1e-40);
    }
}
