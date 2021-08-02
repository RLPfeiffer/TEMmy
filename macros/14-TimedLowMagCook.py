#!Python3.9
#MacroName Timed Low Mag Cook - Python
import serialem as sem

def LowMagCook(Minutes=None, RecordAfter=True):
   if Minutes == None:
      Minutes = sem.EnterDefaultedNumber(7, 2, "Number of minutes to cook?")

   SecondsToCook = Minutes * 60

   MinWobbleDist = 10
   MaxWobbleDist = 20
   WobbleDist = 20
   
   XStart, YStart, _ = sem.ReportStageXYZ()

   print("BEGIN BURN WOBBLE")
   sem.ResetClock()

   while sem.ReportClock() < SecondsToCook:
      if WobbleDist > MaxWobbleDist:
         WobbleDist = MinWobbleDist
      
      # Arrays of stage position deltas:
      ShiftXList = [WobbleDist, -WobbleDist, -WobbleDist, WobbleDist, 0, 0, 0, 0]
      ShiftYList = [0, 0, 0, 0, WobbleDist, -WobbleDist, -WobbleDist, WobbleDist]

      for ShiftX, ShiftY in zip(ShiftXList, ShiftYList):
         sem.MoveStage(ShiftX, ShiftY)
         WaitForStageNotBusy()
         if sem.ReportClock() >= SecondsToCook:
            break
   
      sem.MoveStageTo(XStart, YStart)
      sem.ReportStageXYZ()
      sem.ReportClock()

      WobbleDist += 1

   print("END BURN WOBBLE")
   if RecordAfter:
      sem.SetSpotSize(3)
      sem.Record()

def WaitForStageNotBusy():
   while sem.ReportStageBusy() != 0:
      sem.Delay(500, "msec")