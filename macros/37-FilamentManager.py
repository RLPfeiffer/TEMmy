# Automatically warms up the filament in the mornings, blanks the beam in between captures, 
# and turns the filament off when no one is using the TEMs.
#################
# Configuration #
#################
# The times, on normal workdays, when the main TEM operator clocks in and out of the scope room:
from datetime import time, datetime, date, timedelta
from time import sleep
ClockInTime:time = time(hour=10) # 10 AM
ClockOutTime:time = time(hour=18) # 6 PM
# The maximum number of hours to leave the filament on
MaxUnattendedHours:float = 2
# The number of hours to warm the filament before the operator arrives on schedule
WarmupHours = 1

# This macro runs after every montage finishes.
# Step 1: Blank the beam
SetBeamBlank(True)
print("Beam is blanked")

# Step 2: Wait until the beam needs to be turned off (or the operator cancels FilamentManager to run another capture)

Now:datetime = datetime.now()
EndOfDay:datetime = datetime.combine(Now.date(), ClockOutTime)

UnattendedTimeoutSec:float = MaxUnattendedHours * 60 * 60
EndOfDayTimeoutSec:float = 0
if Now < EndOfDay and Now:
    EndOfDayTimeoutSec = (EndOfDay - Now).total_seconds()

TimeoutReason = "Unattended timeout" if UnattendedTimeoutSec < EndOfDayTimeoutSec else "End of day"
TimeoutSec = min(UnattendedTimeoutSec, EndOfDayTimeoutSec)
print(f"Waiting {TimeoutSec} seconds for {TimeoutReason}")

sleep(TimeoutSec)

# Step 3: Turn the beam off
TurnOffFilament()
print(f"Turning off beam for {TimeoutReason}")

# Step 4: Wait for warmup time of the next workday
Now = datetime.now()
WarmupTime = time(ClockInTime.hour-WarmupHours)
WarmupDateTime:datetime = datetime.combine(Now.date(), WarmupTime)
while WarmupDateTime < Now:
    WarmupDateTime += timedelta(days=1)
while WarmupDateTime.weekday() in [5, 6]:
    WarmupDateTime += timedelta(days=1)

WarmupStartSec = (WarmupDateTime - Now).total_seconds()
print(f"Waiting {WarmupStartSec} to warm up at {WarmupDateTime}")
sleep(WarmupStartSec)

# Step 5: Turn on the filament
print("Turning on filament for start of workday")
TurnOnFilament() # This always turns on beam blank

# Step 6: Wait for unattended timeout, and turn it off again
sleep(UnattendedTimeoutSec)
print(f"Turning off filament as it was left unattended for {MaxUnattendedHours} hours")
TurnOffFilament()