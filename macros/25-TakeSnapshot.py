Sample = ""
CurrentNotes = CurrentSampleNotes()
if CurrentNotes is None:
    PromptForSampleInfo()
    CurrentNotes = CurrentSampleNotes()

assert CurrentNotes is not None
if len(CurrentNotes) > 1:
    choices = []
    for key, _ in CurrentNotes.items():
        choices.append(key)
    while len(choices) < 3:
        choices.append("")
    Sample = ThreeChoiceBox("Which sample are you snapshotting?", choices[0], choices[1], choices[2])
else:
    for key, _ in CurrentNotes.items():
        Sample = key

Investigator = CurrentNotes[Sample][SampleInfoKeys.index("Investigator")]
Experiment = CurrentNotes[Sample][SampleInfoKeys.index("Experiment")]

Label = EnterString("label")
Slack = YesNoBox("send snapshot to slack?")
TakeSnapshot(Slack, f"{Investigator} {Experiment} {Sample} {Label}")