TODO assert that the script is stateless (no * choices, no vars)
TODO (except for notes info)
TODO allow TEM1/TEM2 specified externally

Welcome to Temmy!
This is an interactive tutorial for running RC3 captures on TEM1 and TEM2.
Follow every step carefully. Click "Done" after finishing each step.
When there are multiple links, Click the link that describes what you see, and Temmy will tell you what to do next.
If anything seems to be going wrong, or is too hard, -> message_nat ->

+ Start morning captures
- First, go to TEM2. 

TODO skip low mag cook setup if recapturing
-> start_low_mag_cook ->

Now, go to TEM1.

-> start_low_mag_cook ->

You now have time to kill.
Click "OK" when TEM2 finishes its low mag cook.
+ OK
    Go to TEM2. 
    -> set_up_montage ->
    Go to TEM1
    -> set_up_montage ->

Both montages should send a "Started Capturing" message to Slack. Once they do, you're all done!

- -> END

== start_low_mag_cook
-> log_in ->
Next, -> remove_rod ->
Grab the sticky note. Take it, and the rod, with you to the counter where RC3 grids are kept.
-> switch_grids ->
Take the rod and sticky note back to the TEM.
-> insert_rod ->
Wait for the Penning Gauge to stabilize below 30. If you're on TEM2, it will start red and take some time to turn on.
-> turn_on_filament ->
On the right-side control panel, ress "Screen Up/Down" on TEM2 or "F2" on TEM1, to lower the screen.
Mag should be 150x, Spot size should be 3. Change it to 1, so you can see the sample better.
Use the scroll ball to find the center of the tissue.
Condense the beam to the inner brackets.
Click "Timed Low Mag Cook With Wobble" in SerialEM in the Camera & Script panel.
Press enter to choose 7 minutes.

->->

== set_up_montage

TODO do something if the camera insertion times out, temperature is not stable, etc.
Open the most recent 150x snapshot and compare it to the image taken by SerialEM.
Locate the center point and click on it. Right click and drag the image so the green marker is centered with the red cross.
Press "Record" in the Camera & Script panel.
Click the center point again.
Click "Snapshot". Type the section number and press enter. Do not send the snapshot to Slack unless there is something unusual about it.
Switch the aperture dial to the second white dot.
TODO clockwise/counterclockwise?
On the right-side control panel, press "Mag" on TEM2 or "Mag1" on TEM1.
On the right-side control panel, turn the "Magnification" dial counter-clockwise to go to mag 2000x.
Lower the screen. Center the beam. Focus.
Press Record.
Find the center point. Click on it.
Save a snapshot.
Navigator > Open
Navigator > Montaging and Grids > Add Circle Polygon. 125.
Lower the screen. Scroll to find formvar. Press Record.
TODO make sure it's a good formvar spot.
On the Navigator window, click "Add Stage Pos".
On the right-side control panel, turn the "Magnification" dial clockwise to go to mag 5000x.
On the Navigator window, click the circle polygon.
Click the checkbox for Acquire. Click the checkbox for "New File At Item".
In the window that pops up, click the radio button for "Montaged Images." Click the checkbox for "Fit Montage to Polygon."
Click OK.
Magnification in the next pop-up window should be set to 5000x.
Press OK.
TODO Select the IDOC place.
TODO make sure there is space for the capture.
In the navigator window, click "Go To XY."
Put the screen down. Center the beam. Focus. Spread beam to reach current density 250.
Autofocus. Press Record.
If the green number representing your polygon's center has shifted, click "Move Item" in the navigator window. Click back on the intended center point. Click "Stop Moving."
TODO check for gain referencable artifacts.
If the image appears, well-focused,
Navigator > Acquire at Items > HighMagCook/CalibrateAndRecapture
Run Script After: Beam Blank if someone will run another capture today, AutoWarmupFilament if someone will run a capture in the morning, or "Turn Off Filament" if it's the last capture on Friday.
Click "Go"
When a dialog box opens, type in the folder name where you put the IDOC.
Enter section notes. Press "Save."

->->


->->

