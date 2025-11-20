import shutil
import sys
from pathlib import Path

import utils


def main():
    args = sys.argv[1:]
    data_dir = utils.get_data_dir()

    if len(args) < 1 or args[0] == "help":
        print("Uso: python uninstall.py TARGET")
        print("")
        print("Elimina el paquete instalado en '<name>/<version>' dentro de TARGET.")
        print("El nombre y versión se leen automáticamente de 'typst.toml'.")
        print("")
        print(f"Local package prefix: {data_dir / 'packages/local'}")
        print(f"Local preview prefix: {data_dir / 'packages/preview'}")
        sys.exit(1)

    target_arg = args[0]
    root_dir = Path.cwd()
    target_path = utils.resolve_target(target_arg, data_dir)
    pkg_name, pkg_version = utils.read_typst_toml(root_dir)

    target_to_remove = target_path / pkg_name / pkg_version

    print(f"Paquete a remover: {target_to_remove}")

    if not target_to_remove.exists():
        print("El paquete no fue encontrado.")
    else:
        try:
            shutil.rmtree(target_to_remove)
            print("Eliminación exitosa")
        except OSError as e:
            print(f"Eliminación fallida: {e}")
            sys.exit(1)


if __name__ == "__main__":
    main()
