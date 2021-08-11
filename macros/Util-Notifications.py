import os

def SendMessage(Text:str) -> None:
   # Make sure there is a message file to append to
   os.makedirs(MessageDir, exist_ok=True)
   # Append to the file so multiple messages aren't overwritten
   with open(MessagePath, "a+") as MessageFile:
      MessageFile.write(f"{Text}{newline}")

def SendStart() -> None:
   EstimatedCaptureHours:float = 0
   NumTiles = 0

   if sem.ReportNumTableItems() < 1:
      print("Nav table does not have items indicating total capture time.")
   else:
      NumTiles = NumMontageTiles()
      
      if NumTiles == 1:
         print("Montage claims to have only one image. Using 1 hour as timeframe")
         EstimatedCaptureHours = 1
      else:
         EstimatedCaptureTime = NumTiles * SecondsPerTile
         EstimatedCaptureHours = EstimatedCaptureTime / (60 * 60)
      print(f"Estimating {EstimatedCaptureHours} hours to complete")

      SendMessage(f"Started: Capturing {NumTiles} images on {ScopeName}. Estimating {EstimatedCaptureHours} hours to complete")

def SendStop(CaptureDir:str) -> None:
   SendMessage(f"Copied: {CaptureDir} copied from {ScopeName} to DROPBOX.")