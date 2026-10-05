# Ubuntu runtime evidence — 2026-10-05

The pre-publication snapshot distinguishes three candidates, two installed
GUI-accepted samples, and zero accepted publications. Windows remains 22/1000
and Ubuntu remains `deploymentReady:false`. Consult the ledger and separate
publication receipt for any subsequent market activation.

The subsequent `ubuntu-market-v4-publication.json` records actual service and GUI
activation of two tested entries, retaining nine complete job rows, six native
records and the original root. The GUI proofs below retain their original
pre-publication state.

- `kate-lifecycle-acceptance.json`: fixed .deb GUI file round-trip, windows,
  uninstall/reinstall, service restart and normal session relogin. Its original
  31-attachment proof is immutable.
- `snap-calculator-gui-acceptance.json`: fixed strict Snap 972/48.1, actual
  calculations, nine windows, close and new-process reopen. Empty history, first
  move failure and artifact-hash PermissionError remain recorded.
- `snap-close-evidence-correction.json`: the original close screenshot captured
  a frame before closure; the PID disappearance readback is the original close
  proof. A later process-free readback and `snap-calculator-closed-confirmed.jpg`
  supplement it without rewriting the signed GUI proof.
- `native-journal-readonly-state.json`: historical native deployment readback
  preserving six complete root journals and nine Store IDs. Its job snapshot
  predates the second Flatpak timeout; do not mistake it for current running work.
- `flatpak-retry-timeout-readback.json`: both 1800-second failures, final no-process
  and empty-app-list confirmation, retained cache, and all nine terminal jobs.
- `tuf-cache-serialization-order.json`: original raw-cache hash equality failure
  and exact reproduction using field order alone; pinned signed content and
  signatures stayed unchanged.

Historical proofs and their attachment manifests are not rewritten when later
observations arrive. The first invalid candidate-v1 is not a publishable input.
No evidence here claims a successful Flatpak GUI test, persistent Snap history,
full Ubuntu dependency/license closure, or a cold VM reboot acceptance.

- `arch-rolling-restoration.json` and its desktop screenshot record restoration of the separate Windows rolling environment, unchanged full task history/data/power/trust, and retained recovery attempts. This does not add an application to either acceptance count.