== message_nat
Message Nat. 801-493-5262
If Nat does not get back to you quickly, -> beam_blank ->
If Nat never gets back to you, -> turn_off_filament ->
->->

== beam_blank
On TEM2, press the Beam Blank button on the left-side control panel.
On TEM1, press the F4 button on the right-side control panel.
->->

== turn_off_filament
Press the Filament "Off" button in TEM Center on the left-side computer. Wait for the "On" button to turn completely gray.
->->

== log_in
Wake up the computers by moving the mouse.
+ The right-side computer is logged out
    Click the mouse or press the spacebar.
    Log into the user account VALUEDGATANCUSTOMER with the password $admin
    ++ Done
+ The right-side computer is logged in.
- + A SerialEM window is open with an overview on it.
    ++ The overview has something very wrong with it
        -> message_nat
    ++ The overview looks good
        Close SerialEM. If it asks you, do not save anything.
        +++ Done
+ No SerialEM window
- Open SerialEM.

Check the TEM Center window on the left-side computer.
+ The Filament "On" button is green
    -> turn_off_filament ->
+ The Filament is off
- ->->

== remove_rod
Pull the rod outwards until it stops, and twist it to the left until it stops.
Pull it outwards again until it stops, and twist it to the left again until it stops.
+ Done
- Flip the switch below the rod from "Pump" to "Air."
On the Valve/Vacuum monitor, V16 and V18 will turn green. Wait for them to turn gray again.
+ Done
-> take_lid_off_holder ->
Swiftly but smoothly pull the rod all the way out.
Place the rod in the holder with the tip above the black cylinder. -> put_lid_on_holder ->
- ->->

== take_lid_off_holder
Open the rod's holder and put the lid aside.
+ Done
- ->->

== put_lid_on_holder
Put the lid on the holder and secure both buckles. Very carefully pull up on the handle to make sure the lid is secure before you attempt to move the holder.
+ Done
- ->->

== switch_grids
Set down the sticky note. Take the lid off the rod holder.
Note the section number and Grid slot from the sticky note.
Place the foam triangle on the black cylinder so the tip of the rod is above it.
Use the small flathead screwdriver to slightly untwist the screw on the tip of the rod.
Turn the rod 90 degrees to the side and tap the shaft with the screwdriver. The armature should swing open and the grid should fall onto the foam triangle.
+ Done
- Slide open the grid box where the grid belongs. Use forceps to pick the grid off the triangle and place it back in its slot.
Use the forceps to remove the next section and place it in the tip of the rod with the shiny side up.
- (check_grid) Twist the rod slightly back and forth and use the reflection of light to make sure the formvar is intact and a section is on it.
+ The formvar is broken/The section is missing
    Mark the grid slot with sharpie. Put the bad grid back in the slot, and put the next grid in the rod tip.
    ++ Done -> check_grid
+ The formvar is intact and the section looks good.
- Use the screwdriver to put the armature back in position over the grid. Screw it in all the way (but don't force it too tightly).
-> put_lid_on_holder ->
On the sticky note, cross out the section number and grid slot of the previous grid. Write down this info for the new grid.
->->

== insert_rod
Put the sticky note on the table where you won't lose it. Double-check that it has the section and grid slot number for the section in the rod.
-> take_lid_off_holder ->
The next part is easiest while standing up.
Hold the rod sideways and line the nub on the shaft up with the opening in the scope. Put the tip in carefully, then slide the rod in until the nub is inside and the rod stops going in.
+ Done
- Flip the switch below the rod from "Air" to "Pump".
TODO turn it left if it doesn't catch
Wait for Status of the Specimen Chamber to turn Green.
+ Done
- Twist the rod to the right until it stops. Push it inwards until it stops.
Twist it to the right again until it stops. Hold onto it lightly as it is sucked into the scope.

->->

== turn_on_filament

Click the filament "On" button. Wait for it to turn fully green.
+ Beam current number stayed the same
-> filament_dead
+ Beam current rose by about 15
->->

== filament_dead

The filament is probably dead.
-> message_nat