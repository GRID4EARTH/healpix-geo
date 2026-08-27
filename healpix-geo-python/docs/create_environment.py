#!/usr/bin/env python3
import pathlib


def main():
    docs_root = pathlib.Path(__file__).parent

    library_root = docs_root.parent
    project_root = library_root.parent.parent

    local_repo = project_root / "target/conda"
    package_root = local_repo / "emscripten-wasm32"

    package_paths = list(package_root.glob("*.tar.bz2"))
    if len(package_paths) == 0:
        raise ValueError("no built package found. Run `pixi run build-emscripten`.")
    elif len(package_paths) > 1:
        print("multiple packages found. Taking the first.")

    package_path = package_paths[0]
    print(f"Selecting {str(package_path)!r}.")

    template_path = docs_root / "environment.template.yml"
    env_path = docs_root / "environment.yml"

    template = template_path.read_text()
    # environment = template.replace("{{ package-path }}", str(package_path))
    environment = template.replace("{{ local-repo }}", str(local_repo))

    env_path.write_text(environment)


if __name__ == "__main__":
    main()
