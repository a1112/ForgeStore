#!/usr/bin/env python3
"""Read-only source/contract gate for an exact four-repository Forge combination.

All execution evidence here is synthetic protocol evidence. No Wine, image,
service installer, R-OS producer, VM or application is run.
"""
import argparse
import copy
import hashlib
import json
import os
from pathlib import Path
import re
import stat
import subprocess
import sys
import types

REPOS = {name: "a1112/" + name for name in ("CompatForge", "ForgeStore", "ForgeDesktop", "ForgeOS")}
UNCONFIGURED = {"status": "unconfigured", "sourceRepository": None, "sourceCommit": None, "artifactSha256": None}
COMMON = ("contracts/provider-info-v1.schema.json", "contracts/provider-lock-v1.schema.json",
          "contracts/provider-vectors-v1.json", "contracts/provider-raw-vectors-v1.json",
          "contracts/daemon-handshake-v2.schema.json", "contracts/bound-request-v2.schema.json",
          "contracts/bound-response-v2.schema.json", "contracts/daemon-reply-v2.schema.json")


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
    require(type(value) is dict and set(value) == {"schemaVersion", "contractVersion", "components", "rosRustBundle", "validationScope", "desktopBundle"}
            and value["schemaVersion"] == "2" and value["contractVersion"] == "2.0.0"
            and value["validationScope"] == "synthetic-contract-only", "unsupported composition lock schema or evidence scope")
    require(type(value["components"]) is dict and set(value["components"]) == set(REPOS), "composition must pin all four repositories")
    for name, record in value["components"].items():
        require(type(record) is dict and set(record) == {"repository", "sourceCommit", "baseCommit"}
                and record["repository"] == REPOS[name], "invalid composition repository identity")
        for field in ("sourceCommit", "baseCommit"):
            require(type(record[field]) is str and re.fullmatch(r"[0-9a-f]{40}", record[field]), "composition must use full immutable Git SHAs")
    require(value["rosRustBundle"] == UNCONFIGURED, "unverified R-OS Rust producer must remain unconfigured")
    artifact = value["desktopBundle"]
    require(type(artifact) is dict and set(artifact) == {"receiptSha256", "producerSourceCommit", "runtimeProfile", "relativePath"}
            and artifact["producerSourceCommit"] == value["components"]["ForgeDesktop"]["sourceCommit"]
            and type(artifact["receiptSha256"]) is str and re.fullmatch(r"[0-9a-f]{64}", artifact["receiptSha256"])
            and artifact["runtimeProfile"] == "ubuntu-26.04" and artifact["relativePath"] == "verification/desktop-bundle-v2",
            "composition must pin the exact Desktop source-resource receipt and profile")
    return value


def load_lock(path):
    # This bootstrap cannot import an unchecked worktree adapter to read the lock.
    path = Path(path).absolute()
    require(".." not in path.parts, "invalid composition lock path")
    parent = os.open(path.anchor, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW | os.O_CLOEXEC)
    leaf = None
    try:
        for component in path.parts[1:-1]:
            child = os.open(component, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW | os.O_CLOEXEC, dir_fd=parent)
            os.close(parent)
            parent = child
        leaf = os.open(path.name, os.O_RDONLY | os.O_NONBLOCK | os.O_NOFOLLOW | os.O_CLOEXEC, dir_fd=parent)
        before = os.fstat(leaf)
        require(stat.S_ISREG(before.st_mode) and before.st_nlink == 1 and before.st_size <= 65536
                and before.st_uid in (0, os.getuid()) and not before.st_mode & 0o022,
                "composition lock must be a bounded owned regular file")
        raw = bytearray()
        while len(raw) <= 65536:
            chunk = os.read(leaf, min(8192, 65537 - len(raw)))
            if not chunk:
                break
            raw.extend(chunk)
        def identity(m):
            return (m.st_dev, m.st_ino, m.st_size, m.st_mode, m.st_nlink, m.st_mtime_ns, m.st_ctime_ns)
        require(identity(before) == identity(os.fstat(leaf))
                == identity(os.stat(path.name, dir_fd=parent, follow_symlinks=False))
                and len(raw) == before.st_size, "composition lock changed while reading")
    finally:
        if leaf is not None:
            os.close(leaf)
        os.close(parent)
    require(len(raw) <= 65536, "composition lock exceeds 64 KiB")
    return validate_lock(json.loads(raw.decode("utf-8", errors="strict"), object_pairs_hook=unique,
                                   parse_constant=lambda _: require(False, "invalid JSON constant")))


