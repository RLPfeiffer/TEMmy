// JavaScript functions
EXTERNAL showImage(file)

// TEM1/TEM2 variables
VAR current_density = 0
VAR capture_estimate = 0

// Capture/state variables
VAR recapture = false
VAR next_section = true

-> start

// Dummy functions needed to make Inky play nice:
== function showImage(file)
~ return 0

== tem1_setup
// Set TEM1-specific variables
~ current_density = 250
~ capture_estimate = 4.5

->->

== tem2_setup
// Set TEM2-specific variables
~ current_density = 65
~ capture_estimate = 4.5

->->

// Protocol
== start

~ showImage("temmy.png")
Welcome to TEMmy, the Electron Microscope tutor named after Toby Fox's adorable character from Undertale.

First of all, which electron microscope are you using?

* TEM1
-> tem1_setup ->
* TEM2
-> tem2_setup ->
- Great.

Check HT Voltage.

* HT Voltage is off (red)
    If HT is 80 you can just turn it on again.
    ** Done
* HT Voltage is on, but less than 80.
    Someone is in the middle of increasing the HT Voltage, probably because they installed a new filament. See the filament change protocol.
    ** Go to filament change tutorial -> filament_change
* HT Voltage is on (green) and at 80.

- (new_capture) If Serial EM is open from a previous capture, look at its overview.

Does the overview look good enough to annotate?
* No, it has artifacts which caused the tiles to align poorly
    You'll have to recapture it.
    ~ recapture = true
    ~ next_section = false
* No, the section broke while capturing.
    Open the excel file at \\MarcLab\Data\RC3 Materials\Section_Directory.
    Find the row for the section that broke and make a note in the capture notes column.
    ** Done

* No, whole portions of the polygon are black.
    The filament either blew out or turned off during the capture. Try turning it on again. See if it stays on.
    ** It turns off again by itself
        The filament is blown.
        *** Go to filament change protocol -> filament_change
**it stays on.
    The filament may have turned off because of an arcing event. Continue this set up as a recapture.
    ~ recapture = true
    ~ next_section = false
* Yes

- { next_section:
    -> put_next_section ->
}
* HT is at 80, and turned on.
- {tem2_setup: The Penning Gauge will take a little while to turn on and stabilize.}
  {tem1_setup: The Penning Gauge may burp a little, but should be stable and below 30.}
* Penning Gauge is on and stable below 30.
- Turn on the filament.
Open SerialEM.
Make sure the screen is raised.
Hit the low mag button. The mag in TEM Center should go to 150X.
Turn the aperture dial to the red dot.
* These are all done
- Wait until the filament is fully on (the button should be bright green and stop blinking).
Then lower the screen. (This is the {tem1_setup: F1|Screen Up} button on the right-hand physical control panel.)
Examine the section for damage or large dirt.
* There is large dirt on the sample
    If the dirt is bad enough, It may be worth having the stainer try to clean it first. To do this,
* There is damage/holes in the formvar near the sample
TODO stuff
* It looks good
- Locate the center of the tissue (move using the scroll ball on the left-hand control panel if necessary).

Do you need to run low mag cook?

* No
    Skipping, then.
* Yes
    -> low_mag_cook ->

- -> find_center_150x ->

