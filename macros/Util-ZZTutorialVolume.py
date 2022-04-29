# This file is named with a Z so it comes after every other Python function in Util files is defined

def MainVolumeSteps(investigator:str, name:str, radius:int, detailed:bool, recap:bool) -> list[Step]:
    FocusSteps = DetailedFocusSteps if detailed else FastFocusSteps
    
    return [
        OpenLastSnapshot(recap, investigator, name, 150),
        TellOperatorSEM("Locate the center point at 150x, click it, and click 'Add Marker' in the navigator window."),
        DoAutomatically(lambda: MoveToNavItem(PolygonIndex)),
        DoAutomatically(Record),
        ManuallyCheckCenterPoint,
        DoAutomatically(lambda: TakeSnapshotWithNotes("", False))
    ] + SwitchToHighMagSteps(recap, investigator, name, 600, HighMag600, SpotSize2, True, True, []) + SwitchToHighMagSteps(recap, investigator, name, 2000, HighMag2000, SpotSize1, False, True, FocusSteps) + [
        TellOperatorSEM(f"In the menubar, click Navigator -> Montaging and Grids -> Add Circle Polygon. Type {radius}"),
        TellOperatorSEM("In the navigator window, delete every item EXCEPT FOR the formvar reference point and the circle Polygon."),
        DoAutomatically(lambda: SetMagIndex(HighMag5000)),
        TellOperatorSEM("With the circle polygon selected, check the Navigator checkboxes for 'Aquire', 'New File At Item', 'Montaged Images', 'Fit Montage to Polygon'. Make sure 'Go from center out and anchor at 2000x' is active and click ok. Then select the generated idoc file. Choose to overwrite it."),
        DoAutomatically(lambda: MoveToNavItem(PolygonIndex)),
        DoAutomatically(ScreenDown)
    ] + FocusSteps + FinalSteps(detailed, False, recap)


Steps["RC3"] = NewSpecimenSteps + LowMagCookSteps + MainVolumeSteps("Jones", "RC3", 125, True, False)
Steps["RC3 Recapture"] = NewSpecimenSteps + MainVolumeSteps("Jones", "RC3", 125, True, True)

Steps["RC3 Fast"] = NewSpecimenSteps + LowMagCookSteps + MainVolumeSteps("Jones", "RC3", 125, False, False)
Steps["RC3 RecapFast"] = NewSpecimenSteps + MainVolumeSteps("Jones", "RC3", 125, False, True)

