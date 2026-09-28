#!/usr/bin/env python3
"""Build two open-source Forge package test artifacts outside Git."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]


def build(tool: Path, output: Path):
    output = output.resolve()
    if output == ROOT or ROOT in output.parents:
        raise ValueError("fixture output must be outside the ForgeStore repository")
    output.mkdir(parents=True, exist_ok=True)
    result = {}
    for label, version in (("v1", "1.0.0"), ("v2", "2.0.0")):
        source = ROOT / "fixtures" / "forge-package" / label / "bin" / "forge-store-fixture"
        payload = output / ("payload-" + version)
        (payload / "bin").mkdir(parents=True, exist_ok=False)
        target = payload / "bin" / "forge-store-fixture"
        shutil.copyfile(source, target)
        os.chmod(target, 0o755)
        shutil.copyfile(ROOT / "fixtures" / "forge-package" / "LICENSE", payload / "LICENSE")
        spec = {"schemaVersion": 1, "id": "org.forgeos.storefixture", "version": version,
                "kind": "linux", "architecture": "x86_64", "entrypoint": "bin/forge-store-fixture",
                "permissions": []}
        manifest = output / ("manifest-" + version + ".json")
        manifest.write_text(json.dumps(spec, sort_keys=True, separators=(",", ":")), encoding="utf-8")
        name = "org.forgeos.storefixture-" + version + ".forgepkg"
        package = output / name
        command = [sys.executable, str(tool), "build", "--manifest", str(manifest),
                   "--payload", str(payload), "--output", str(package)]
        reported = json.loads(subprocess.run(command, check=True, capture_output=True, text=True).stdout)
        digest = hashlib.sha256(package.read_bytes()).hexdigest()
        if reported["sha256"] != digest:
            raise ValueError("Forge package builder reported a different digest")
        result[version] = {"file": name, "sha256": digest, "size": package.stat().st_size,
                           "license": "MIT", "source": str(source.relative_to(ROOT)).replace("\\", "/")}
    receipt = {"schemaVersion": 1, "id": "org.forgeos.storefixture", "packages": result}
    (output / "receipt.json").write_text(json.dumps(receipt, sort_keys=True, separators=(",", ":")) + "\n", encoding="ascii")
    return receipt


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--forge-package-tool", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    build(args.forge_package_tool, args.output)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
