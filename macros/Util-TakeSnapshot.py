# Function for automated use: saves a snapshot of the current image to our DROPBOX.
# If called with True for the first argument, will also send this image to slack in the #tem-bot channel
def TakeSnapshot(SendToSlack, Name):
   # Reports the current mag; also sets reportedValue2 to 1 if low mag mode, 0 if not
   (CurrentMag, LowMag) = sem.ReportMag()
   Filename = f"TEMSnapshots/{Name} x{int(CurrentMag)} {sem.GetVariable('ScopeName')}.jpg"
   FilenameInDropbox = f"{sem.GetVariable('DropboxPath')}/{Filename}"
   # Takes a snapshot of the image display in the current active buffer and saves it to the file with the given name and closes the file.
   #  #S sets the amount of image scaling relative to the display with a value of 1 or greater, or 0 or less to take the whole image at 1:1 zoom.
   #  #T sets the scaling of text labels and line thicknesses with a value of 1 or more, or 0 or less to use the same scaling as for the image.
   #  Set feat non-zero to skip feature drawing, with the sum of 1 to skip some features and 2 to skip Navigator items.  typ specifies the
   # type of file and can be TIF, TIFF, JPG, JPEG, CUR, or -1 (case insensitive), where CUR or -1 means use the current file type 
   # selected in the Image Snapshots dialog.  cmp specifies the compression for a TIFF file and can be NONE,  LZW, ZIP, JPG, 
   # JPEG, CUR, or -1 (case insensitive), where CUR or -1 means use the current compression in the Image Snapshots dialog. 
   # If JPG or JPEG is entered for 'typ', a true JPEG file will be written, not a JPEG-compressed TIFF, and the compression 
   # entry does not matter (but must be present before the filename file). 
   # SnapshotToFile #S #T feat typ cmp file
   try:
      sem.SnapshotToFile(0, 0, "0", "JPG", "CUR", FilenameInDropbox)
   except:
      try:
         sem.SaveToOtherFile("B", "JPG", "CUR", FilenameInDropbox)
      except:
         SendMessage(f"SnapshotToFile and SaveToOtherFile both failed for snapshot {Filename}")
         return

   if SendToSlack:
      SendMessage(f"Snapshot: {Filename}")