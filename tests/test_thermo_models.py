#!/usr/bin/env python
# -*- coding: utf-8 -*-

import numpy
import pytest

from chempy import constants
from chempy.thermo import NASAModel, NASAPolynomial, ThermoError, ThermoGAModel, WilhoitModel


class TestThermoModels:
    """
    Tests for various thermodynamics models in chempy.thermo.
    """

    def test_thermo_ga_model(self):
        """
        Test the ThermoGAModel class.
        """
        Tdata = numpy.array([300.0, 400.0, 500.0, 600.0, 800.0, 1000.0, 1500.0])
        Cpdata = numpy.array([30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0])
        H298 = 100000.0
        S298 = 200.0
        model = ThermoGAModel(Tdata=Tdata, Cpdata=Cpdata, H298=H298, S298=S298, Tmin=298.15, Tmax=2000)

        # Test Heat Capacity interpolation
        assert model.getHeatCapacity(300.0) == 30.0
        assert model.getHeatCapacity(350.0) == pytest.approx(35.0)
        assert model.getHeatCapacity(1000.0) == 80.0

        # Test Enthalpy and Entropy at 298.15 (should be close to H298, S298 if Tdata starts at 300)
        # Note: ThermoGAModel.getEnthalpy starts from H298 and integrates.
        # If T < Tdata[0], it uses Cpdata[0].
        # Let's check the code:
        # H = self.H298
        # for Tmin, Tmax, Cpmin, Cpmax in zip(self.Tdata[:-1], self.Tdata[1:], self.Cpdata[:-1], self.Cpdata[1:]):
        #     if T > Tmin: ...
        # if T > self.Tdata[-1]: H += self.Cpdata[-1] * (T - self.Tdata[-1])
        # So for T=298.15, H = H298.
        assert model.getEnthalpy(298.15) == H298
        assert model.getEntropy(298.15) == S298

        # Test out of bounds
        with pytest.raises(ThermoError):
            model.getHeatCapacity(200.0)

    def test_thermo_ga_model_add(self):
        """
        Test addition of ThermoGAModel objects.
        """
        Tdata = numpy.array([300.0, 400.0, 500.0])
        model1 = ThermoGAModel(Tdata=Tdata, Cpdata=numpy.array([10.0, 20.0, 30.0]), H298=1000.0, S298=10.0)
        model2 = ThermoGAModel(Tdata=Tdata, Cpdata=numpy.array([5.0, 5.0, 5.0]), H298=500.0, S298=5.0)

        model3 = model1 + model2
        assert numpy.all(model3.Cpdata == numpy.array([15.0, 25.0, 35.0]))
        assert model3.H298 == 1500.0
        assert model3.S298 == 15.0

    def test_wilhoit_model(self):
        """
        Test the WilhoitModel class.
        """
        cp0 = 3.5 * constants.R
        cpInf = 10.0 * constants.R
        a0, a1, a2, a3 = 0.1, 0.2, 0.3, 0.4
        H0 = 10000.0
        S0 = 100.0
        B = 500.0
        model = WilhoitModel(cp0=cp0, cpInf=cpInf, a0=a0, a1=a1, a2=a2, a3=a3, H0=H0, S0=S0, B=B)

        T = 500.0
        Cp = model.getHeatCapacity(T)
        assert isinstance(Cp, float)

        H = model.getEnthalpy(T)
        S = model.getEntropy(T)
        G = model.getFreeEnergy(T)
        assert G == pytest.approx(H - T * S)

    def test_wilhoit_fit_to_data(self):
        """
        Test fitting WilhoitModel to data.
        """
        Tlist = numpy.array([300, 400, 500, 600, 800, 1000, 1500], numpy.float64)
        Cplist = numpy.array([30, 40, 50, 60, 70, 80, 90], numpy.float64)
        H298 = 100000.0
        S298 = 200.0

        model = WilhoitModel()
        # nFreq = (3*N - 6) or similar. Let's just use some values.
        # cpInf = cp0 + (nFreq + 0.5 * nRotors) * R
        # for linear=False, cp0 = 4R.
        model.fitToDataForConstantB(Tlist, Cplist, linear=False, nFreq=10, nRotors=2, B=500.0, H298=H298, S298=S298)

        assert model.cp0 == 4.0 * constants.R
        assert model.cpInf == (4.0 + 10 + 1.0) * constants.R
        assert model.getEnthalpy(298.15) == pytest.approx(H298)
        assert model.getEntropy(298.15) == pytest.approx(S298)

    def test_nasa_polynomial(self):
        """
        Test the NASAPolynomial class.
        """
        # Example coefficients (from some real species or arbitrary)
        coeffs = [3.5, 1e-3, 1e-6, 1e-9, 1e-12, 1000.0, 10.0]
        model = NASAPolynomial(Tmin=300, Tmax=1000, coeffs=coeffs)

        T = 500.0
        Cp = model.getHeatCapacity(T)
        # Cp/R = a1 + a2 T + a3 T^2 + a4 T^3 + a5 T^4
        expected_Cp_over_R = coeffs[0] + coeffs[1] * T + coeffs[2] * T**2 + coeffs[3] * T**3 + coeffs[4] * T**4
        assert Cp == pytest.approx(expected_Cp_over_R * constants.R)

        H = model.getEnthalpy(T)
        S = model.getEntropy(T)
        G = model.getFreeEnergy(T)
        assert G == pytest.approx(H - T * S)

    def test_nasa_model(self):
        """
        Test the NASAModel class (multi-polynomial).
        """
        poly1 = NASAPolynomial(Tmin=300, Tmax=1000, coeffs=[3.5, 0, 0, 0, 0, 1000, 10])
        poly2 = NASAPolynomial(Tmin=1000, Tmax=3000, coeffs=[4.5, 0, 0, 0, 0, 2000, 20])
        model = NASAModel(polynomials=[poly1, poly2], Tmin=300, Tmax=3000)

        assert model.getHeatCapacity(500.0) == poly1.getHeatCapacity(500.0)
        assert model.getHeatCapacity(2000.0) == poly2.getHeatCapacity(2000.0)

        with pytest.raises(ThermoError):
            model.getHeatCapacity(200.0)
