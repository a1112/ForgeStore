cmake_minimum_required(VERSION 3.25)
project(ForgeStoreUi LANGUAGES CXX)
if(APPLE)
    enable_language(OBJCXX)
endif()

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_AUTOMOC ON)

find_package(Qt6 REQUIRED COMPONENTS Core Network Qml Quick QuickControls2 Test)

enable_testing()
add_library(forge-store-window-effects STATIC src/windoweffects.cpp src/windoweffects.h src/effectspolicy.h src/macosutils.h)
target_include_directories(forge-store-window-effects PUBLIC src)
target_link_libraries(forge-store-window-effects PUBLIC Qt6::Core Qt6::Gui)
if(APPLE)
    target_sources(forge-store-window-effects PRIVATE src/macosutils.mm)
    target_link_libraries(forge-store-window-effects PRIVATE "-framework AppKit")
elseif(WIN32)
    target_link_libraries(forge-store-window-effects PRIVATE dwmapi)
endif()
add_library(forge-store-bridge STATIC src/StoreBridge.cpp src/StoreBridge.h)
target_include_directories(forge-store-bridge PUBLIC src)
target_link_libraries(forge-store-bridge PUBLIC Qt6::Core Qt6::Network)

add_executable(forge-store-bridge-test tests/store_bridge_test.cpp)
target_include_directories(forge-store-bridge-test PRIVATE src)
target_link_libraries(forge-store-bridge-test PRIVATE forge-store-bridge Qt6::Test)
add_test(NAME forge-store-bridge-test COMMAND forge-store-bridge-test)

add_executable(forge-store-qml-smoke-test tests/qml_smoke_test.cpp)
target_compile_definitions(forge-store-qml-smoke-test PRIVATE FORGE_STORE_UI_SOURCE_DIR="${CMAKE_CURRENT_SOURCE_DIR}" FORGE_STORE_UI_BINARY="$<TARGET_FILE:forge-store-ui>")
target_link_libraries(forge-store-qml-smoke-test PRIVATE forge-store-bridge forge-store-window-effects Qt6::Qml Qt6::Quick Qt6::QuickControls2 Qt6::Test)
qt_add_resources(forge-store-qml-smoke-test "forge-store-test-assets"
    PREFIX "/forge-store"
    FILES qml/Main.qml qml/WindowChrome.qml qml/ResizeHandles.qml translations/Strings.js assets/forge-store.svg)
add_test(NAME forge-store-qml-smoke-test COMMAND forge-store-qml-smoke-test)
set_tests_properties(forge-store-qml-smoke-test PROPERTIES ENVIRONMENT "QT_QPA_PLATFORM=offscreen;QSG_RHI_BACKEND=software")

add_executable(forge-store-window-effects-test tests/window_effects_test.cpp)
target_link_libraries(forge-store-window-effects-test PRIVATE forge-store-bridge forge-store-window-effects Qt6::Qml Qt6::Quick Qt6::QuickControls2 Qt6::Test)
qt_add_resources(forge-store-window-effects-test "forge-store-effects-test-assets"
    PREFIX "/forge-store"
    FILES qml/Main.qml qml/WindowChrome.qml qml/ResizeHandles.qml translations/Strings.js)
add_test(NAME forge-store-window-effects-test COMMAND forge-store-window-effects-test)
set_tests_properties(forge-store-window-effects-test PROPERTIES TIMEOUT 60 ENVIRONMENT "QT_QPA_PLATFORM=offscreen;QT_QUICK_BACKEND=software")

qt_add_executable(forge-store-ui src/main.cpp)
target_link_libraries(forge-store-ui PRIVATE forge-store-bridge forge-store-window-effects Qt6::Qml Qt6::Quick Qt6::QuickControls2)
qt_add_resources(forge-store-ui "forge-store-assets"
    PREFIX "/forge-store"
    FILES qml/Main.qml qml/WindowChrome.qml qml/ResizeHandles.qml translations/Strings.js assets/forge-store.svg)
add_dependencies(forge-store-qml-smoke-test forge-store-ui)

install(TARGETS forge-store-ui RUNTIME DESTINATION bin)
install(FILES assets/forge-store.desktop DESTINATION share/applications RENAME forge-store.desktop)
install(FILES assets/forge-store.svg DESTINATION share/icons/hicolor/scalable/apps)
install(FILES manifest-v1.json DESTINATION share/forge-store RENAME ui-manifest-v1.json)
