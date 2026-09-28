#!/usr/bin/env python3
"""Verify and install an offline ForgeStore bundle into a mounted rootfs."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import stat
import tarfile
import tempfile

MAX_FILES = 128
MAX_FILE = 64 * 1024 * 1024
MAX_TOTAL = 512 * 1024 * 1024
MAX_MANIFEST = 64 * 1024
UNIT = "/usr/lib/systemd/user/forge-store.service"
ENABLE = "etc/systemd/user/default.target.wants/forge-store.service"
HEX64 = re.compile(r"[0-9a-f]{64}\Z")
HEX40 = re.compile(r"[0-9a-f]{40}\Z")


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def safe_path(path: str) -> str:
    if not isinstance(path, str) or not path.startswith("/usr/"):
        raise ValueError("bundle file must be an absolute /usr path")
    parts = path[1:].split("/")
    if any(part in ("", ".", "..") or not part.isascii() for part in parts):
        raise ValueError("invalid bundle file path")
    return "/".join(parts)


def parse_manifest(raw: bytes) -> dict:
    if len(raw) > MAX_MANIFEST:
        raise ValueError("bundle manifest is too large")
    manifest = json.loads(raw)
    if set(manifest) != {"schemaVersion", "sourceCommit", "files"} or manifest["schemaVersion"] != 1:
        raise ValueError("invalid bundle manifest schema")
    if not isinstance(manifest["sourceCommit"], str) or not HEX40.fullmatch(manifest["sourceCommit"]):
        raise ValueError("invalid source commit")
    records = manifest["files"]
    if not isinstance(records, list) or not 1 <= len(records) <= MAX_FILES:
        raise ValueError("invalid bundle file count")
    seen = set()
    total = 0
    for record in records:
        if not isinstance(record, dict) or set(record) != {"path", "sha256", "mode", "size"}:
            raise ValueError("invalid file record")
        name = safe_path(record["path"])
        if name in seen or not isinstance(record["sha256"], str) or not HEX64.fullmatch(record["sha256"]):
            raise ValueError("duplicate path or invalid digest")
        if record["mode"] not in ("0644", "0755"):
            raise ValueError("invalid bundle file mode")
        size = record["size"]
        if type(size) is not int or size < 0 or size > MAX_FILE:
            raise ValueError("invalid bundle file size")
        total += size
        seen.add(name)
    if total > MAX_TOTAL or UNIT[1:] not in seen:
        raise ValueError("bundle exceeds size limit or omits user service")
    if [item["path"] for item in records] != sorted(item["path"] for item in records):
        raise ValueError("bundle manifest files are not sorted")
    return manifest


def inspect(archive: tarfile.TarFile) -> tuple[dict, dict]:
    members = archive.getmembers()
    by_name = {}
    for member in members:
        if member.name in by_name or not member.isfile() or member.name.startswith("/"):
            raise ValueError("duplicate, linked, or invalid bundle member")
        by_name[member.name] = member
    metadata = by_name.get("bundle-manifest.json")
    if metadata is None or metadata.size > MAX_MANIFEST:
        raise ValueError("bundle manifest missing or oversized")
    stream = archive.extractfile(metadata)
    if stream is None:
        raise ValueError("bundle manifest unreadable")
    manifest = parse_manifest(stream.read(MAX_MANIFEST + 1))
    expected = {safe_path(record["path"]): record for record in manifest["files"]}
    if set(by_name) != set(expected) | {"bundle-manifest.json"}:
        raise ValueError("bundle member set differs from manifest")
    for name, record in expected.items():
        member = by_name[name]
        if member.size != record["size"] or member.mode != int(record["mode"], 8):
            raise ValueError("bundle member size or mode mismatch")
        digest = hashlib.sha256()
        stream = archive.extractfile(member)
        if stream is None:
            raise ValueError("bundle member unreadable")
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
        if digest.hexdigest() != record["sha256"]:
            raise ValueError("bundle member digest mismatch")
    return manifest, by_name


def ensure_real_parents(root: Path, destination: Path) -> None:
    current = root
    if current.is_symlink():
        raise ValueError("rootfs is a symlink")
    for part in destination.relative_to(root).parts[:-1]:
        current = current / part
        if current.is_symlink() or (current.exists() and not current.is_dir()):
            raise ValueError("bundle parent is not a real directory")
        current.mkdir(exist_ok=True)


def install(bundle: Path, rootfs: Path) -> dict:
    if rootfs.is_symlink() or not rootfs.is_dir():
        raise ValueError("rootfs must be an existing real directory")
    bundle_sha = sha256(bundle)
    with tarfile.open(bundle, "r:gz") as archive:
        manifest, members = inspect(archive)
        for record in manifest["files"]:
            name = safe_path(record["path"])
            destination = rootfs / name
            ensure_real_parents(rootfs, destination)
            if destination.is_symlink() or (destination.exists() and not destination.is_file()):
                raise ValueError("bundle destination is not a regular file")
            stream = archive.extractfile(members[name])
            if stream is None:
                raise ValueError("bundle member unreadable")
            fd, temporary = tempfile.mkstemp(prefix=".forge-store-", dir=destination.parent)
            try:
                with os.fdopen(fd, "wb") as output:
                    for block in iter(lambda: stream.read(1024 * 1024), b""):
                        output.write(block)
                    output.flush()
                    os.fsync(output.fileno())
                os.chmod(temporary, int(record["mode"], 8))
                os.replace(temporary, destination)
            finally:
                if os.path.exists(temporary):
                    os.unlink(temporary)
            if destination.stat().st_size != record["size"] or sha256(destination) != record["sha256"]:
                raise ValueError("post-install file verification failed")
            if os.name != "nt" and stat.S_IMODE(destination.stat().st_mode) != int(record["mode"], 8):
                raise ValueError("post-install mode verification failed")
    enable = rootfs / ENABLE
    ensure_real_parents(rootfs, enable)
    if enable.is_symlink():
        if os.readlink(enable) != UNIT:
            raise ValueError("existing user service enable link differs")
    elif enable.exists():
        raise ValueError("existing user service enable path is not a link")
    else:
        enable.symlink_to(UNIT)
    receipt = {"schemaVersion": 1, "bundleSha256": bundle_sha,
               "sourceCommit": manifest["sourceCommit"], "files": manifest["files"],
               "serviceEnabled": enable.is_symlink() and os.readlink(enable) == UNIT}
    return receipt


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--bundle", required=True, type=Path)
    parser.add_argument("--rootfs", required=True, type=Path)
    args = parser.parse_args()
    print(json.dumps(install(args.bundle, args.rootfs), sort_keys=True, separators=(",", ":")))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
