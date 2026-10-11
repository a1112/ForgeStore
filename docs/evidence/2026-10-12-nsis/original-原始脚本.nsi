Unicode true
Name "ForgeOS original fixture"
OutFile "C:\NSISTests\original-setup.exe"
RequestExecutionLevel user
SilentInstall silent
InstallDir "C:\NSISTests\original-result"
Section
  SetOutPath "$INSTDIR"
  File "C:\NSISTests\payload.txt"
  IntOp $0 21 + 21
  FileOpen $1 "$INSTDIR\result.txt" w
  FileWrite $1 "ORIGINAL42=$0$\r$\n"
  FileClose $1
SectionEnd
