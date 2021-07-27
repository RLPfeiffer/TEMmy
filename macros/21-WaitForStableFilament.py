#!Python3.9
#MacroName WaitForStableFilament - Python
import serialem as sem

def WaitForStableFilament():
    MaxPercentChangeOverCapture = float(sem.GetVariable("MaxPercentChangeOverCapture"))
    MaxPercentChangeOverCapturePercentage = MaxPercentChangeOverCapture * 100

    print("Checking for stable filament")
    print(f"Beam intensity must change less than {MaxPercentChangeOverCapturePercentage} percent over capture to be considered stable")

    EstimatedCaptureTime = 60 * 60

    NumNavItems = sem.ReportNumTableItems()
    if NumNavItems < 1:
        print("Nav table does not have items indicating total capture time. Using 1 hour as timeframe.")
    else:
        NumTiles = sem.ReportNumMontagePieces()

        if NumTiles == 1:
            print("SerialEM claiming only 1 image in montage item. Using 1 hour as timeframe.")
        else:
            EstimatedCaptureTime = NumTiles * float(sem.GetVariable("SecondsPerTile"))
            EstimatedCaptureHours = EstimatedCaptureTime / (60 * 60)
            print(f"Capturing {NumTiles} images. Estimating {EstimatedCaptureHours} hours to complete")

    sem.ResetClock()

    sem.Record()
    LastMeanCounts = sem.ReportMeanCounts()
    LastCaptureTimeStamp = sem.ReportClock()

    # Make the first check short in case the beam is stable.  If it isn't we'll see the greatest 
    # change in the beginning so we should still see the warmup change.

    # The beam has higher counts cold, and lower counts warm
    sem.Delay(60, "sec")

    for iLoop in range(22):
        sem.Record()
        MeanCounts = sem.ReportMeanCounts()
        CaptureTimeStamp = sem.ReportClock()

        TimeInterval = CaptureTimeStamp - LastCaptureTimeStamp
        PercentChangeOverTimeInterval = 1.0 - (MeanCounts / LastMeanCounts) 
        
        # If the beam is warm, the tolerance is low, and the capture is long then normal fluctuations can prevent
        # the test from passing. So we only look for counts dropping. If the counts increase we consider the filament
        # warm 
        # PercentChangeOverTimeInterval = ABS $PercentChangeOverTimeInterval 
        ReadablePercentChangeOverTimeInterval = PercentChangeOverTimeInterval * 100.0
        print(f"Measured {ReadablePercentChangeOverTimeInterval} percent change in counts over {TimeInterval} seconds")

        NumIntervalsOverCapture = EstimatedCaptureTime / TimeInterval
        # print("Estimating {} intervals".format(NumIntervalsOverCapture)) 
        PercentChangeOverCapture = abs(PercentChangeOverTimeInterval * NumIntervalsOverCapture)
        ReadablePercentChangeOverCapture = PercentChangeOverCapture * 100

        print(f"Estimating {ReadablePercentChangeOverCapture} percent change in image counts over entire capture.")
        if PercentChangeOverCapture < MaxPercentChangeOverCapture:
            print(f"Estimated change is below tolerance of {MaxPercentChangeOverCapturePercentage} percent. Filament is stable!")
            sem.ReportClock()
            break

        LastMeanCounts = MeanCounts
        LastCaptureTimeStamp = CaptureTimeStamp

        sem.Delay(120, "sec")