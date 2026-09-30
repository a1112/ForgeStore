# KDiff3 1.12.4 Windows rolling acceptance

KDiff3 passed in the isolated ForgeOS v12 / Wine 11.14 ordinary-user environment and was published in the verified local ForgeStore. Cumulative distinct accepted Windows applications: **9 / 1000**. Six explicit candidate blockers and three remaining source-review candidates are recorded separately; none is counted as complete. No remote public market release was performed.

## Source, license and installation correction

KDE's fixed [Windows installer](https://download.kde.org/stable/kdiff3/kdiff3-1.12.4-windows-x86_64.exe) is 81,410,248 bytes with SHA-256 `2d58b4d675934290146faf08a239788ffc36fd6ac2c480456100fb1d8ab2bb16`, matching the [official mirror manifest](https://download.kde.org/stable/kdiff3/kdiff3-1.12.4-windows-x86_64.exe.mirrorlist). The official URL redirected to the KDE-listed `kde.cs.nycu.edu.tw` mirror. Source receipt, digest verification and guest transfer are recorded. The [tagged application source](https://raw.githubusercontent.com/KDE/kdiff3/1.12.4/src/kdiff3.cpp) specifies `GPL-2.0-or-later`; installed bundled notice paths and digests are retained in the file evidence. Notices were not removed. This local test does not establish readiness for public binary redistribution.

Initial installation `job-1790747398033-18` exited zero but was marked failed with `registry I/O failed: No such file or directory (os error 2)`: the expected launcher `KDiff3/bin/kdiff3.exe` did not exist. Read-only prefix inspection found the actual `Program Files/KDiff3/bin/kdiff3.exe`, SHA-256 `cfb8e14df21b449c4106ab827056c84cd4c54a569ff5ae3fc2dee468ef5e567a`. The intended `/D=C:\KDiff3` location had not been used. The recipe was corrected to this observed launcher path and `/S`, then reinstalled. Job `job-1790747482506-19` succeeded. The first failure is preserved rather than retrospectively counted as a successful installation.

## Real GUI workflow

Two prepared UTF-8 fixtures contained identical headers and Chinese text but different `line: left` / `line: right` lines. Before launch, no output file existed. GUI launch `job-1790747560345-20` displayed the inputs and reported one unresolved conflict. Through noVNC, the B toolbar action was clicked to choose the right-hand line. The GUI reported zero remaining conflicts and Save wrote a new output file.

Independent file inspection verified the 46-byte output exactly matches B: SHA-256 `c33aa0f457d694175a95b8763d0a682dd00f016938e1c78885625aa0016fae37`. A remained unchanged with a different digest. Reopen job `job-1790747695013-21` compared the saved output to B, showed Chinese text correctly and reported that files A and B are binary equal. Conflict, saved-output and reopened-equality screenshots are retained. No automatic merge flag or backend file write generated the output.

## Signed local market

Candidate v9 has targets/snapshot/timestamp version 18, 11 catalogue entries and nine distinct Windows applications; two internal fixtures remain excluded. The KDiff3 GUI receipt is a signed target. Activation preserved all previous installed IDs and Store job history. Trusted root remains `345053af51944c6ad56e4d5ac79c65be0b31b515befcc160fe5d226c6a027d0e`.

Verified-cache update `d34365e8-1865-4d35-aaae-2da4c4bca117` and rollback `5e74f69a-856d-43ba-afd6-02f7601bac3b` succeeded. Store restart preserved installed state and job history; saved output digest remained unchanged after rollback. An initial update helper assertion was accidentally changed from UID 1000 to UID 1100 during helper adaptation, failed before job submission, and was corrected. This was a test harness error, not an application failure.

Fresh-machine recipe registration, online downloading inside the guest, three-way and directory merge, and cross-version upgrade remain unverified. Signed catalogue publication alone is not used as proof of those features. Keys, passwords and installers are outside the source repository.
