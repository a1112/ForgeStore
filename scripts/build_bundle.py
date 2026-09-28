#!/usr/bin/env python3
"""Build a deterministic, source-pinned ForgeStore image bundle outside Git."""
import argparse
import gzip
import hashlib
import io
import json
from pathlib import Path
import tarfile

MAX_FILE = 64 * 1024 * 1024
MAX_TOTAL = 512 * 1024 * 1024
MAX_FILES = 128


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def collect(args) -> dict[str, tuple[Path, str]]:
    files: dict[str, tuple[Path, str]] = {}

    def add(target: str, source: Path, mode: str = "0644") -> None:
        if target in files or not source.is_file() or source.is_symlink():
            raise ValueError(f"missing, linked, or duplicate bundle source: {target}")
        files[target] = (source, mode)

    add("/usr/bin/forge-store-service", args.service_binary, "0755")
    ui = args.ui_stage
    add("/usr/bin/forge-store-ui", ui / "bin/forge-store-ui", "0755")
    add("/usr/share/applications/forge-store.desktop", ui / "share/applications/forge-store.desktop")
    add("/usr/share/icons/hicolor/scalable/apps/forge-store.svg",
        ui / "share/icons/hicolor/scalable/apps/forge-store.svg")
    add("/usr/share/forge-store/ui-manifest-v1.json", ui / "share/forge-store/ui-manifest-v1.json")
    for version, source in (("tuf", args.candidate_v1), ("tuf-v2", args.candidate_v2)):
        for path in sorted(source.rglob("*")):
            if path.is_file():
                add("/usr/share/forge-store/" + version + "/" + path.relative_to(source).as_posix(), path)
    receipt = json.loads((args.fixture_dir / "receipt.json").read_text(encoding="utf-8"))
    for fixture in receipt["packages"].values():
        source = args.fixture_dir / fixture["file"]
        if sha256(source) != fixture["sha256"] or source.stat().st_size != fixture["size"]:
            raise ValueError("ForgeOS package fixture receipt mismatch")
        add("/usr/share/forge-store/fixtures/" + fixture["sha256"] + ".forgepkg", source)
    add("/usr/share/forge-store/flatpak/fixture-public.gpg", args.flatpak_dir / "forge-store-fixture-public.gpg")
    for version in ("repo-v1", "repo-v2"):
        source = args.flatpak_dir / version
        if any(path.is_symlink() for path in source.rglob("*")):
            raise ValueError("Flatpak repository has a symlink")
        for path in sorted(source.rglob("*")):
            # OSTree creates a root-owned coordination lock. It is not repository data.
            if path.name == ".lock" and path.parent == source:
                continue
            if path.is_file():
                add("/usr/share/forge-store/flatpak/" + version + "/" + path.relative_to(source).as_posix(), path)
    add("/usr/lib/systemd/user/forge-store.service", args.source_root / "packaging/forge-store.service")
    add("/usr/lib/forge-store/provision-flatpak-fixture",
        args.source_root / "packaging/provision-flatpak-fixture", "0755")
    add("/usr/share/forge-store/source-inventory.json", args.source_root / "docs/source-inventory.json")
    add("/usr/share/forge-store/fixture-LICENSE", args.source_root / "fixtures/flatpak/LICENSE")
    if len(files) > MAX_FILES or sum(path.stat().st_size for path, _ in files.values()) > MAX_TOTAL:
        raise ValueError("bundle exceeds file count or total byte limit")
    return files


def build(args) -> dict:
    files = collect(args)
    records = []
    for target, (source, mode) in sorted(files.items()):
        size = source.stat().st_size
        if size > MAX_FILE:
            raise ValueError(f"oversized bundle file: {target}")
        if mode == "0755":
            with source.open("rb") as stream:
                first = stream.read(256)
            if first.startswith(b"#!") and b"\r" in first.split(b"\n", 1)[0]:
                raise ValueError(f"executable source has a CRLF shebang: {target}")
        records.append({"path": target, "sha256": sha256(source), "mode": mode, "size": size})
    manifest = {"schemaVersion": 1, "sourceCommit": args.source_commit, "files": records}
    encoded = (json.dumps(manifest, sort_keys=True, separators=(",", ":")) + "\n").encode("ascii")
    args.output.parent.mkdir(parents=True, exist_ok=True)
    with args.output.open("wb") as raw, gzip.GzipFile(fileobj=raw, mode="wb", filename="", mtime=0) as compressed:
        with tarfile.open(fileobj=compressed, mode="w") as archive:
            for record in records:
                source = files[record["path"]][0]
                info = tarfile.TarInfo(record["path"].lstrip("/"))
                info.size = record["size"]
                info.mode = int(record["mode"], 8)
                info.uid = info.gid = info.mtime = 0
                with source.open("rb") as stream:
                    archive.addfile(info, stream)
            info = tarfile.TarInfo("bundle-manifest.json")
            info.size = len(encoded)
            info.mode = 0o644
            info.uid = info.gid = info.mtime = 0
            archive.addfile(info, io.BytesIO(encoded))
    args.output.chmod(0o644)
    return {"bundle": str(args.output), "sha256": sha256(args.output),
            "files": len(records), "bytes": sum(item["size"] for item in records),
            "sourceCommit": args.source_commit}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("service-binary", "ui-stage", "candidate-v1", "candidate-v2", "fixture-dir",
                 "flatpak-dir", "source-root", "output"):
        parser.add_argument("--" + name, type=Path, required=True)
    parser.add_argument("--source-commit", required=True)
    args = parser.parse_args()
    if len(args.source_commit) != 40 or not all(ch in "0123456789abcdef" for ch in args.source_commit):
        raise ValueError("source commit must be a full lowercase Git SHA")
    print(json.dumps(build(args), sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
