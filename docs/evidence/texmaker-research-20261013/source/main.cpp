/***************************************************************************
 *   copyright       : (C) 2003-2023 by Pascal Brachet                     *
 *   https://www.xm1math.net/texmaker/                                     *
 *                                                                         *
 *   addons by Frederic Devernay <frederic.devernay@m4x.org>               *
 *   addons by Luis Silvestre                                              *
 *   This program is free software; you can redistribute it and/or modify  *
 *   it under the terms of the GNU General Public License as published by  *
 *   the Free Software Foundation; either version 2 of the License, or     *
 *   (at your option) any later version.                                   *
 *                                                                         *
 *   @license GPL-2.0+ <https://spdx.org/licenses/GPL-2.0+.html>           *
 ***************************************************************************/

#include "texmakerapp.h"
#include <QStringList>
#include <QDebug>
#include <QTextStream>
#define STRINGIFY_INTERNAL(x) #x
#define STRINGIFY(x) STRINGIFY_INTERNAL(x)

#define VERSION_STR STRINGIFY(TEXMAKERVERSION)

// char** appendCommandLineArguments(int argc, char **argv, const QStringList& args)
// {
//   size_t newSize = (argc + args.length() + 1) * sizeof(char*);
//   char** newArgv = (char**)calloc(1, newSize);
//   memcpy(newArgv, argv, (size_t)(argc * sizeof(char*)));
//
//   int pos = argc;
//   for(const QString& str : args)
//     newArgv[pos++] = qstrdup(str.toUtf8().data());
//
//   return newArgv;
// }

int main( int argc, char ** argv )
{

#if !defined(Q_OS_MAC)
QCoreApplication::setAttribute(Qt::AA_EnableHighDpiScaling);
#endif
#if QT_VERSION >= QT_VERSION_CHECK(5, 14, 0)
qputenv("QT_ENABLE_HIGHDPI_SCALING", "1");
QGuiApplication::setHighDpiScaleFactorRoundingPolicy(Qt::HighDpiScaleFactorRoundingPolicy::PassThrough);
#endif

QStringList rawargs;
for ( int i = 0; i < argc; ++i )
	{
	rawargs.append( argv[ i ]);
	}
for (QStringList::Iterator it = ++(rawargs.begin()); it != rawargs.end(); it++)
    {
    if ( ( *it == "-dpiscale") && (++it != rawargs.end())) {qputenv("QT_SCALE_FACTOR", (*it).toUtf8());}
    else if (  *it == "--version" )
        {
        QTextStream output (stdout);
        output << QLatin1String(VERSION_STR)+QString::fromUtf8(" (compiled with Qt ")+QLatin1String(QT_VERSION_STR)+")\n";
        output.flush();
        return 0;
        }
    else if (  *it == "--help" )
        {
        QTextStream output (stdout);
        output << "Usage: " <<  QString::fromUtf8("texmaker [-master] [-line xx] [-insert foo] [-n] [-dpiscale x] [file]\n\n");
        output << QString::fromUtf8("-master : with this option, the document will be automatically defined as a master document.\n\n");
        output << QString::fromUtf8("-line xx : with this option, Texmaker will ask you if you want to jump to the xx line after loading the document.\n\n");
        output << QString::fromUtf8("-insert foo : with this option, a latex command foo can be inserted by an external program to the current document while texmaker is running.\n\n");
        output << QString::fromUtf8("-n : with this option, a new instance of Texmaker is launched\n\n");
        output << QString::fromUtf8("-dpiscale x : with this option, the environment variable QT_SCALE_FACTOR will be set to x before building the GUI\n\n");
        output.flush();
        return 0;
        }
    }


// QStringList g_qtFlags = {"--disable-web-security"};
// char **newArgv = appendCommandLineArguments(argc, argv, g_qtFlags);
// int newArgc = argc + g_qtFlags.size();
//
// TexmakerApp app("TexMaker", newArgc, newArgv );
TexmakerApp app("TexMaker", argc, argv );
QStringList args = QCoreApplication::arguments();
app.setApplicationName("TexMaker");
app.setApplicationVersion(VERSION_STR);
app.setOrganizationName("xm1");
app.setAttribute(Qt::AA_DontShowIconsInMenus, true);
app.setAttribute(Qt::AA_UseHighDpiPixmaps);

#if defined(Q_OS_UNIX) && !defined(Q_OS_MAC)
QApplication::setAttribute(Qt::AA_DontUseNativeMenuBar);
#endif



if ( app.isRunning() && !args.contains("-n")) 
    {
    QString msg;
    msg = args.join("#!#");
    msg += "#!#";
    app.sendMessage( msg );
    return 0;
    }

app.init(args); // Initialization takes place only if there is no other instance running.

QObject::connect( &app, SIGNAL( messageReceived(const QString &) ), 
                  app.mw,   SLOT( onOtherInstanceMessage(const QString &) ) );

return app.exec();
}
