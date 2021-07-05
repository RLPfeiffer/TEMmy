#Include, CaptureScreen().ahk
#NoEnv  ; Recommended for performance and compatibility with future AutoHotkey releases.
; #Warn  ; Enable warnings to assist with detecting common errors.
SendMode Input  ; Recommended for new scripts due to its superior speed and reliability.
SetWorkingDir %A_ScriptDir%  ; Ensures a consistent starting directory.

CoordMode, Mouse, Screen

; Test when run directly:
If (A_ScriptName = "util.ahk") {
    TakeScreenshots("hey")
    ; ConfirmScreenPosition(x, y, "FOO", "the foo button")
    ;ConfirmScreenPosition(x, y, "BAR", "the bar button")
}

; Source: https://www.autohotkey.com/boards/viewtopic.php?p=129052#p129052
WaitForShift(message) {
    Gui,help:Add, Text,     , %message%
    Gui,help:+toolwindow
    Gui,help:Show,x0 y0
    KeyWait, LShift, D
    ; Wait for release in case another screen position tries to confirm itself on the same input frame
    KeyWait, LShift, U
    Gui, help: Destroy
}

WinMoveMsgBox() {
    SetTimer, WinMoveMsgBox, OFF
    ID:=WinExist("Confirm position")
    WinMove, ahk_id %ID%, , 50, 50 
}

ConfirmScreenPosition(ByRef x, ByRef y, file, description) {
    if !FileExist(file) {
        WaitForShift("Please move the mouse to the current position of " . description . " and press Shift.")
        MouseGetPos, x, y
        fileOutput := x . "," . y
        fileObj := FileOpen(file, "w")
        fileObj.write(fileOutput)
        fileObj.close()
    }
    else {
        fileObj := FileOpen(file, "r")
        savedValue := fileObj.readLine()
        coords := StrSplit(savedValue, ",")
        x := coords[1]
        y := coords[2]
        MouseMove, x, y
        SetTimer, WinMoveMsgBox, 50
        MsgBox, 4,Confirm position, "Is this the correct position for %description%?"
        IfMsgBox, Yes 
        {
            fileObj.close()
        } else {
            fileObj.close()
            FileDelete, %file%
            ConfirmScreenPosition(x, y, file, description)
        }
    }
}

TakeScreenshot(File) {
    ConfirmScreenPosition(Left, Top, "TOPLEFT", "Top left corner of the recording")
    ConfirmScreenPosition(Right, Bottom, "BOTTOMRIGHT", "Bottom right corner of the recording")
    CaptureScreen(Left . ", " . Top . ", " . Right . ", " . Bottom,,File)
}

TakeFullScreenshot(File) {
    CaptureScreen(0,,File)
}

TakeScreenshots(Prefix) {
    ConfirmScreenPosition(Left, Top, "TOPLEFT", "Top left corner of the recording")
    ConfirmScreenPosition(Right, Bottom, "BOTTOMRIGHT", "Bottom right corner of the recording")
    ConfirmScreenPosition(ClickX, ClickY, "CLICKHERE", "Where to click for the next page")
    InputBox, pages, Number of pages
    InputBox, milli1, Milliseconds for screen capture
    InputBox, milli2, Milliseconds for page load
    Sleep, %milli2%
    Loop %pages% {
        Number = %A_Index%
        PaddingNeeded := StrLen(pages) - StrLen(Number)
        Number := A_Index
        Loop %PaddingNeeded% {
            Number := "0" . Number
        }

        CaptureScreen(Left . ", " . Top . ", " . Right . ", " . Bottom,,Prefix . Number . ".png")
        Sleep, %milli1%
        MouseMove, ClickX, ClickY
        MouseClick, left
        Sleep, %milli2%
    }
}