Open Navigator. Hit add points, and place a point at the center point.
Stop adding points. Go to XY (or manually move the sample until the center point is centered on the SerialEM capture.

* Done
- Raise the screen. 
Turn the aperture to the {tem1_setup: first|second} white dot (aperture 1, largest white dot on dial).
Hit the mag 1 button.

Go to 2000X in SerialEM (TODO or is it TEM Center for this?).

Recenter the aperture using the dials on the aperture (the other dials on the piece where you just changed to the white dot).

Recenter the beam using the {tem1_setup: green dials|XY dials} on the left and right control panels.

Delete the point (if you have one) and hit record.

-> find_center_2000x ->
Make sure the Navigator is open.

Navigator (in the menu bar) > Montage and Grids > Add Circle polygon > Enter desired radius (125 for RC3; TODO add a choice to manage this with other samples/core scans).

In the navigator window, check the boxes for Acquire, New File at item. Create a new left-zero-padded 4-digit folder for the section number (off the sticky note), open that folder and save sec#.idoc (no zero-padding).

Go to 5000X with spot size 2.

Recenter the beam.

Put the mirror down and brighten/darken the beam so the current density on TEM Center is about {current_density}.

* Done
- (focus) Hit the OL wobble button. Check if the dot is moving in a very tiny circle. If not, hit bright tilt and use {tem1_setup: yellow dials|XY dials} to get it back in calibration.

Run auto focus. WATCH all of the pictures to make sure none are staticky.

* Some images were staticky
-> focus
* Autofocus proceeded normally
- Hit record to check if focused. (You are looking to make sure you can see 2 distinct lines for the fine cell membranes.)
Make sure you are at 5000X and spot size 2.
Run Calibration > Image shift.

* It says "THIS IS BAD" or "THIS IS TERRIBLE"
-> focus
* It says "THIS IS GOOD"
- Click "OK". If a message pops up, select NO.

Use the scrollball to find an empty part of formvar. In the Navigator, click "Add Stage Position".

* Done
- Check again for current density close enough to {current_density}. Move the mirror back out of the beam.

Navigator > Acquire at Items

Is this a recapture?

* Yes
Run script Calibrate and Recapture
* No
Run script High Mag Cook

- Is this your last capture of the day (in other words, will you still be here in {capture_estimate} hours?)
* Last capture
Make sure to leave "turn off filament at end of capture" checked.
* Not the last capture
Make sure "turn off filament at end of capture" is NOT checked.

- Click OK.

The status bar should say "capturing X of 900".

Fill out the notes file with anything strange that happened during the process. Save it in the folder you created for this section.

Put the lid on.

TODO step 39 says Image Shift will run again -- but is that true?

Step 40: copy the 150X, 2000X screenshots, and the capture data, into a folder labeled with the section number. Upload to Dropbox.

* Run another capture
-> new_capture


// Subroutines

== find_center_2000x

Use the screenshot to find your high mag center point. For this part, you may need to {tem1_setup: move the sample with the scroll ball and keep hitting "record" to find the spot|use the "Search" function of SerialEM to move the sample and find the center.}

Save your new screenshot to the desktop.

* Done
->->

== find_center_150x

Spread the beam and hit RECORD.
* Done
- Find the center point at 150x, mark it, and save a screenshot to the desktop.

TODO this is a little complicated too, and could use its own interactive tutorial.

* Done
->->

== low_mag_cook

Change spot size to 1.
Center the beam and condense it to the inner brackets. Make sure you are at 150X.
Start Macro 1 (Burn Wobble).

* Started it
- Now you have to wait 7 minutes--but keep an eye on the process for any problems. I'll time that for you.

Now hit stop.

* Done
->->

== rod_in

Do you have the next section ready in a rod?
* Yes
-

It's easiest to do this right if you're standing up.
Line up the rod so the knob on the long part will fit the hole in the scope.
Insert the rod, making sure that the thin part with the grid in it doesn't scrape or hit on the side of the hole.
Stop inserting when you hit the first point of resistance. 

This is called "safety position".

* done


-Flip the pump switch to PUMP. The yellow light should be on!

- (yellow_light)
+ The yellow light isn't on
    You can twist the rod a tiny bit counter-clockwise, or push it in a little more, to see if the light turns on
    -> yellow_light
+ The yellow light was on but it turned off!
You can twist the rod a tiny bit counter-clockwise, or push it in a little more, to see if the light turns on
    -> yellow_light
* Yellow pump light is on AND after a while, specimen chamber in TEM Center has turned GREEN.
- You can now twist the rod clockwise twice and hold the rod as it is sucked in
* Done
->->

== rod_out

Pull the specimen rod directly out until you can twist counter-clockwise, then twist, then pull again, then twist again. It will stay there.

This is called "safety position".

* Done

- Flip the Pump switch to AIR.
Watch V18 and V16 in the TEM Center window.

* V18 and V16 have turned green, then back to black (closed).
- Remove the rod all the way.
Counter-intuitively, it's safest to do this in one quick motion.
Place the rod in its holder.
* Done
-

->->

== put_next_section
-> rod_out ->

Remove the grid from the rod and place it back in the container where it came from. Insert a new grid. Mark the new grid's section number and slot (for example, A5 or E10) on your sticky note.

-> rod_in ->

->->

== filament_change

The filament change protocol hasn't been written as an interactive tutorial yet. Find the text file in temmy/protocols/filamentChange.txt

-> DONE 