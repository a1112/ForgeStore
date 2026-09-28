# ForgeStore UI

The native Qt 6 window talks only to the per-user ForgeStore service at
`${XDG_RUNTIME_DIR}/forge-store/store.sock`. It does not perform installation
work inside the UI process. Closing and reopening the window keeps service
jobs and their state intact.

Build with CMake and Qt 6 Core, Network, Qml, Quick, QuickControls2 and Test:

```sh
cmake -S ui -B build/forge-store-ui -G Ninja
cmake --build build/forge-store-ui
ctest --test-dir build/forge-store-ui --output-on-failure
cmake --install build/forge-store-ui --prefix /usr
```

The executable contains the QML interface and Chinese/English strings as Qt
resources. The install step adds `bin/forge-store-ui`, a desktop entry, an
original SVG icon and `share/forge-store/ui-manifest-v1.json`. The runtime
needs the Qt Quick Controls and Layouts QML modules and an appropriate Qt
platform plugin (`xcb` on the current X11 desktop). Set
`FORGE_STORE_SOFTWARE_RENDER=1` for software Qt Quick rendering in the VM.

The initial language is simplified Chinese. The language selector switches
the UI to English without restarting. Catalogue names and summaries follow
the selected language when both translations are supplied by the catalogue.

`ui/manifest-v1.json` records the local candidate source and the Arch Qt
packages used in the verified build. The project has no declared repository
license, so the manifest does not claim permission for public redistribution.
