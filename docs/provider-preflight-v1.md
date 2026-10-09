# CompatForge preflight and fixed compositions

The Store backend runs read-only `provider-info` before every `service-call`,
including availability queries. It checks the exact clean provider source,
package/contract version, Linux target/service, command versions, request/reply
schemas, generation/job capabilities and operations against
`contracts/compatforge-provider-lock-v1.json`. It creates no request directory or
file when preflight fails. Snapshot availability carries the bounded concrete
rejection reason; an old binary merely existing on disk cannot enable the backend.
`service-response` schema/identity validation remains a separate reply gate.

The Rust crate is an exact copy of CompatForge's independently versioned
`forge-provider-contract` 1.0.0. Shared schemas and negative vectors are byte
identical. The public-code adapter maps the domain errors to R-SDK interop v1
families while preserving the independent Forge protocol/ABI and domain code.
No R-SDK crate, native ABI or compatibility engine is modified.

`tools/verify_provider_composition.py --workspace <four-checkout-parent> --lock
<composition.json>` verifies exact, clean source revisions, base ancestry,
identical reusable contract copies, matching provider pins and negative
version/schema/command/source cases for Store and Desktop. The versioned lock
contains four immutable source/base SHAs, repository identities and the explicit
`synthetic-contract-only` scope. The unverified R-OS Rust producer must have
`status=unconfigured` and null source/digest fields. Moving branches, a missing
component or fabricated producer evidence are rejected.

The independent candidate review must cover each repository against its stated
base. CompatForge is derived solely from the managed-MSI candidate, without
reconciling its divergence with main. Rollback restores a matching complete
consumer/provider combination; native and Flatpak backends are unchanged.
Synthetic tests and the macOS `provider-info` binary do not establish Linux
service readiness, Wine/application behavior, image boot or VM acceptance.
