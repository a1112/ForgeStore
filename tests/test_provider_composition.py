import copy
from pathlib import Path
import importlib.util
import json
import os
import subprocess
import tempfile
import unittest

spec = importlib.util.spec_from_file_location("composition", Path(__file__).parents[1] / "tools/verify_provider_composition.py")
gate = importlib.util.module_from_spec(spec)
spec.loader.exec_module(gate)


class CompositionLockTests(unittest.TestCase):
    def fixture(self):
        return {"schemaVersion": "2", "contractVersion": "2.0.0", "validationScope": "synthetic-contract-only",
                "desktopBundle": {"receiptSha256":"c" * 64, "producerSourceCommit":"a" * 40,
                                  "runtimeProfile":"ubuntu-26.04", "relativePath":"verification/desktop-bundle-v2"},
                "rosRustBundle": copy.deepcopy(gate.UNCONFIGURED), "components": {
                    name: {"repository": repo, "sourceCommit": "a" * 40, "baseCommit": "b" * 40}
                    for name, repo in gate.REPOS.items()}}

    def test_complete_immutable_combination_is_valid(self):
        gate.validate_lock(self.fixture())

    def test_moving_refs_wrong_repository_and_missing_components_are_rejected(self):
        for mutation in ("moving", "repository", "missing"):
            value = self.fixture()
            if mutation == "moving": value["components"]["CompatForge"]["sourceCommit"] = "main"
            if mutation == "repository": value["components"]["CompatForge"]["repository"] = "lcxinc/CompatForge"
            if mutation == "missing": del value["components"]["ForgeOS"]
            with self.assertRaises(ValueError): gate.validate_lock(value)

    def test_unverified_producer_and_runtime_evidence_claims_are_rejected(self):
        value = self.fixture()
        value["rosRustBundle"]["artifactSha256"] = "a" * 64
        with self.assertRaises(ValueError): gate.validate_lock(value)

    def test_hidden_worktree_modification_is_never_imported(self):
        with tempfile.TemporaryDirectory() as temp:
            repo = Path(temp)
            def git(*args):
                return subprocess.run(["git", "-C", str(repo), *args], check=True, capture_output=True, text=True).stdout.strip()
            git("init", "--quiet")
            path = repo / "adapter.py"
            path.write_text("value = 'pinned'\n")
            git("add", "adapter.py")
            git("-c", "user.name=Contract test", "-c", "user.email=contract@example.invalid",
                "commit", "--quiet", "-m", "temporary synthetic object")
            commit = git("rev-parse", "HEAD")
            git("update-index", "--assume-unchanged", "adapter.py")
            marker = repo / "must-not-exist"
            path.write_text(f"from pathlib import Path\nPath({str(marker)!r}).write_text('bad')\nvalue = 'unpinned'\n")
            self.assertEqual(git("status", "--porcelain"), "")
            raw = gate.blob(repo, commit, "adapter.py")
            adapter = gate.module("synthetic_adapter", raw, "git:synthetic:adapter.py")
            self.assertEqual(adapter.value, "pinned")
            self.assertFalse(marker.exists())

    def test_composition_lock_special_files_fail_without_blocking(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            regular = root / "lock"
            regular.write_text(json.dumps(self.fixture()))
            link, fifo = root / "link", root / "fifo"
            link.symlink_to(regular)
            os.mkfifo(fifo)
            for path in (link, fifo):
                with self.assertRaises((OSError, ValueError)):
                    gate.load_lock(path)

    def test_unbound_artifact_source_profile_or_missing_pin_is_rejected(self):
        for field, value in (("producerSourceCommit", "f" * 40), ("receiptSha256", "moving-latest"),
                             ("runtimeProfile", "unknown"), ("relativePath", "../foreign")):
            lock = self.fixture()
            lock["desktopBundle"][field] = value
            with self.assertRaises(ValueError):
                gate.validate_lock(lock)
        value = self.fixture()
        value["validationScope"] = "wine-runtime-verified"
        with self.assertRaises(ValueError): gate.validate_lock(value)


if __name__ == "__main__": unittest.main()
