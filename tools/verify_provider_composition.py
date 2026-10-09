#!/usr/bin/env python3
"""Read-only source/contract gate for an exact four-repository Forge combination.

All execution evidence here is synthetic protocol evidence. No Wine, image,
service installer, R-OS producer, VM or application is run.
"""
import argparse
import copy
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import subprocess

REPOS = {name: "a1112/" + name for name in ("CompatForge", "ForgeStore", "ForgeDesktop", "ForgeOS")}
UNCONFIGURED = {"status": "unconfigured", "sourceRepository": None, "sourceCommit": None, "artifactSha256": None}
COMMON = ("contracts/provider-info-v1.schema.json", "contracts/provider-lock-v1.schema.json", "contracts/provider-vectors-v1.json")


def require(condition, message):
    if not condition:
        raise ValueError(message)


def unique(pairs):
    result = {}
    for key, value in pairs:
        require(key not in result, "duplicate composition lock field")
        result[key] = value
    return result


def validate_lock(value):
    require(type(value) is dict and set(value) == {"schemaVersion", "contractVersion", "components", "rosRustBundle", "validationScope"}
            and value["schemaVersion"] == "1" and value["contractVersion"] == "1.0.0"
            and value["validationScope"] == "synthetic-contract-only", "unsupported composition lock schema or evidence scope")
    require(type(value["components"]) is dict and set(value["components"]) == set(REPOS), "composition must pin all four repositories")
    for name, record in value["components"].items():
        require(type(record) is dict and set(record) == {"repository", "sourceCommit", "baseCommit"}
                and record["repository"] == REPOS[name], "invalid composition repository identity")
        for field in ("sourceCommit", "baseCommit"):
            require(type(record[field]) is str and re.fullmatch(r"[0-9a-f]{40}", record[field]), "composition must use full immutable Git SHAs")
    require(value["rosRustBundle"] == UNCONFIGURED, "unverified R-OS Rust producer must remain unconfigured")
    return value


def load_lock(path):
    with Path(path).open("rb") as stream:
        raw = stream.read(65537)
    require(len(raw) <= 65536, "composition lock exceeds 64 KiB")
    return validate_lock(json.loads(raw, object_pairs_hook=unique,
                                   parse_constant=lambda _: require(False, "invalid JSON constant")))


def git(repo, *arguments):
    output = subprocess.run(["git", "-C", str(repo), *arguments], capture_output=True, text=True,
                            timeout=10, check=True)
    return output.stdout.strip()


def module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    result = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(result)
    return result


def verify(workspace, lock):
    workspace = Path(workspace).resolve()
    for name, record in lock["components"].items():
        repo = workspace / name
        require(git(repo, "rev-parse", "HEAD") == record["sourceCommit"], f"{name}: source commit differs from lock")
        require(not git(repo, "status", "--porcelain", "--untracked-files=normal"), f"{name}: source tree is dirty")
        git(repo, "merge-base", "--is-ancestor", record["baseCommit"], record["sourceCommit"])
    hashes = {}
    for relative in COMMON:
        sources = [(workspace / repo / relative).read_bytes() for repo in ("CompatForge", "ForgeStore", "ForgeDesktop")]
        require(sources[0] == sources[1] == sources[2], f"shared schema/vector differs: {relative}")
        hashes[relative] = hashlib.sha256(sources[0]).hexdigest()
    for relative in ("crates/forge-provider-contract/Cargo.toml", "crates/forge-provider-contract/src/lib.rs"):
        raw = (workspace / "CompatForge" / relative).read_bytes()
        require(raw == (workspace / "ForgeStore" / relative).read_bytes(), "Rust contract copies differ")
        hashes[relative] = hashlib.sha256(raw).hexdigest()
    adapter_path = "tools/forge_provider_contract.py"
    raw = (workspace / "CompatForge" / adapter_path).read_bytes()
    for repo in ("ForgeDesktop", "ForgeOS"):
        require(raw == (workspace / repo / adapter_path).read_bytes(), "Python adapter copies differ")
    hashes[adapter_path] = hashlib.sha256(raw).hexdigest()
    adapter = module("forge_provider_contract_composition", workspace / "CompatForge" / adapter_path)
    vectors = json.loads((workspace / "CompatForge" / COMMON[2]).read_bytes())
    synthetic = copy.deepcopy(vectors["report"])
    synthetic["sourceCommit"] = lock["components"]["CompatForge"]["sourceCommit"]
    outcomes = {}
    for repo, relative in (("ForgeStore", "contracts/compatforge-provider-lock-v1.json"),
                           ("ForgeDesktop", "tools/compatforge-provider-lock-v1.json")):
        required = adapter.load_lock(workspace / repo / relative)
        require(required["sourceCommit"] == synthetic["sourceCommit"], f"{repo}: provider source pin differs")
        adapter.negotiate(synthetic, required)
        cases = [("commands", {}, "capability-missing"), ("schemas", {}, "schema-mismatch"),
                 ("providerVersion", "0.13.0", "unsupported-version"),
                 ("sourceCommit", "f" * 40, "source-mismatch")]
        results = []
        for field, value, expected in cases:
            info = copy.deepcopy(synthetic)
            info[field] = value
            try:
                adapter.negotiate(info, required)
            except adapter.ContractError as error:
                require(error.code == expected, f"{repo}: unexpected rejection code")
                results.append({"field": field, "domainCode": error.code, "publicCode": error.public_code})
            else:
                raise ValueError(f"{repo}: incompatible provider accepted")
        outcomes[repo] = {"matchingSyntheticProvider": "accepted", "rejections": results}
    return {"schemaVersion": "1", "validationScope": "synthetic-contract-only",
            "components": lock["components"], "contractSha256": hashes, "consumers": outcomes,
            "rosRustBundle": UNCONFIGURED}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--workspace", type=Path, required=True)
    parser.add_argument("--lock", type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps(verify(args.workspace, load_lock(args.lock)), sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
