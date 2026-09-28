#!/usr/bin/env python3
"""Prepare a reviewed unsigned catalogue and target files outside Git for TUF signing."""
import argparse
import hashlib
import json
from pathlib import Path
import shutil

ROOT = Path(__file__).resolve().parents[1]
MAX_WINDOWS_BYTES = 32 * 1024 * 1024


def digest(path: Path) -> tuple[str, int]:
    hasher = hashlib.sha256()
    size = 0
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            hasher.update(block)
            size += len(block)
    return hasher.hexdigest(), size


def prepare(pins: Path, windows_cache: Path, fixture_receipt: Path, fixture_dir: Path,
            output: Path, package_version: str = "1.0.0") -> dict:
    output = output.resolve()
    if output == ROOT or ROOT in output.parents:
        raise ValueError("signing input must be outside the ForgeStore repository")
    targets = output / "targets"
    targets.mkdir(parents=True, exist_ok=False)
    pin_data = json.loads(pins.read_text(encoding="utf-8"))
    receipt = json.loads(fixture_receipt.read_text(encoding="utf-8"))
    entries = []
    names = {
        "7zip": ("7-Zip", "7-Zip", "压缩与解压文件", "File archiver", "Igor Pavlov", "https://www.7-zip.org/"),
        "notepad-plus-plus": ("Notepad++", "Notepad++", "文本与代码编辑器", "Text and code editor", "Notepad++", "https://notepad-plus-plus.org/"),
        "sumatrapdf": ("SumatraPDF", "SumatraPDF", "轻量文档阅读器", "Lightweight document reader", "SumatraPDF", "https://www.sumatrapdfreader.org/"),
    }
    for pin in pin_data["applications"]:
        app_id = pin["id"]
        zh_name, en_name, zh_summary, en_summary, publisher, origin = names[app_id]
        source = windows_cache / pin["file"]
        sha, size = digest(source)
        if sha != pin["sha256"] or not 0 < size <= MAX_WINDOWS_BYTES:
            raise ValueError(f"reviewed Windows installer changed: {pin['file']}")
        shutil.copyfile(source, targets / pin["file"])
        entries.append({"id": app_id, "name": {"zhCN": zh_name, "en": en_name},
                        "summary": {"zhCN": zh_summary, "en": en_summary},
                        "publisher": publisher, "license": pin["license"], "version": pin["version"],
                        "origin": origin, "permissions": ["user-files"],
                        "compatibility": {"status": "tested", "evidence": "2026-09-28-real-vm-gui"},
                        "delivery": {"backend": "compatforge", "reviewedApplicationId": app_id,
                                     "artifact": {"target": pin["file"], "url": pin["url"],
                                                  "sha256": sha, "size": size}}})
    entries.append({"id": "org.forgeos.storefixture-flatpak",
                    "name": {"zhCN": "Flatpak 示例", "en": "Flatpak Fixture"},
                    "summary": {"zhCN": "本地签名仓库验证程序", "en": "Signed local repository test application"},
                    "publisher": "ForgeOS", "license": "MIT", "version": package_version,
                    "origin": "https://flatpak.org/", "permissions": [],
                    "compatibility": {"status": "unknown", "evidence": None},
                    "delivery": {"backend": "flatpak", "remote": "forge-store-fixture",
                                 "reference": "app/org.forgeos.StoreFixture/x86_64/stable"}})
    fixture = receipt["packages"][package_version]
    package = fixture_dir / fixture["file"]
    sha, size = digest(package)
    if sha != fixture["sha256"] or size != fixture["size"]:
        raise ValueError("ForgeOS package fixture changed")
    shutil.copyfile(package, targets / fixture["file"])
    entries.append({"id": "org.forgeos.storefixture",
                    "name": {"zhCN": "ForgeOS 包示例", "en": "ForgeOS Package Fixture"},
                    "summary": {"zhCN": "可回退的原生示例应用", "en": "Rollback capable native test application"},
                    "publisher": "ForgeOS", "license": "MIT", "version": package_version,
                    "origin": "https://github.com/lcxinc/ForgeOS", "permissions": [],
                    "compatibility": {"status": "unknown", "evidence": None},
                    "delivery": {"backend": "forge-package",
                                 "artifact": {"target": fixture["file"],
                                              "url": f"file:///usr/share/forge-store/fixtures/{sha}.forgepkg",
                                              "sha256": sha, "size": size}}})
    catalog = {"schemaVersion": 1, "entries": entries}
    (targets / "catalogue.json").write_text(json.dumps(catalog, ensure_ascii=False, sort_keys=True,
                                                        separators=(",", ":")) + "\n", encoding="utf-8")
    (output / "source-receipt.json").write_text(json.dumps({"schemaVersion": 1,
        "pins": str(pins), "fixtureReceipt": str(fixture_receipt),
        "targets": {path.name: {"sha256": digest(path)[0], "size": path.stat().st_size}
                    for path in sorted(targets.iterdir())}}, indent=2) + "\n", encoding="utf-8")
    return catalog


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--pins", required=True, type=Path)
    parser.add_argument("--windows-cache", required=True, type=Path)
    parser.add_argument("--fixture-receipt", required=True, type=Path)
    parser.add_argument("--fixture-dir", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--package-version", default="1.0.0")
    args = parser.parse_args()
    prepare(args.pins, args.windows_cache, args.fixture_receipt, args.fixture_dir,
            args.output, args.package_version)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
