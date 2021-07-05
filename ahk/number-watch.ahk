#SingleInstance, Force
SendMode Input
SetWorkingDir, %A_ScriptDir%

#Include, util.ahk

If (A_ScriptName = "number-watch.ahk") {
    InputBox, DataFile,,"File for data?"
    ConfirmScreenPosition(Left, Top, "TOPLEFT", "Top left corner of the readout to monitor")
    ConfirmScreenPosition(Right, Bottom, "BOTTOMRIGHT", "Bottom right corner of the readout to monitor")

    ; read the number 1 time a second if possible
    MsgBox,Press Ctrl+C to stop recording data
    SetTimer, ReadNumber, 1000
}

ReadNumber:
    FormatTime, TimeVar,, HH:mm:ss
    RunWait, %ComSpec% /c Capture2Text_CLI --screen-rect "%Left% %Top% %Right% %Bottom%" --output-format "%TimeVar%${tab}${capture}${linebreak}" --whitelist "0123456789." --output-file %DataFile% --output-file-append,, Hide
    ; MsgBox,%Command%
    
    ; --output-format ${capture}   : OCR Text.
    ;                                 ${linebreak} : Line break (\r\n).
    ;                                 ${tab}       : Tab character.
    ;                                 ${timestamp} : Time that screen or each
    ;                                 file was processed.
    ;                                 ${file}      : File that was processed or
    ;                                 screen rect.

; Ctrl+c stops recording
^c::
    SetTimer, ReadNumber, OFF
    MsgBox,Done recording data
    return
