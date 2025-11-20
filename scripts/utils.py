import fnmatch
import os
import platform
import sys
from pathlib import Path

import tomllib


def get_data_dir():
    if platform.system() == "Windows":
        return Path(os.environ["APPDATA"])
    elif platform.system() == "Darwin":
        return Path.home() / "Library/Application Support"
    elif platform.system() == "Linux":
        return Path.home() / ".local/share"
    else:
        raise NotImplementedError(f"Unsupported platform: {platform.system()}")


def resolve_target(target: str, data_dir: Path) -> Path:
    if target == "@local":
        return data_dir / "typst/packages/local"
    elif target == "@preview":
        return data_dir / "typst/packages/preview"
    else:
        return Path(target)


def read_typst_toml(root_path: Path):
    """Extrae el nombre y version del paquete del archivo typst.toml"""
    toml_path = root_path / "typst.toml"
    if not toml_path.exists():
        print(f"ERROR: typst.toml no encontrado en el directorio {root_path}")
        sys.exit(1)
    name = None
    version = None

    with open(toml_path, "rb") as f:
        data = tomllib.load(f)
        name = data["package"]["name"]
        version = data["package"]["version"]

    if not name or not version:
        print(f"ERROR: nombre o version no encontrados en {toml_path}")
        sys.exit(1)
    return name, version


def load_ignores(root_path: Path):
    """Carga las reglas de .typstignore"""
    ignore_path = root_path / ".typstignore"
    ignores = []

    if ignore_path.exists():
        with open(ignore_path, "r", encoding="utf-8") as f:
            for line in f:
                line = line.strip()
                if line and not line.startswith("#"):
                    ignores.append(line)

    return ignores


def should_include(path, root_path, ignores):
    """
    Determina si un archivo debe incluirse basándose en reglas estilo .gitignore.
    """
    # Obtenemos la ruta relativa (ej: 'carpeta/archivo.txt')
    # as_posix() asegura que use '/' incluso en Windows para compatibilidad con fnmatch
    rel_path = path.relative_to(root_path).as_posix()

    # Por defecto incluimos todo
    include = True

    # Si no hay reglas, incluimos todo
    if not ignores:
        return True

    # Procesamos las reglas en orden.
    # La última regla que coincida es la que decide (esto permite que ! funcione).
    for pattern in ignores:
        is_negation = pattern.startswith("!")
        if is_negation:
            rule = pattern[1:]
        else:
            rule = pattern

        # Ajuste para directorios: si la regla termina en /, solo coincide con directorios
        # (Aquí simplificamos y quitamos la barra final para coincidir el nombre)
        if rule.endswith("/"):
            rule = rule[:-1]

        # Lógica de coincidencia estilo git
        match = False

        if "/" in rule:
            # Si la regla tiene barra (ej: "src/tmp"), coincide con la ruta relativa exacta
            # o con un patrón glob específico
            if rule.startswith("/"):
                rule = rule[1:]  # Quitamos la barra inicial para fnmatch relativo
            match = fnmatch.fnmatch(rel_path, rule)
        else:
            # Si NO tiene barra (ej: "*.pdf"), coincide en cualquier nivel de profundidad
            # Coincide si el nombre del archivo coincide O si algún componente de la ruta coincide
            match = (
                fnmatch.fnmatch(path.name, rule)
                or fnmatch.fnmatch(rel_path, rule)
                or fnmatch.fnmatch(rel_path, f"*/{rule}")
            )

        # Aplicar la decisión si hubo coincidencia
        if match:
            if is_negation:
                include = True  # "!" re-incluye el archivo
            else:
                include = False  # Regla normal excluye el archivo

    return include
