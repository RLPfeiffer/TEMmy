#!Python3.9
#MacroName TakeSnapshot - Python
#include Snapshot - Python
#include Notifications - Python
import serialem as sem

sem.EnterString("label", "section # and label")
Slack = sem.YesNoBox("send snapshot to slack?")
if int(Slack) == 1:
   Slack = True
else:
   Slack = False

TakeSnapshot(Slack, sem.GetVariable("label"))