#!/usr/bin/env python3
"""Sign local candidate targets; copy only public TUF metadata and catalogue into source."""
import argparse
import hashlib
from pathlib import Path
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[1]


def sign(tuftool: Path, root_metadata: Path, private_key: Path, input_targets: Path,
         private_output: Path, public_output: Path, version: int, expires: str) -> None:
    private_output = private_output.resolve()
    public_output = public_output.resolve()
    if ROOT in private_output.parents or private_output == ROOT:
        raise ValueError("full signing output must be outside the source repository")
    if ROOT in private_key.resolve().parents or private_key.resolve() == ROOT:
        raise ValueError("private signing key must be outside the source repository")
    if public_output.exists() or private_output.exists():
        raise FileExistsError("choose fresh output directories")
    command = [str(tuftool), "create", "--key", str(private_key), "--outdir", str(private_output),
               "--root", str(root_metadata), "--add-targets", str(input_targets),
               "--snapshot-expires", expires, "--snapshot-version", str(version),
               "--targets-expires", expires, "--targets-version", str(version),
               "--timestamp-expires", expires, "--timestamp-version", str(version)]
    subprocess.run(command, check=True)
    catalogue = input_targets / "catalogue.json"
    digest = hashlib.sha256(catalogue.read_bytes()).hexdigest()
    (public_output / "metadata").mkdir(parents=True)
    (public_output / "targets").mkdir()
    for name in ("1.root.json", f"{version}.targets.json", f"{version}.snapshot.json", "timestamp.json"):
        shutil.copyfile(private_output / "metadata" / name, public_output / "metadata" / name)
    shutil.copyfile(root_metadata, public_output / "trusted-root.json")
    shutil.copyfile(catalogue, public_output / "targets" / f"{digest}.catalogue.json")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("tuftool", "root-metadata", "private-key", "input-targets", "private-output", "public-output"):
        parser.add_argument("--" + name, required=True, type=Path)
    parser.add_argument("--version", required=True, type=int)
    parser.add_argument("--expires", required=True)
    args = parser.parse_args()
    if args.version < 1:
        raise ValueError("TUF role version must be positive")
    sign(args.tuftool, args.root_metadata, args.private_key, args.input_targets,
         args.private_output, args.public_output, args.version, args.expires)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
