Unicode true
Name "ForgeOS market fixture"
OutFile "C:\NSISTests\market-setup.exe"
RequestExecutionLevel user
SilentInstall silent
InstallDir "C:\NSISTests\market-result"
Section
  SetOutPath "$INSTDIR"
  File "C:\NSISTests\payload.txt"
  IntOp $0 40 + 2
  FileOpen $1 "$INSTDIR\result.txt" w
  FileWrite $1 "MARKET42=$0$\r$\n"
  FileClose $1
SectionEnd
