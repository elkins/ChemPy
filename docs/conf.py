# Project configuration file for Sphinx documentation builder
# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/config.html

import os
import sys

# Add the project source directory to path
sys.path.insert(0, os.path.abspath('..'))

# Project information
project = 'ChemPy'
copyright = '2024, Joshua W. Allen'
author = 'Joshua W. Allen'
version = '0.2.0'
release = '0.2.0'

# Extensions
extensions = [
    'sphinx.ext.autodoc',
    'sphinx.ext.doctest',
    'sphinx.ext.intersphinx',
    'sphinx.ext.todo',
    'sphinx.ext.coverage',
    'sphinx.ext.mathjax',
    'sphinx.ext.viewcode',
    'sphinx_rtd_theme',
]

# Add any paths that contain templates
templates_path = ['_templates']

# The suffix of source filenames
source_suffix = '.rst'

# The root document
root_doc = 'index'

# Theme
html_theme = 'sphinx_rtd_theme'
html_theme_options = {
    'display_version': True,
    'sticky_navigation': True,
    'navigation_depth': 4,
}

# HTML output
html_static_path = ['_static']

# Autodoc options
autodoc_default_options = {
    'members': True,
    'member-order': 'bysource',
    'undoc-members': True,
    'show-inheritance': True,
}
