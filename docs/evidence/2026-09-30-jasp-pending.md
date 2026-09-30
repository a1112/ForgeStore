# JASP Statistics 0.97.1 — fixed package verified, service install failed

## Source, package and license

Reviewed [official tagged release](https://github.com/jasp-stats/jasp-desktop/releases/tag/v0.97.1) and GitHub release API. Fixed asset `JASP-0.97.1-Windows-Community.msi`, 1,299,197,952 bytes, SHA-256 `360410ff1cbf63dea1821f6df5c6b883c65e3a9ccc9aed3324fbb2aa4cf71e93` matches official asset digest and existing Mac-Win recipe. Downloaded to builder cache, verified before SSH transfer, then verified again in guest before service submission. Source receipt: `2026-09-30-jasp-source.json`. Installer remains outside Git.

[Tagged COPYING.txt](https://github.com/jasp-stats/jasp-desktop/blob/v0.97.1/COPYING.txt) contains GNU Affero General Public License v3; header and source checksum recorded in `2026-09-30-jasp-license.json`. Bundled components still require their notices at publication. [Current download page](https://jasp-stats.org/download/) offers newer 0.98.1; this test deliberately pins the existing 0.97.1 recipe, not a moving/latest URL.

## Actual installation failure

Normal UID 1000 in existing isolated v12 VM. Registered unknown-rated JASP definition; submitted fixed MSI through real CompatForge install service. Job `job-1790744678544-9`, generation `gen-job-1790744678544-9`, status **failed** before installation. Exact error:

`executable inspection failed: PE image exceeds 268435456 bytes: 1299197952`

`2026-09-30-jasp-install-failure.json` records artifact, submit result and failed service job. This is a service format/handler boundary, not evidence of JASP runtime incompatibility. No msiexec process or GUI workflow ran. No binary masquerading, PE size-limit relaxation, raw Wine bypass or manual CAB extraction was used.

Code investigation: service jobs.rs resolves installer as ImmutableArtifact and unconditionally calls PE inspect_path. InstallerDefinition has no MSI handler. Existing `install-request.schema.json` and non-executing validator require a fixed MSI with closed msiexec semantics, but bound it to 1 GiB. `2026-09-30-jasp-contract.json` independently reproduces rejection using official package metadata; it is a metadata-only test, not an execution receipt.

Existing Phase 2.3 tests: on Windows, 10 ran with 1 failure and 1 error because fixtures use Linux absolute paths and native pathlib rejects them. On Linux builder, the same test file plus referenced source/schema files passed all 10; `2026-09-30-msi-contract-linux.json`. An initial incomplete test transfer omitted scripts/validate_repository.py, then a transient script-generation error occurred; neither was a product test regression. The corrected complete transfer was used for the reported pass.

## Required repair and acceptance

Proposed design in CompatForge `docs/plans/2026-09-30-rolling-msi-integration-design.md`, based on existing Phase 2.3 MSI design. It connects immutable package staging, runtime-owned msiexec and authorized Prepared Install to managed service generations, keeping EXE PE semantics unchanged. It also aligns schema and validator on a bounded 2 GiB package limit. Design review is pending; execution code has not been changed.

After implementation: install exact package, import CSV with Chinese labels and five values 1..5, verify N=5/mean=3/sample SD=sqrt(2.5), save/reopen .jasp, and exercise verified-cache Store update/rollback with data preservation. Four GiB VM RAM is the vendor minimum, so monitor memory rather than presume success.

Count remains **7/1000** accepted local-market applications. Five distinct candidates have explicit pending/failure blockers: Firefox (terms), PortableApps (license), LTspice (moving artifact), WinSCP (SFTP authentication), JASP (MSI service boundary). Source-review-only candidates are not counted as blocked or accepted. No new signed catalogue or GUI-tested receipt was created; candidate-v7 TUF trust preserved.
