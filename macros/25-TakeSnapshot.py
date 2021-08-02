Label = EnterString("section # and label")
Slack = YesNoBox("send snapshot to slack?")
TakeSnapshot(Slack, Label)