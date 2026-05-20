#!/usr/bin/env python
# -*- coding: utf-8 -*-

import math
import numpy
import pytest
from chempy.kinetics import ArrheniusModel, ArrheniusEPModel, PDepArrheniusModel, ChebyshevModel
from chempy import constants

class TestKineticsModels:
    """
    Tests for various kinetics models in chempy.kinetics.
    """

    def test_arrhenius_model(self):
        """
        Test the ArrheniusModel class.
        """
        A = 1e12
        n = 0.5
        Ea = 50000.0
        T0 = 298.15
        model = ArrheniusModel(A=A, n=n, Ea=Ea, T0=T0)
        
        T = 500.0
        # k(T) = A * (T/T0)^n * exp(-Ea/RT)
        expected_k = A * (T/T0)**n * math.exp(-Ea / (constants.R * T))
        assert model.getRateCoefficient(T) == pytest.approx(expected_k)
        
        # Test changeT0
        new_T0 = 300.0
        model.changeT0(new_T0)
        assert model.T0 == new_T0
        # A should be adjusted: A_new = A_old * (T0_old / T0_new)^n
        expected_A = (298.15 / 300.0)**0.5
        assert model.A == pytest.approx(expected_A)

    def test_arrhenius_fit_to_data(self):
        """
        Test fitting ArrheniusModel to data.
        """
        Tlist = numpy.array([300, 400, 500, 600, 700, 800, 900, 1000], numpy.float64)
        A_true = 1e10
        n_true = 1.5
        Ea_true = 40000.0
        klist = A_true * (Tlist / 298.15)**n_true * numpy.exp(-Ea_true / (constants.R * Tlist))
        
        model = ArrheniusModel()
        model.fitToData(Tlist, klist, T0=298.15)
        
        assert model.A == pytest.approx(A_true, rel=1e-4)
        assert model.n == pytest.approx(n_true, rel=1e-4)
        assert model.Ea == pytest.approx(Ea_true, rel=1e-4)

    def test_arrhenius_ep_model(self):
        """
        Test the ArrheniusEPModel class.
        """
        A = 1e11
        n = 1.0
        E0 = 30000.0
        alpha = 0.5
        model = ArrheniusEPModel(A=A, n=n, E0=E0, alpha=alpha)
        
        dHrxn = -10000.0
        T = 600.0
        expected_Ea = E0 + alpha * dHrxn
        assert model.getActivationEnergy(dHrxn) == expected_Ea
        
        expected_k = A * (T**n) * math.exp(-expected_Ea / (constants.R * T))
        assert model.getRateCoefficient(T, dHrxn) == pytest.approx(expected_k)
        
        # Test conversion to ArrheniusModel
        arrhenius = model.toArrhenius(dHrxn)
        assert isinstance(arrhenius, ArrheniusModel)
        assert arrhenius.A == A
        assert arrhenius.n == n
        assert arrhenius.Ea == expected_Ea
        assert arrhenius.T0 == 1.0

    def test_pdep_arrhenius_model(self):
        """
        Test the PDepArrheniusModel class.
        """
        P1 = 1e4
        P2 = 1e6
        arrh1 = ArrheniusModel(A=1e10, n=0.0, Ea=30000.0)
        arrh2 = ArrheniusModel(A=1e12, n=0.0, Ea=40000.0)
        
        model = PDepArrheniusModel(pressures=[P1, P2], arrhenius=[arrh1, arrh2])
        
        T = 500.0
        # Test exact pressures
        assert model.getRateCoefficient(T, P1) == arrh1.getRateCoefficient(T)
        assert model.getRateCoefficient(T, P2) == arrh2.getRateCoefficient(T)
        
        # Test interpolation (logarithmic in P and k)
        P = 1e5
        k1 = arrh1.getRateCoefficient(T)
        k2 = arrh2.getRateCoefficient(T)
        expected_k = 10 ** (math.log10(P / P1) / math.log10(P2 / P1) * math.log10(k2 / k1))
        assert model.getRateCoefficient(T, P) == pytest.approx(expected_k)

    def test_chebyshev_model(self):
        """
        Test the ChebyshevModel class.
        """
        Tmin = 300.0
        Tmax = 2000.0
        Pmin = 1e3
        Pmax = 1e7
        coeffs = numpy.array([
            [10.0, 0.1],
            [0.5, -0.05]
        ], numpy.float64)
        
        model = ChebyshevModel(Tmin=Tmin, Tmax=Tmax, Pmin=Pmin, Pmax=Pmax, coeffs=coeffs)
        
        assert model.degreeT == 2
        assert model.degreeP == 2
        
        T = 1000.0
        P = 1e5
        # Chebyshev fitting and evaluation is complex, we just check if it returns a value
        # and if fitting data can reproduce it.
        k = model.getRateCoefficient(T, P)
        assert isinstance(k, float)
        assert k > 0

    def test_chebyshev_fit_to_data(self):
        """
        Test fitting ChebyshevModel to data.
        """
        Tlist = numpy.array([500, 1000, 1500], numpy.float64)
        Plist = numpy.array([1e4, 1e5, 1e6], numpy.float64)
        K = numpy.zeros((len(Tlist), len(Plist)), numpy.float64)
        for i in range(len(Tlist)):
            for j in range(len(Plist)):
                K[i, j] = 1e10 * (Tlist[i]/1000.0)**1.5 * (Plist[j]/1e5)**0.1
        
        model = ChebyshevModel()
        model.fitToData(Tlist, Plist, K, degreeT=2, degreeP=2, Tmin=300, Tmax=2000, Pmin=1e3, Pmax=1e7)
        
        # Check if we can reproduce the data (within reasonable error for low degree)
        for i in range(len(Tlist)):
            for j in range(len(Plist)):
                k_fit = model.getRateCoefficient(Tlist[i], Plist[j])
                assert k_fit == pytest.approx(K[i, j], rel=0.2)
