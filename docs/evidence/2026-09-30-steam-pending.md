# Steam bootstrap review — 2026-09-30

Official installer URL https://cdn.fastly.steamstatic.com/client/installer/SteamSetup.exe returned 2380800 bytes, MZ signature and SHA-256 7d3654531c32d941b8cae81c4137fc542172bfa9635f169cb392f245a0a12bcb, matching the existing recipe. This is bootstrap artifact identity, not a fixed full Steam client version. Full client download/update content, version, applicable redistribution terms and an authorized functional workflow still require review. No install, GUI acceptance or market publication was performed.

Existing Mac-Win recipe contains certificate-ignore and CEF sandbox-disable overrides. They were read as source data and were not applied to ForgeOS. Acceptance must preserve current security protections; the recipe's settings do not authorize a security change.
