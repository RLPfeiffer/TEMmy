import os

def SendMessage(Text:str) -> None:
   MessageDir = sem.GetVariable("V2MessageDir")
   MessagePath = sem.GetVariable("V2MessagePath")
   # Make sure there is a message file to append to
   os.makedirs(MessageDir, exist_ok=True)
   # Append to the file so multiple messages aren't overwritten
   with open(MessagePath, "a+") as MessageFile:
      MessageFile.write("{}{}".format(Text, os.linesep))

def SendStart() -> None:
   EstimatedCaptureHours = 0
   NumTiles = 0
   SecondsPerTile = sem.GetVariable("SecondsPerTile")
   ScopeName = sem.GetVariable("ScopeName")

   if sem.ReportNumTableItems() < 1:
      print("Nav table does not have items indicating total capture time.")
   else:
      NumTiles = sem.ReportNumMontagePieces(1)
      
      if NumTiles == 1:
         print("Montage claims to have only one image. Using 1 hour as timeframe")
         EstimatedCaptureHours = 1
      else:
         EstimatedCaptureTime = NumTiles * SecondsPerTile
         EstimatedCaptureHours = EstimatedCaptureTime / (60 * 60)
      print("Estimating {} hours to complete".format(EstimatedCaptureHours))

      SendMessage("Started: Capturing {} images on {}. Estimating {} hours to complete".format(NumTiles, ScopeName, EstimatedCaptureHours))

def SendStop() -> None:
   SendMessage("Copied: {} copied from {} to DROPBOX.".format(sem.GetVariable('CaptureDir'), sem.GetVariable('ScopeName')))