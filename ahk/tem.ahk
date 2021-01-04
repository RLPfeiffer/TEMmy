#Include, util.ahk

SectionNum := ""
Screenshot1 := ""
Screenshot2 := ""
Screenshot3 := ""
GridNum := ""
RodNum := ""
Investigator := ""
Series := ""
Capturer := ""

; Create the popup menu by adding some items to it.
Menu, MyMenu, Add, Set Section Info, SetSectionInfo

Menu, MyMenu, Add  ; Add a separator line below the first option.

; Create another menu destined to become a submenu of the above menu.
Menu, Submenu1, Add, Save 150x screenshot, TakeScreenshot1
Menu, Submenu1, Add, Save 600x screenshot, TakeScreenshot2
Menu, Submenu1, Add, Save 2000x screenshot, TakeScreenshot3

; Create a submenu in the first menu (a right-arrow indicator). When the user selects it, the second menu is displayed.
Menu, MyMenu, Add, RC3 Center Points, :Submenu1

Menu, MyMenu, Add  ; Add a separator line below the submenu.
Menu, MyMenu, Add, Type the section folder, TypeSectionFolder  ; Add another menu item beneath the submenu.
Menu, MyMenu, Add, Add section info to notes, TypeNotes ; Add another menu item beneath the submenu.
return  ; End of script's auto-execute section.

SetSectionInfo:
InputBox, SectionNum, Section #, Enter the section number:
; THE COMPUTER MUST HAVE Y: mapped to \\OpR-Marc-RC2\Data\Dropbox
Screenshot1 := "Y:/RC3 Center Points/RC3 " . SectionNum . " 150x.png"
Screenshot2 := "Y:/RC3 Center Points/RC3 " . SectionNum . " 600x.png"
Screenshot3 := "Y:/RC3 Center Points/RC3 " . SectionNum . " 2000x.png"
InputBox, GridNum, Grid #, Enter the grid number:
InputBox, RodNum, Rod #, Enter the rod number:
InputBox, Investigator, Investigator, Enter the investigator:, , , , , , , , Jones
InputBox, Series, Series, Enter the experiment series:, , , , , , , , RC3
InputBox, Capturer, Capturer, Enter the capturer:, , , , , , , , Nat Nelson
return

TakeScreenShot1:
TakeScreenshot(Screenshot1)
return

TakeScreenShot2:
TakeScreenshot(Screenshot2)
return

TakeScreenShot3:
TakeScreenshot(Screenshot3)
return

TypeSectionFolder:
LoopTimes := 4 - StrLen(SectionNum)
Loop %LoopTimes% {
    Send 0
}
Send, %SectionNum%
return

TypeNotes:
Send, %SectionNum%
Send {Tab}
Send, %GridNum%
Send {Tab}
Send ^A
Send, %RodNum%
Send {Tab}
Send, %Investigator%
Send {Tab}
Send, %Series%
Send {Tab}
Send, %Capturer%
return

#z::Menu, MyMenu, Show  ; i.e. press the Win-Z hotkey to show the menu.