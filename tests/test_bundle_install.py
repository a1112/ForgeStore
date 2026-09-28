"""Offline bundle installer must leave a verifiable image receipt."""
import hashlib
import io
import json
from pathlib import Path
import tarfile
import tempfile
import unittest

from scripts.install_bundle import install


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def bundle(path: Path, *, bad_hash=False, linked=False, crlf_script=False):
    payloads = {
        "/usr/bin/forge-store-service": (b"service", "0755"),
        "/usr/lib/systemd/user/forge-store.service": (b"[Install]\nWantedBy=default.target\n", "0644"),
        "/usr/share/forge-store/flatpak/repo-v1/config": (b"[core]\nrepo_version=1\n", "0644"),
        "/usr/share/forge-store/flatpak/repo-v2/config": (b"[core]\nrepo_version=1\n", "0644"),
    }
    if crlf_script:
        payloads["/usr/lib/forge-store/provision-flatpak-fixture"] = (b"#!/bin/sh\r\nexit 0\r\n", "0755")
    records = [{"path": target, "sha256": "0" * 64 if bad_hash else digest(data),
                "mode": mode, "size": len(data)}
               for target, (data, mode) in sorted(payloads.items())]
    manifest = {"schemaVersion": 1, "sourceCommit": "a" * 40, "files": records}
    with tarfile.open(path, "w:gz") as archive:
        for target, (data, mode) in sorted(payloads.items()):
            info = tarfile.TarInfo(target.lstrip("/"))
            info.mode = int(mode, 8)
            if linked and target.endswith("forge-store-service"):
                info.type = tarfile.SYMTYPE
                info.linkname = "/etc/passwd"
                archive.addfile(info)
            else:
                info.size = len(data)
                archive.addfile(info, io.BytesIO(data))
        data = json.dumps(manifest).encode()
        info = tarfile.TarInfo("bundle-manifest.json")
        info.size = len(data)
        archive.addfile(info, io.BytesIO(data))
    return manifest


class BundleInstallTest(unittest.TestCase):
    def test_installs_exact_files_and_enabled_user_service(self):
        with tempfile.TemporaryDirectory() as name:
            root = Path(name)
            archive = root / "bundle.tar.gz"
            manifest = bundle(archive)
            image = root / "image"
            image.mkdir()
            receipt = install(archive, image)
            self.assertEqual(receipt["files"], manifest["files"])
            self.assertEqual(receipt["bundleSha256"], digest(archive.read_bytes()))
            self.assertTrue(receipt["serviceEnabled"])
            self.assertEqual((image / "usr/bin/forge-store-service").read_bytes(), b"service")
            link = image / "etc/systemd/user/default.target.wants/forge-store.service"
            self.assertEqual(link.readlink().as_posix(), "/usr/lib/systemd/user/forge-store.service")
            for version in ("repo-v1", "repo-v2"):
                self.assertTrue((image / f"usr/share/forge-store/flatpak/{version}/tmp/cache").is_dir())
                self.assertTrue((image / f"usr/share/forge-store/flatpak/{version}/state").is_dir())

    def test_rejects_wrong_digest_and_linked_member(self):
        with tempfile.TemporaryDirectory() as name:
            root = Path(name)
            archive = root / "bundle.tar.gz"
            bundle(archive, bad_hash=True)
            (root / "image").mkdir()
            with self.assertRaises(ValueError):
                install(archive, root / "image")
            bundle(archive, linked=True)
            with self.assertRaises(ValueError):
                install(archive, root / "image")
            bundle(archive, crlf_script=True)
            with self.assertRaises(ValueError):
                install(archive, root / "image")


if __name__ == "__main__":
    unittest.main()
