#pragma once
#include <QMargins>
#include <QRect>
#include <QSize>
#include <algorithm>

struct FittedWindowGeometry { QRect geometry; QSize minimum; };
inline FittedWindowGeometry fitWindowGeometry(QRect available, QRect requested, QMargins frame, bool firstLaunch) {
    const int edge = firstLaunch ? 16 : 0;
    const QRect area = available.adjusted(edge + frame.left(), edge + frame.top(), -edge - frame.right(), -edge - frame.bottom());
    if (area.isEmpty()) return {requested, QSize(1, 1)};
    // A valid small saved window remains small when returning to a larger screen.
    const QSize minimum(std::min({820, area.width(), firstLaunch ? 820 : std::max(1, requested.width())}),
                        std::min({580, area.height(), firstLaunch ? 580 : std::max(1, requested.height())}));
    const int capWidth = firstLaunch ? int(area.width() * 0.84) : area.width();
    const int capHeight = firstLaunch ? int(area.height() * 0.84) : area.height();
    requested.setSize(QSize(std::clamp(requested.width(), minimum.width(), std::max(minimum.width(), capWidth)),
                            std::clamp(requested.height(), minimum.height(), std::max(minimum.height(), capHeight))));
    if (firstLaunch) requested.moveCenter(area.center());
    requested.moveLeft(std::clamp(requested.x(), area.left(), area.right() - requested.width() + 1));
    requested.moveTop(std::clamp(requested.y(), area.top(), area.bottom() - requested.height() + 1));
    return {requested, minimum};
}
