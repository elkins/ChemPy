#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""
Benchmarks for reaction kinetics calculations.
"""

import pytest

from chempy.kinetics import ArrheniusModel
from chempy.reaction import Reaction
from chempy.species import Species


class TestArrheniusKinetics:
    """Benchmark Arrhenius kinetics calculations."""

    @pytest.fixture(autouse=True)
    def setup(self):
        """Create test kinetics models."""
        # Create Arrhenius kinetics with typical parameters
        self.arrhenius_low = ArrheniusModel(A=1.0e10, n=0.5, Ea=50000.0)
        self.arrhenius_high = ArrheniusModel(A=1.0e13, n=1.0, Ea=100000.0)

        # Temperature range for testing
        self.T_low = 300.0  # K
        self.T_medium = 1000.0  # K
        self.T_high = 2000.0  # K

    def test_rate_coefficient_low_temp(self, benchmark):
        """Benchmark rate coefficient calculation at low temperature."""
        result = benchmark(self.arrhenius_low.getRateCoefficient, self.T_low)
        assert result > 0

    def test_rate_coefficient_medium_temp(self, benchmark):
        """Benchmark rate coefficient calculation at medium temperature."""
        result = benchmark(self.arrhenius_high.getRateCoefficient, self.T_medium)
        assert result > 0

    def test_rate_coefficient_high_temp(self, benchmark):
        """Benchmark rate coefficient calculation at high temperature."""
        result = benchmark(self.arrhenius_high.getRateCoefficient, self.T_high)
        assert result > 0


class TestReactionRate:
    """Benchmark forward reaction rate calculations."""

    @pytest.fixture(autouse=True)
    def setup(self):
        """Create test reaction."""
        # Create a simple A + B -> C reaction with just kinetics
        self.speciesA = Species(label="A")
        self.speciesB = Species(label="B")
        self.speciesC = Species(label="C")

        self.kinetics = ArrheniusModel(A=1.0e10, n=0.5, Ea=50000.0)
        self.reaction = Reaction(
            reactants=[self.speciesA, self.speciesB],
            products=[self.speciesC],
            kinetics=self.kinetics,
        )

        # Concentration conditions
        self.concentrations = {
            self.speciesA: 1.0,  # mol/L
            self.speciesB: 2.0,  # mol/L
            self.speciesC: 0.0,  # mol/L
        }

        self.T = 1000.0  # K
        self.P = 101325.0  # Pa

    def test_forward_rate_calculation(self, benchmark):
        """Benchmark calculating forward rate with concentration products."""

        def calculate_forward_rate():
            # Calculate rate constant
            k = self.kinetics.getRateCoefficient(self.T, self.P)
            # Calculate concentration product
            forward = 1.0
            for reactant in self.reaction.reactants:
                if reactant in self.concentrations:
                    forward *= self.concentrations[reactant]
            return k * forward

        result = benchmark(calculate_forward_rate)
        assert result > 0
