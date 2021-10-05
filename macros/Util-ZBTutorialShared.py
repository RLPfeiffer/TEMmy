# This file is named with a Z so it comes after every other Python function in Util files is defined

RodChangeSteps:list[Step] = [
    DoAutomatically(TurnOffFilament),
    TellOperator("Pull the rod handle straight out until it stops, then twist it counter-clockwise until it stops."),
    TellOperator("One more time, pull the rod handle straight out until it stops, then twist it counter-clockwise until it stops. Then flip the PUMP switch to AIR."),
    TellOperator("After flipping the PUMP switch to AIR, V16 and V18 in TEM Center will turn green. Wait for them to turn back to gray."),
    TellOperator("Pull the rod all the way out, and go put the next grid in it."),
    TellOperator("Line up the knob on the rod with the opening in the specimen chamber. Push the rod in until it stops."),
    TellOperator("Keep your hand on the rod, and flip the switch from AIR to PUMP."),
    TellOperator("A yellow light should be on. Wait for the specimen chamber to pump down and turn Green in TEM Center."),
    TellOperator("Twist the rod handle clockwise and allow it to be pulled inward. Twist clockwise again and the rod should go in all the way."),
    DependingOnScope(DoNothing, TellOperator("Wait for the Penning Gauge to turn on (Green) and stabilize below 30.")),
]

NewSpecimenSteps:list[Step] = [
    DependingOnYesNo("Is the sample already loaded in the scope?", SkipSteps(RodChangeSteps), RunNextStep)
] + RodChangeSteps + [
    # TODO go to low mag 150x automatically. Only prompt to change aperture
    TellOperator("Go to low mag 150x with no aperture inserted."),
    # TODO on TEM2, coach a camera insertion workaround to avoid penning gauge spike with filament on?
    DoAutomatically(TurnOnFilament),
    DoAutomatically(ScreenDown),
    TellOperator("Scroll the stage to find a region of formvar, and click 'Add Stage Pos' in the navigator window."),
]

LowMagCookSteps:list[Step] = [
    DoAutomatically(lambda: SetSpotSize(1)),
    TellOperator("Scroll the stage to the center of the tissue. Remove the mirror. Center and tighten the beam to the inner brackets. Then click 'Next Step' and wait for 7 minutes."),
    DoAutomatically(lambda: LowMagCook(7)),
]

AfterMontageSteps:list[Step] = [
    DependingOnYesNo("Does the overview look good enough to move onto new tissue?", DoAutomatically(TurnOffFilament), TellOperator("When selecting your next tutorial, choose the 'recapture' variation and do not switch samples.")),
    TellOperator("Close SerialEM. Click 'No' when it asks you whether to save anything. Then open SerialEM again and click 'Start Tutorial' for your next capture.")
]

AcquireAtItemsMessage = "In the menubar, click Navigator -> AcquireAtItems. Choose '*'. Leave FilamentManager selected, and click OK. Then move the mirror out of the way."

SwitchToHighMagSteps:list[Step] = [
    DoAutomatically(lambda: SetBeamBlank(True)),
    # TODO go to high mag 2000x automatically
    TellOperator("Go to high mag 2000x with the second aperture inserted."),
    DependingOnScope(TellOperator("Spread the beam by several turns (by turning the 'brightness' knob clockwise.)"), DoNothing),
    DoAutomatically(ScreenDown),
    TellOperator("Use the aperture X/Y dials to center the aperture.")
]

FocusStep:Step = TellOperator("Tighten the beam, center it, and use image wobble and the focus knob to adjust focus. Make sure the beam is spread around 100 Current Density, and click Next Step.")

ChooseMontageMacro:Step = DependingOnYesNo("Are there any holes in the section or formvar?", TellOperator(AcquireAtItemsMessage.replace("*", "CalibrateAndRecapturePy")), TellOperator(AcquireAtItemsMessage.replace("*", "HighMagCookPy")))
UseRecaptureMacro:Step = TellOperator(AcquireAtItemsMessage.replace("*", "CalibrateAndRecapturePy"))
