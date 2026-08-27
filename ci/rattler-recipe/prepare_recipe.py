#!/usr/bin/env python

import pathlib
import textwrap

import packaging.version
import tomllib


def dev_version(most_recent_release):
    v = packaging.version.parse(most_recent_release)

    next_version = (v.major, v.minor, v.micro + 1)
    return str(v.__replace__(release=next_version, dev=0))


def extract_version(cargo_data):
    parsed = tomllib.loads(cargo_data)

    return packaging.version.parse(parsed["package"]["version"])


def main():
    root = pathlib.Path.cwd()
    cargo_config_path = root / "healpix-geo-python" / "Cargo.toml"
    version = extract_version(cargo_config_path.read_text()).__replace__(dev=0)

    recipe_root = root / "ci/rattler-recipe"

    template_path = recipe_root / "recipe_template.yaml"
    template = template_path.read_text()

    context = textwrap.dedent(f"""\
    context:
      name: healpix-geo
      version: {version}
      path: {root}
    """.rstrip())
    recipe = "\n".join([context, "", template])
    recipe_path = recipe_root / "recipe.yaml"
    recipe_path.write_text(recipe)


if __name__ == "__main__":
    main()
