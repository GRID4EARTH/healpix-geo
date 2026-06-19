# -- Library configuration ---------------------------------------------------

import matplotlib

# ignore matplotlib warnings
matplotlib.set_loglevel("critical")

# -- Generate tables ---------------------------------------------------------

import pathlib
import subprocess


def run_script(name, outpath):
    status = subprocess.run(["python", name, outpath], stderr=subprocess.PIPE)
    if status.returncode != 0:
        raise RuntimeError(f"script {name} failed to run:\n{status.stderr.decode()}")


script_root = pathlib.Path("scripts").absolute()
docs_root = pathlib.Path.cwd()
scripts = [
    ("generate_healpix_levels_table.py", "healpix/healpix_levels_table.md"),
    ("generate_latitude_diff_table.py", "latitude_diff_table.md"),
    ("generate_latitude_graphic.py", "ellipsoidal_latitudes.png"),
]
for name, outpath in scripts:
    run_script(script_root.joinpath(name), docs_root.joinpath(outpath))

# -- Project information -----------------------------------------------------

project = "healpix-geo"
year = "2025"
author = "grid4earth project"

copyright = f"{year}, {author}"

# root toctree document
root_doc = "index"

# -- General configuration ---------------------------------------------------

# enabled extensions
extensions = [
    "sphinx.ext.autosummary",
    "sphinx.ext.autodoc",
    "sphinx.ext.napoleon",
    "sphinx.ext.intersphinx",
    "sphinx_design",
    "jupyterlite_sphinx",
    "myst_nb",
]


templates_path = ["_templates"]
exclude_patterns = [
    "_build",
    "Thumbs.db",
    ".DS_Store",
    ".ipynb_checkpoints",
    "jupyter_execute",
]

# -- intersphinx -------------------------------------------------------------

intersphinx_mapping = {
    "matplotlib": ("https://matplotlib.org/stable/", None),
    "numpy": ("https://numpy.org/doc/stable", None),
    "python": ("https://docs.python.org/3/", None),
    "cdshealpix": ("https://cds-astro.github.io/cds-healpix-python/", None),
    "lonboard": ("https://developmentseed.org/lonboard/latest/", None),
    "shapely": ("https://shapely.readthedocs.io/en/stable", None),
}

# -- autosummary / autodoc ---------------------------------------------------

autosummary_generate = True
autodoc_typehints = "none"

# -- napoleon ----------------------------------------------------------------

napoleon_numpy_docstring = True
napoleon_use_param = False
napoleon_use_rtype = False
napoleon_preprocess_types = True
napoleon_type_aliases = {
    # healpix-geo
    "ellipsoid-like": ":term:`ellipsoid-like`",
    "array-like": ":py:class:`numpy.ndarray`",
}

# -- jupyterlite-sphinx ------------------------------------------------------

global_enable_try_examples = True
try_examples_global_button_text = "Try it in your browser!"
try_examples_global_warning_text = (
    "Interactive examples are experimental and may not always work as expected."
)

# unsilence to see the error message of jupyter lite build
jupyterlite_silence = False

# -- myst-parser -------------------------------------------------------------

myst_enable_extensions = [
    "dollarmath",
    "colon_fence",
]

# Execute notebooks during build
nb_execution_mode = "auto"

# Cache execution results to speed up rebuilds
nb_execution_cache_path = "_build/.jupyter_cache"

# Raise error if notebook execution fails
nb_execution_raise_on_error = True

# Execution timeout in seconds
nb_execution_timeout = 120

# -- Options for HTML output -------------------------------------------------

html_theme = "pydata_sphinx_theme"
html_static_path = ["_static"]
html_css_files = ["css/custom.css"]
html_theme_options = {
    "github_url": "https://github.com/GRID4EARTH/healpix-geo",
    "icon_links_label": "Quick Links",
}
