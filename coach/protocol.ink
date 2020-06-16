// JavaScript functions
EXTERNAL showImage(file)
EXTERNAL timer(time)
EXTERNAL beep(times)

// TEM1/TEM2 variables
VAR current_density = 0
VAR capture_estimate = 0

-> first_time_ever

== tem1_setup
// Set TEM1-specific variables
~ current_density = 250
~ capture_estimate = 7

->->

== tem2_setup
// Set TEM2-specific variables
~ current_density = 65
~ capture_estimate = 3.5

->->

// Protocol
== first_time_ever

~ showImage("temmy.png")
Welcome to TEMmy, the Electron Microscope tutor named after Toby Fox's adorable character from Undertale.

/* // Uncomment this block to test the timer() function:
* Start
-


Waiting 5 seconds
~ timer("5 s")
Waited, now
* click here
- waiting again (1 minute)
~ timer("1 m")
Waited!
*/
// This choice should only need to be made once, when TEMmy is opened on the computer corresponding to one of our two TEMs.

First of all, which electron microscope are you using?

* TEM1
-> tem1_setup ->
* TEM2
-> tem2_setup ->
- Great.

-> first_time_today

== first_time_today

You need to warm up the filament in your scope. Make sure TEM Core is open.

Check HT Voltage.

* HT Voltage is off (red) or less than 80.
Someone is in the middle of increasing the HT Voltage, probably because they installed a new filament.
-> increase_ht_voltage ->
* HT Voltage is on (green) and at 80.

- -> turn_on_filament ->


- (new_capture) If Serial EM is open from a previous capture, close it. Do not save anything.
Turn off the filament {tem2_setup: and BEAM BLANK}.

* Ok.
- -> rod_out ->
Flip the Pump switch to AIR.
Watch V18 and V16 in the TEM Center window.

* V18 and V16 have turned green, then back to black (closed).
- Remove the rod all the way and place it in its holder.

Remove the grid from the rod and place it back in the container where it came from. Insert a new grid. Mark where it came from on a sticky note.

-> rod_in ->

Flip the pump switch to PUMP. The yellow light should be on!

* Yellow pump light is on, specimen chamber in TEM Center is GREEN.
- You can now twist and insert the rod completely.

* HT is at 80, and turned on.
- * Penning Gauge is on and below 30.
- Turn on the filament.
Open SerialEM.
Make sure the screen is raised.
Hit the low mag button. SerialEM should go to 150X.
Turn the aperture dial to the red dot.
* These are all done
- Wait until the filament is fully on (the button should be bright green and stop blinking).
Then lower the screen. (This is the {tem1_setup: F1|Screen Up} button on the right-hand physical control panel.)
Examine the section for damage or large dirt.
* There is damage/large dirt/holes in the formvar near the sample
-> kevin
* It looks good
- Locate the center of the tissue (move using the scroll ball on the left-hand control panel if necessary).

-> low_mag_cook ->
-> find_center_150x ->

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

Create a new left-zero-padded 4-digit folder for the section number (off the sticky note), open that folder and save sec#.idoc (no zero-padding).

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
~ timer("7 minutes")
Now hit stop.

* Done
->->

== rod_in

Insert the specimen rod into safety position. THIS DOES NOT INVOLVE TWISTING I THINK.

TODO: More specific instructions/pictures.
* Done
->->

== rod_out

Pull the specimen rod into safety position.

(Pull, twist, pull again. Click.)

THIS PART IS VERY DELICATE AND CONFUSING. TODO: more specific instructions/pictures.

* Done
->->

== increase_ht_voltage
// TODO notes for this are handwritten
// ->->
-> kevin

== turn_on_filament
{tem2_setup: Use the physical control panel on the left to turn on BEAM BLANK. The button should light up.} 

Turn the filament on in TEM Core. The button should blink green for a while before becoming solid.

Then it could take around 30 minutes to be warmed up.

* Time it for me
~ timer("30 m")
* Skip that

- Ok, should be good to go!

->->

== kevin
Talk to Kevin.