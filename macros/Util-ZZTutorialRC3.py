# This file is named with a Z so it comes after every other Python function in Util files is defined

def OpenLastRC3Snapshot(mag:int) -> Step:
    def step() -> None:
        try:
            startfile(glob(join(DropboxPath, "TEMSnapshots", f"Jones RC3 * x{mag} *.jpg"))[-1])
            RunNextStep()
        except:
            TellOperator(f"Open DROPBOX/TEMSnapshots and open the latest RC3 snapshot at x{mag}")
    return step

def MainRC3Steps(detailed:bool) -> list[Step]:
    FocusSteps = DetailedFocusSteps if detailed else [FastFocusStep]

    return [
        OpenLastRC3Snapshot(150),
        TellOperatorSEM("Locate the center point at 150x, click it, and click 'Add Marker' in the navigator window."),
        DoAutomatically(lambda: MoveToNavItem(PolygonIndex)),
        DoAutomatically(Record),
        DoAutomatically(lambda: TakeSnapshotWithNotes("", False))
    ] + SwitchToHighMagSteps + FocusSteps + [
        DoAutomatically(Record),
        OpenLastRC3Snapshot(2000),
        TellOperatorSEM("Find the center point at 2000x, and click it. Then delete the last navigator item."),
        DoAutomatically(lambda: TakeSnapshotWithNotes("", False)),
        TellOperatorSEM("In the menubar, click Navigator -> Montaging and Grids -> Add Circle Polygon. Type 125"),
        DoAutomatically(lambda: SetMagIndex(HighMag5000)),
        TellOperatorSEM("With the circle polygon selected, check the Navigator checkboxes for 'Aquire', 'New File At Item', 'Montaged Images', 'Fit Montage to Polygon'. Make sure 'Go from center out and anchor at 2000x' is active and click ok. Then select the generated idoc file. Choose to overwrite it."),
        DoAutomatically(lambda: MoveToNavItem(PolygonIndex)),
        DoAutomatically(ScreenDown)
    ] + FocusSteps + [
        DoAutomatically(Autofocus),
        DoAutomatically(Record),
        TellOperatorSEM("If the focus looks good, click 'Next Step'. If not, redo the focus, click 'Autofocus', then 'Record.' Keep doing this until it looks good."),
        TellOperatorSEM("If the green number representing the circle's center has shifted from where you put it, use 'Move item' to fix it, then click 'Stop Moving.'"),
    ]


Steps["RC3"] = NewSpecimenSteps + LowMagCookSteps + MainRC3Steps(True) + [ ChooseMontageMacro ] + AfterMontageSteps 
Steps["RC3 Recapture"] = NewSpecimenSteps + MainRC3Steps(True) + [ UseRecaptureMacro ] + AfterMontageSteps

Steps["RC3 Fast"] = NewSpecimenSteps + LowMagCookSteps + MainRC3Steps(False) + [ ChooseMontageMacro ] + AfterMontageSteps 
Steps["RC3 RecapFast"] = NewSpecimenSteps + MainRC3Steps(False) + [ UseRecaptureMacro ] + AfterMontageSteps

