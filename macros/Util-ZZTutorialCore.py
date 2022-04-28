# This file is named with a Z so it comes after every other Python function in Util files is defined

def MainCoreSteps(detailed:bool, recap:bool) ->list[Step]:
    FocusSteps = DetailedFocusSteps if detailed else FastFocusSteps
    return [
        TellOperatorSEM("Locate the Region of Interest at 150x (you may need to move the stage). Click 'Add Points' in the navigator window and click on the corners of the ROI. Then click 'Stop Adding Points'."),
        TellOperatorSEM("Select each of the corner points and type 'C' on the keyboard to mark them as corner points."),
        DoAutomatically(lambda: TakeSnapshotWithNotes("corners", False))
    ] + SwitchToHighMagSteps("", "", 2000, HighMag2000, 1, True, False, []) + [
        DoAutomatically(lambda: SetMagIndex(HighMag5000))
    ] + FocusSteps + [
        TellOperatorSEM("For each corner point, click 'Go to XY' and take a recording. If the point is not where you expect it to be, zoom out with the scrollbar to see its position relative to the other corners, then zoom back in and use 'Search' to find its real position. Then use 'Move item' to move the point."),
        TellOperatorSEM("In the menubar, click Navigator -> Montaging and Grids -> Polygon from Corners. Zoom out to make sure the generated polygon is your intended shape, then delete the corner points."),
        TellOperatorSEM("With the polygon selected, check the Navigator checkboxes for 'Aquire', 'New File At Item', 'Montaged Images', 'Fit Montage to Polygon'. Make sure 'Go from center out and anchor at 2000x' is NOT active and click ok. Then select the generated idoc file. Choose to overwrite it."),
        DoAutomatically(lambda: MoveToNavItem(PolygonIndex)),
        DoAutomatically(ScreenDown)
    ] + FocusSteps + FinalSteps(detailed, True, recap)

Steps["Core"] = NewSpecimenSteps + LowMagCookSteps + MainCoreSteps(True, False)
Steps["Core Recapture"] = NewSpecimenSteps + MainCoreSteps(True, True)

Steps["Core Fast"] = NewSpecimenSteps + LowMagCookSteps + MainCoreSteps(False, False)
Steps["Core RecapFast"] = NewSpecimenSteps + MainCoreSteps(False, True)