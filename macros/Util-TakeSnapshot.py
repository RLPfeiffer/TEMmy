import sys

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