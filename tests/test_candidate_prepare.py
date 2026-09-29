"""A new installer pin must not inherit a historical compatibility verdict."""
import hashlib
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest

SPEC = importlib.util.spec_from_file_location(
    "prepare_candidate_catalog", Path(__file__).resolve().parents[1] / "scripts/prepare_candidate_catalog.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class CandidatePreparationTest(unittest.TestCase):
    def test_new_installer_is_unknown_until_actual_acceptance_review(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            installer = root / "new-installer.exe"
            installer.write_bytes(b"unexecuted installer fixture")
            package = root / "fixture.forgepkg"
            package.write_bytes(b"package fixture")
            pins = root / "pins.json"
            pins.write_text(json.dumps({"applications": [{
                "id": "7zip", "file": installer.name, "version": "99.0-unverified",
                "sha256": hashlib.sha256(installer.read_bytes()).hexdigest(),
                "url": "https://www.7-zip.org/a/new-installer.exe", "license": "LGPL-2.1-or-later"
            }]}))
            receipt = root / "receipt.json"
            receipt.write_text(json.dumps({"packages": {"1.0.0": {
                "file": package.name, "sha256": hashlib.sha256(package.read_bytes()).hexdigest(),
                "size": package.stat().st_size
            }}}))
            catalog = MODULE.prepare(pins, root, receipt, root, root / "out")
            self.assertEqual(catalog["entries"][0]["compatibility"],
                             {"status": "unknown", "evidence": None})
            saved = json.loads((root / "out/targets/catalogue.json").read_text(encoding="utf-8"))
            self.assertEqual(saved, catalog)


if __name__ == "__main__":
    unittest.main()
