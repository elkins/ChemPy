#!/usr/bin/env python
# -*- coding: utf-8 -*-

import unittest

from gaussianTest import *  # noqa: F403,F401
from geometryTest import *  # noqa: F403,F401
from graphTest import *  # noqa: F403,F401
from moleculeTest import *  # noqa: F403,F401
from reactionTest import *  # noqa: F403,F401
from statesTest import *  # noqa: F403,F401
from thermoTest import *  # noqa: F403,F401

if __name__ == "__main__":
    unittest.main(testRunner=unittest.TextTestRunner(verbosity=2))
