from threading import Thread

SampleInfoThread = Thread(target=PromptForSampleInfo)
SampleInfoThread.start()

NumNavItems = int(sem.ReportNumTableItems())
if NumNavItems == 0:
   LowMagCook(7)
else:
   for idx in range(1, NumNavItems+1):
      sem.MoveToNavItem(idx)
      LowMagCook(7, False) # cook without dimming beam and recording after.
   sem.SetSpotSize(3)
   sem.Record() # Record after the last cook

SampleInfoThread.join()