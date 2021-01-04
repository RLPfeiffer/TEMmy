#SingleInstance, Force
#NoEnv
SendMode Input
SetWorkingDir, %A_ScriptDir%

#Include, util.ahk

If (A_ScriptName = "book-thief.ahk") {
    InputBox,title,title
    TakeScreenshots(title)
}