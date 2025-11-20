import os
import shutil
import sys
from pathlib import Path

import utils


def main():
    args = sys.argv[1:]
    data_dir = utils.get_data_dir()

    if len(args) < 1 or args[0] == "help":
        print("Empaqueta los archivos en '<name>/<version>' dentro de TARGET")
        print(
            "Si TARGET es @local o @preview, usa el directorio predeterminado de paquetes de Typst"
        )
        print("El nombre del paquete y version se leen del archivo 'typst.toml'")
        print("")
        print(f"Local package path: {data_dir / 'typst/packages/local'}")
        print(f"Preview package path: {data_dir / 'typst/packages/preview'}")
        sys.exit(1)

    target_arg = args[0]
    root_dir = Path.cwd()
    target_path = utils.resolve_target(target_arg, data_dir)
    pkg_name, pkg_version = utils.read_typst_toml(root_dir)

    final_target = target_path / pkg_name / pkg_version
    ignores = utils.load_ignores(root_dir)
    files_to_copy = []

    for current_root, dirs, files in os.walk(root_dir):
        if ".git" in dirs:
            dirs.remove(".git")
        current_root_path = Path(current_root)

        dirs_to_remove = []
        for d in dirs:
            d_path = current_root_path / d
            if not utils.should_include(d_path, root_dir, ignores):
                dirs_to_remove.append(d)

        for d in dirs_to_remove:
            dirs.remove(d)

        for f in files:
            if f == ".typstignore":
                continue

            full_path = current_root_path / f
            if utils.should_include(full_path, root_dir, ignores):
                files_to_copy.append(full_path)

    print(f"Source: {root_dir}")
    print(f"Destination: {final_target}")

    if final_target.exists():
        print("Sobreescribiendo version existente")
        shutil.rmtree(final_target)

    count = 0
    for src in files_to_copy:
        rel_path = src.relative_to(root_dir)
        dest = final_target / rel_path
        dest.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(src, dest)
        count += 1

    print(f"Copiados {count} archivos")


if __name__ == "__main__":
    main()
