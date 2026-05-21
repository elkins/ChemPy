#!/usr/bin/python
# -*- coding: utf-8 -*-

import unittest
import numpy as np
from chempy.thermo import WilhoitModel, NASAModel, ThermoGAModel
from chempy.ext.thermo_converter import convertWilhoitToNASA, convertGAtoWilhoit
import chempy.constants as constants

class ThermoConverterTest(unittest.TestCase):
    """
    Contains unit tests for the chempy.ext.thermo_converter module.
    """

    def testGAtoWilhoit(self):
        """
        Test the conversion from ThermoGAModel to WilhoitModel.
        """
        # Data for Ethane (roughly)
        Tdata = np.array([300, 400, 500, 600, 800, 1000, 1500], dtype=float)
        Cpdata = np.array([52.4, 65.2, 77.8, 89.1, 107.5, 122.2, 146.4], dtype=float)
        H298 = -84.0 * 1000 # J/mol
        S298 = 229.5 # J/mol*K
        
        ga_model = ThermoGAModel(Tdata=Tdata, Cpdata=Cpdata, H298=H298, S298=S298)
        
        # Ethane: 8 atoms, 1 rotor (C-C), non-linear
        atoms = 8
        rotors = 1
        linear = False
        
        wilhoit = convertGAtoWilhoit(ga_model, atoms, rotors, linear)
        
        self.assertIsInstance(wilhoit, WilhoitModel)
        # Check that Wilhoit reproduces Cp data reasonably well
        for i in range(len(Tdata)):
            cp_w = wilhoit.getHeatCapacity(Tdata[i])
            self.assertAlmostEqual(cp_w / Cpdata[i], 1.0, places=2)
            
        self.assertAlmostEqual(wilhoit.getEnthalpy(298.15) / H298, 1.0, places=3)
        self.assertAlmostEqual(wilhoit.getEntropy(298.15) / S298, 1.0, places=3)

    def testWilhoitToNASA(self):
        """
        Test the conversion from WilhoitModel to NASAModel.
        """
        # Create a dummy Wilhoit model
        wilhoit = WilhoitModel(
            cp0 = 4.0 * constants.R,
            cpInf = 20.0 * constants.R,
            a0 = 0.0,
            a1 = 0.0,
            a2 = 0.0,
            a3 = 0.0,
            H0 = 100000.0,
            S0 = 200.0,
            B = 500.0,
        )
        wilhoit.Tmin = 300.0
        wilhoit.Tmax = 3000.0
        
        nasa = convertWilhoitToNASA(wilhoit, Tmin=300.0, Tmax=3000.0, Tint=1000.0)
        
        self.assertIsInstance(nasa, NASAModel)
        
        # Check values at some temperatures
        # Use a bit more relaxed tolerance for NASA fit as it is an approximation
        for T in [500.0, 1000.0, 1500.0, 2000.0]:
            cp_w = wilhoit.getHeatCapacity(T)
            cp_n = nasa.getHeatCapacity(T)
            self.assertAlmostEqual(cp_w / cp_n, 1.0, places=2)
            
            h_w = wilhoit.getEnthalpy(T)
            h_n = nasa.getEnthalpy(T)
            self.assertAlmostEqual(h_w / h_n, 1.0, places=2)

            s_w = wilhoit.getEntropy(T)
            s_n = nasa.getEntropy(T)
            self.assertAlmostEqual(s_w / s_n, 1.0, places=2)

if __name__ == '__main__':
    unittest.main(testRunner=unittest.TextTestRunner(verbosity=2))
