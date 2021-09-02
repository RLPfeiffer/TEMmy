import sys
from typing import Optional

# Function for automated use: saves a snapshot of the current image to our DROPBOX.
# If called with True for the first argument, will also send this image to slack in the #tem-bot channel
def TakeSnapshot(SendToSlack:bool, Name:str, Overview:bool=False) -> None:
   # Reports the current mag; also sets reportedValue2 to 1 if low mag mode, 0 if not
   (CurrentMag, LowMag) = sem.ReportMag()
   Filename = f"TEMSnapshots/{Name} x{int(CurrentMag)} {ScopeName}.jpg"
   FilenameInDropbox = f"{DropboxPath}/{Filename}"
   try:
      if Overview:
         sem.SaveToOtherFile("B", "JPG", "CUR", FilenameInDropbox)
      else:
         sem.SnapshotToFile(0, 0, "0", "JPG", "CUR", FilenameInDropbox)
   except:
      SendMessage(f"TakeSnapshot() failed for snapshot {Filename} with {sys.exc_info()[0]}")
      return

   if SendToSlack:
      SendMessage(f"Snapshot: {Filename}")

# Function for manual use: Prompts for sample notes and takes a snapshot
def TakeSnapshotWithNotes(Label:Optional[str] = None, Slack:Optional[bool] = None) -> None:
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

   if Label is None:
      Label = EnterString("label")
   if Slack is None:
      Slack = YesNoBox("send snapshot to slack?")
   TakeSnapshot(Slack, f"{Investigator} {Experiment} {Sample} {Label}")