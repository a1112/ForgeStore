"""The open-source Forge package fixture is built outside the repository."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
FORGE_PACKAGE_TOOL = os.environ.get("FORGE_PACKAGE_TOOL")


@unittest.skipUnless(FORGE_PACKAGE_TOOL and sys.platform == "linux", "requires Linux ForgeOS package tool")
class FixtureBuildTest(unittest.TestCase):
    def test_builds_two_installable_versions_without_committing_payloads(self):
        with tempfile.TemporaryDirectory() as workspace:
            out = Path(workspace)
            subprocess.run([sys.executable, str(ROOT / "scripts/build_fixture_packages.py"),
                            "--forge-package-tool", FORGE_PACKAGE_TOOL, "--output", workspace], check=True)
            receipt = json.loads((out / "receipt.json").read_text())
            self.assertEqual(set(receipt["packages"]), {"1.0.0", "2.0.0"})
            for version, item in receipt["packages"].items():
                path = out / item["file"]
                self.assertEqual(hashlib.sha256(path.read_bytes()).hexdigest(), item["sha256"])
                verify = subprocess.run([sys.executable, FORGE_PACKAGE_TOOL, "verify",
                    "--package", str(path), "--sha256", item["sha256"]],
                    check=True, capture_output=True, text=True)
                self.assertEqual(json.loads(verify.stdout)["version"], version)


if __name__ == "__main__":
    unittest.main()
