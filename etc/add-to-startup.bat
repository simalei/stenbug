@echo off
set /p "stenbug=Enter path to the Stenbug executable: "
reg add HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Run /v Stenbug /t REG_SZ /d %stenbug% /f
echo Stenbug should be set to startup automatically
echo You can disable auto startup through Windows Task Manager
pause