@echo off

set ROOT=%~dp0
set SRCDIR=%ROOT%src
set BINDIR=%ROOT%bin
set OBJDIR=%ROOT%obj

if not exist %BINDIR% mkdir %BINDIR%

rustc -o %BINDIR%\eyja.exe %SRCDIR%/eyja/main.rs