def git(repo, *arguments):
    output = subprocess.run(["git", "-C", str(repo), *arguments], capture_output=True, text=True,
                            timeout=10, check=True)
    return output.stdout.strip()


def blob(repo, commit, path, maximum=65536):
    """Freeze validated immutable Git object bytes, never a worktree pathname."""
    reference = f"{commit}:{path}"
    size = int(git(repo, "cat-file", "-s", reference))
    require(size <= maximum, "Git blob exceeds its byte limit")
    raw = subprocess.run(["git", "-C", str(repo), "cat-file", "blob", reference],
                         capture_output=True, timeout=10, check=True).stdout
    require(len(raw) == size, "Git blob size differs")
    actual = hashlib.sha1(b"blob " + str(len(raw)).encode() + b"\0" + raw).hexdigest()
    require(actual == git(repo, "rev-parse", reference), "contract Git blob identity differs")
    return raw


def module(name, raw, origin):
    result = types.ModuleType(name)
    result.__package__ = name.rpartition(".")[0]
    result.__file__ = origin
    exec(compile(raw.decode("utf-8", errors="strict"), origin, "exec"), result.__dict__)
    return result


def verify_artifact(workspace, lock, source):
    """Validate with frozen OS/producer code and bind every resource to Git bytes.

    Temporarily install an isolated 'tools' namespace because the pinned producer
    uses absolute tools imports. No imports resolve through a worktree directory.
    """
    names = ("tools", "tools.forge_provider_contract", "tools.build_bundle",
             "tools.build_plasma_bundle", "tools.install_forgedesktop_plasma")
    previous = {name: sys.modules.get(name) for name in names}
    try:
        package = types.ModuleType("tools")
        package.__path__ = []
        sys.modules["tools"] = package
        for name, repo, relative in (
            (names[1], "CompatForge", "tools/forge_provider_contract.py"),
            (names[2], "ForgeDesktop", "tools/build_bundle.py"),
            (names[3], "ForgeDesktop", "tools/build_plasma_bundle.py"),
            (names[4], "ForgeOS", "tools/install_forgedesktop_plasma.py"),
        ):
            sys.modules[name] = module(name, source(repo, relative),
                                       f"git:{lock['components'][repo]['sourceCommit']}:{relative}")
        consumer, producer = sys.modules[names[4]], sys.modules[names[3]]
        artifact = lock["desktopBundle"]
        receipt, payloads = consumer.validate_bundle(workspace / artifact["relativePath"], artifact["receiptSha256"],
                                                    runtime_profile=artifact["runtimeProfile"])
        require(receipt["sourceCommit"] == artifact["producerSourceCommit"], "bundle source differs from composition")
        require(consumer.PROVIDER_ASSETS <= set(payloads), "this composition requires the complete provider pair")
        provider = sys.modules[names[1]].decode(payloads[consumer.PROVIDER_LOCK][0])
        require(provider["sourceCommit"] == lock["components"]["CompatForge"]["sourceCommit"], "bundle provider source differs")
        mapping = producer.repository_asset_sources(workspace / "ForgeDesktop", profile=artifact["runtimeProfile"])
        require(set(mapping) == set(payloads), "bundle resources differ from the locked producer mapping")
        for destination, path in mapping.items():
            relative = path.relative_to(workspace / "ForgeDesktop").as_posix()
            raw = blob(workspace / "ForgeDesktop", artifact["producerSourceCommit"], relative, consumer.MAX_FILE)
            expected = raw if destination.endswith(".mo") else raw.replace(b"\r\n", b"\n")
            require(payloads[destination][0] == expected, f"bundle resource is not the locked Git blob: {destination}")
        return {**artifact, "fileCount": len(payloads), "sourceAssetBytesBoundToGit": True,
                "installed": False, "runtimeExecuted": False}
    finally:
        for name in names:
            if previous[name] is None:
                sys.modules.pop(name, None)
            else:
                sys.modules[name] = previous[name]


