from typing import Optional
def LowMagCook(Minutes:Optional[float]=None, RecordAfter:bool=True) -> None:
   SecondsToCook:float = 60
   if Minutes is None:
      SecondsToCook *= EnterDefaultedFloat(7, "Number of minutes to cook?")
   else:
      SecondsToCook *= Minutes 

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

def WaitForStageNotBusy() -> None:
   while sem.ReportStageBusy() != 0:
      sem.Delay(500, "msec")