#!/bin/bash
# Simple test application for Bryndza testing
# This script creates a basic GUI window for testing purposes

if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    osascript -e 'tell application "TextEdit" to activate'
    osascript -e 'tell application "System Events" to tell process "TextEdit" to keystroke "n" using command down'
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux
    which gedit >/dev/null 2>&1 && gedit &
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
    # Windows
    notepad.exe
fi