def verify(workspace, lock):
    workspace = Path(workspace).resolve()
    for name, record in lock["components"].items():
        repo = workspace / name
        require(git(repo, "rev-parse", "HEAD") == record["sourceCommit"], f"{name}: source commit differs from lock")
        require(not git(repo, "status", "--porcelain", "--untracked-files=normal"), f"{name}: source tree is dirty")
        git(repo, "merge-base", "--is-ancestor", record["baseCommit"], record["sourceCommit"])
    hashes = {}
    def source(repo, relative):
        return blob(workspace / repo, lock["components"][repo]["sourceCommit"], relative)
    for relative in COMMON:
        sources = [source(repo, relative) for repo in ("CompatForge", "ForgeStore", "ForgeDesktop")]
        require(sources[0] == sources[1] == sources[2], f"shared schema/vector differs: {relative}")
        hashes[relative] = hashlib.sha256(sources[0]).hexdigest()
    for relative in ("crates/forge-provider-contract/Cargo.toml", "crates/forge-provider-contract/src/lib.rs",
                     "crates/forge-provider-contract/src/binding.rs"):
        raw = source("CompatForge", relative)
        require(raw == source("ForgeStore", relative), "Rust contract copies differ")
        hashes[relative] = hashlib.sha256(raw).hexdigest()
    adapter_path = "tools/forge_provider_contract.py"
    raw = source("CompatForge", adapter_path)
    for repo in ("ForgeDesktop", "ForgeOS"):
        require(raw == source(repo, adapter_path), "Python adapter copies differ")
    hashes[adapter_path] = hashlib.sha256(raw).hexdigest()
    adapter = module("forge_provider_contract_composition", raw,
                     f"git:{lock['components']['CompatForge']['sourceCommit']}:{adapter_path}")
    vectors = json.loads(source("CompatForge", COMMON[2]))
    synthetic = copy.deepcopy(vectors["report"])
    synthetic["sourceCommit"] = lock["components"]["CompatForge"]["sourceCommit"]
    require(synthetic["contractVersion"] == lock["contractVersion"], "provider protocol differs from composition version")
    outcomes = {}
    for repo, relative in (("ForgeStore", "contracts/compatforge-provider-lock-v1.json"),
                           ("ForgeDesktop", "tools/compatforge-provider-lock-v1.json")):
        required = adapter.decode(source(repo, relative))
        require(required["sourceCommit"] == synthetic["sourceCommit"], f"{repo}: provider source pin differs")
        require(required["contractVersion"] == lock["contractVersion"]
                and {"provider-binding-v2"} <= set(required["capabilities"])
                and all(required["schemas"].get(name) == "2" for name in
                        ("daemon-handshake", "daemon-reply", "bound-request", "bound-response")),
                f"{repo}: composition requires daemon/execution binding v2")
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
    artifact = verify_artifact(workspace, lock, source)
    return {"schemaVersion": "2", "validationScope": "synthetic-contract-only", "inputs": "immutable-git-blobs",
            "components": lock["components"], "contractSha256": hashes, "consumers": outcomes,
            "rosRustBundle": UNCONFIGURED, "desktopBundle": artifact}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--workspace", type=Path, required=True)
    parser.add_argument("--lock", type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps(verify(args.workspace, load_lock(args.lock)), sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
