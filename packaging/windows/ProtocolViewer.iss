#ifndef AppVersion
  #define AppVersion "0.1.0"
#endif
#ifndef SourceDir
  #define SourceDir "..\..\target\release"
#endif
#ifndef OutputDir
  #define OutputDir "..\..\dist"
#endif

[Setup]
AppId={{6D60FCF2-9674-4BA5-8D11-17E27B945CA2}
AppName=Protocol Viewer
AppVersion={#AppVersion}
AppPublisher=zerojacks
DefaultDirName={localappdata}\Programs\Protocol Viewer
DefaultGroupName=Protocol Viewer
DisableProgramGroupPage=yes
PrivilegesRequired=lowest
OutputDir={#OutputDir}
OutputBaseFilename=protocol-viewer-windows-x86_64-{#AppVersion}-setup
SetupArchitecture=x64
ArchitecturesInstallIn64BitMode=x64
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
UninstallDisplayIcon={app}\protocol-viewer.exe

[Tasks]
Name: "desktopicon"; Description: "创建桌面快捷方式"; GroupDescription: "附加快捷方式："; Flags: unchecked

[Files]
Source: "{#SourceDir}\protocol-viewer.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\Protocol Viewer"; Filename: "{app}\protocol-viewer.exe"
Name: "{autodesktop}\Protocol Viewer"; Filename: "{app}\protocol-viewer.exe"; Tasks: desktopicon

[Run]
Filename: "{app}\protocol-viewer.exe"; Description: "启动 Protocol Viewer"; Flags: postinstall nowait skipifsilent
