# This file is named with a Z so it comes after every other Python function in Util files is defined

MainCoreSteps:list[Step] = [
    TellOperator("Locate the Region of Interest at 150x (you may need to move the stage). Click 'Add Points' in the navigator window and click on the corners of the ROI. Then click 'Stop Adding Points'."),
    TellOperator("Select each of the corner points and type 'C' on the keyboard to mark them as corner points."),
    DoAutomatically(lambda: TakeSnapshotWithNotes("corners", False))
] + SwitchToHighMagSteps + [
    DoAutomatically(lambda: SetMagIndex(HighMag5000)),
    FocusStep,
    TellOperator("For each corner point, click 'Go to XY' and take a recording. If the point is not where you expect it to be, zoom out with the scrollbar to see its position relative to the other corners, then zoom back in and use 'Search' to find its real position. Then use 'Move item' to move the point."),
    TellOperator("In the menubar, click Navigator -> Montaging and Grids -> Polygon from Corners. Zoom out to make sure the generated polygon is your intended shape, then delete the corner points."),
    TellOperator("With the polygon selected, check the Navigator checkboxes for 'Aquire', 'New File At Item', 'Montaged Images', 'Fit Montage to Polygon'. Make sure 'Go from center out and anchor at 2000x' is NOT active and click ok. Then select the generated idoc file. Choose to overwrite it."),
    DoAutomatically(lambda: MoveToNavItem(PolygonIndex)),
    DoAutomatically(ScreenDown),
    FocusStep,
    DoAutomatically(Autofocus),
    DoAutomatically(Record),
    TellOperator("If the focus looks good, click 'Next Step'. If not, redo the focus, click 'Autofocus', then 'Record.' Keep doing this until it looks good."),
]

Steps["Core"] = NewSpecimenSteps + LowMagCookSteps + MainCoreSteps + [ ChooseMontageMacro ] + AfterMontageSteps
Steps["Core Recapture"] = NewSpecimenSteps + MainCoreSteps + [ UseRecaptureMacro ] + AfterMontageSteps