import copy
from pathlib import Path
import importlib.util
import unittest

spec = importlib.util.spec_from_file_location("composition", Path(__file__).parents[1] / "tools/verify_provider_composition.py")
gate = importlib.util.module_from_spec(spec)
spec.loader.exec_module(gate)


class CompositionLockTests(unittest.TestCase):
    def fixture(self):
        return {"schemaVersion": "1", "contractVersion": "1.0.0", "validationScope": "synthetic-contract-only",
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
        value = self.fixture()
        value["validationScope"] = "wine-runtime-verified"
        with self.assertRaises(ValueError): gate.validate_lock(value)


if __name__ == "__main__": unittest.main()
