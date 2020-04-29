#NoEnv  ; Recommended for performance and compatibility with future AutoHotkey releases.
; #Warn  ; Enable warnings to assist with detecting common errors.
SendMode Input  ; Recommended for new scripts due to its superior speed and reliability.
SetWorkingDir %A_ScriptDir%  ; Ensures a consistent starting directory.

ConfirmScreenPosition(ByRef x, ByRef y, file, description) {
    if !FileExist(file) {
        MsgBox, "Please move the mouse to the current position of %description% and press Shift."
        KeyWait, LShift, D
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
        MsgBox, 4,, "Is this the correct position for %description%?"
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

ConfirmScreenPosition(x, y, "FOO", "the foo button")
ConfirmScreenPosition(x, y, "BAR", "the bar button")