# HoYoPlay candidate source review

The Mac-Win Git recipe `MacWinManager/Sources/MacWinManagerApp/Resources/Catalog/recipes/hoyoplay-cn.json` uses `installer.mode = alreadyInstalled`, a `$existing` launcher and a path hint to version `1.16.1.364` on a different Mac. It does not contain an installation package URL, artifact size or SHA-256. A path/version hint does not prove a downloadable, licensed fixed package.

No Windows guest installation, GUI game workflow, acceptance signature or local market publication was attempted. The candidate remains uncounted, with stage `existing-install-only-no-pinned-artifact`. This limitation does not block source-pinned independent applications from entering the rolling queue.
