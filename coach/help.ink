// JavaScript functions
EXTERNAL showImage(file)

VAR camera_shutdown = false

-> main

== main

* TEM operator questions -> tem_questions
* Questions about maintaining Nat's code -> code_questions
* [Questions or help that isn't covered here] -> extra_questions

== tem_questions

* One of the hard drives is full. How do I free up space? -> wiz_tree
* SerialEM had a random error while running a tutorial. What do I do now? -> serialem_error
* Bob (or the Bob control panel) isn't running. How do I restart it? -> launch_bob
* How do I connect to build1? -> connect_to_build1
* How do I manage builds through the control panel? -> builds_panel
* I see a weird spot on the camera images that won't go away. -> gain_ref
* When and how do I run gain reference? -> gain_ref
* What were the MOST up-to-date/useful protocols for changing a filament and running beam calibration at the time Nat left -> new_filament

== code_questions

* How do I install rust and compile Bob? -> compile_bob
* How do I change the SerialEM Python scripts and package them onto DROPBOX? -> package_temmy
* How do I define a new SerialEM protocol tutorial? -> new_tutorial
* How does rito, the program managing the TEMBot slack messages, work? -> rito

== serialem_error

Sometimes the connection between the SerialEM/camera computer and the TEM computer starts to have errors when it has been open for too long.

Whatever error you're seeing may not be as big a deal as it seems, because when running a tutorial, SerialEM remembers what step it needs to run next, even if you have to restart the program.

Close SerialEM. If you have navigator items, save them to a file.
* Done
- Close Digital Micrograph.
* Done
- If you have time to wait a few minutes, you can also shut down the SerialEM/camera computer for good measure. You will restart it later.
* I have time to shut it down
    ~ camera_shutdown = true
* I want to skip the full shutdown this time
    ~ camera_shutdown = false
- Turn off the filament in TEM Center.
* Done
- Close TEM Center.
* Done
- If you have time to wait a few minutes, you can also shut down and restart the TEM Center computer for good measure.
* I have time to shut it down
    Shut down and restart the TEM Center computer. Wait for TEM Center to open again automatically.
* I want to skip the full shutdown this time
    Close TEM Center. Then open it again and wait for it to finish loading.
- {camera_shutdown: Turn the SerialEM/Camera computer on again.}
Open Digital Micrograph.
Open SerialEM.
* Done
- Turn the filament on again.
When you click "Next Step", the tutorial should attempt to complete the task that caused the error before. You can continue as normal.
When SerialEM asks you for notes on the capture, note that TEM Center or the TEM computer had to be restarted.

-> DONE

== wiz_tree

When I need to clear space on one of the network hard drives, I use a program called WizTree.

https:\/\/www.diskanalyzer.com/download

Download the portable version of WizTree and extract the folder. Run WizTree64.exe and select the network drive that is full to run a scan on.

* I have downloaded WizTree Portable and run the scan
- WizTree will show you a tree view and a grid view of the places where the most file storage is being used. When you hover over the grid view, it will show you where the selected file is. I use this to spot folders of data that are safe to move or delete to other places.

Never delete anything unless you know what it is and have double-checked with the lab if anyone needs or wants those files saved.

~ showImage("wiztree.png")

-> DONE

== connect_to_build1

Press the windows key and type "Remote Desktop", then click on that application.

Type `OpR-Marc-Build1` into the Computer field. Press ENTER and type in your password.

If it doesn't work, Jamie may need to give your account permission to access Build1.

-> DONE

== launch_bob

<- connect_to_build1

A folder called temmy needs to exist in C:/Python37/Scripts. (It should already.)

If it doesn't, ask Becca or Jamie to clone the connectomes/temmy repository in that folder.

Open a terminal on Build1.

Type these lines, followed by ENTER:

```
cd \\Python37\\Scripts
Y:\\Dropbox\\bob.exe
```

This also launches the Bob web UI.

-> DONE

== builds_panel


To run builds with the Bob control panel, first make sure Bob is running.

* It is
* It's not
    First, launch Bob by following these instructions:
    <- launch_bob

-

<- connect_to_build1
Open Firefox.

Type http:\/\/155.100.106.88:5000/ into the browser window followed by a volume name and the start and end number of sections to work on.

For example: http:\/\/155.100.106.88:5000/rc3/1400/1450

You should see a table of sections that require attention.

* I can't see images when I click on MosaicReports.
    Copy the `distribution` folder from `C:/Python37/Scripts/temmy/bob-web-ui` into `C:/Program Files/Mozilla Firefox`. Reopen firefox and try loading again.
* It works
-
-> DONE

== gain_ref

Gain reference is an important calibration to remove dark spots from our camera images. Whenever you see a dark spot in SerialEM that doesn't move or go away when you move the stage, you will know you need to run Gain Reference.

There is a new protocol for Gain Reference which Gatan wants us to use. Becca knows how to do it. If she's available, ask her to teach you.

If Becca is not available, there is an old protocol for Gain Reference that still works, for now:

1. Pull rod out to first position
2. Put foil thingy into the slot to keep the vacuum on.
3. Go to High Mag 5000X
4. Center the Beam
5. In Digital Micrograph, Click Help > User Mode > Power User
6. Camera>Acquire Gain reference
7. Follow the instructions in Digital Micrograph
8.	Click Help > User Mode > Regular

-> DONE

== compile_bob

The latest instructions for installing Rust are here: https:\/\/www.rust-lang.org/tools/install

After installing Rust, clone the temmy repository: https:\/\/github.com/connectomes/temmy

Then in a windows terminal:

```
cd temmy/bob

deploy
```

(deploy.cmd is a script in the bob directory. It calls `cargo build` and if the compilation is successful, it copies the binary to our DROPBOX drive where Build1 can find it.)

-> DONE

== new_tutorial

Clone the `connectomes/temmy` repository and open either Util-ZZTutorialCore.py or Util-ZZTutorialVolume.py from the macros folder in a text editor.

Define your new tutorial by adding a line like this one at the bottom of the file: 
`Steps["RC3"] = NewSpecimenSteps + LowMagCookSteps + MainVolumeSteps("Jones", "RC3", 125, True, False)`

After making a new tutorial, you need to package temmy, correct any errors, and load the new script package.

-> package_temmy

== package_temmy

Make sure you have a Y: network mapping for \\OpR-Marc-RC2\Data

```
cd temmy
python package.py
```

package.py will detect any type errors in your tutorial. These need to be fixed before your tutorial can be used.

If package.py succeeds, tem1package and tem2package txt files will appear in DROPBOX/nat/script_packages/ with new timestamps.

On each microscope, open SerialEM and Click Scripts > Load New Package, and choose the corresponding new package file.

Test your tutorial as soon as possible.

-> DONE

== new_filament

Becca is in charge of the filament change protocols. She can help.
->DONE

== rito

rito is Nat's Python library for sending automated notifications. https:\/\/github.com/nqnstudios/rito

Bob relies on rito to send slack notifications. To install rito:

`pip install rito`

Make sure the Scripts folder of your Python installation is in your PATH.

You also have to add some Environment Variables.

OPENCV_IO_MAX_IMAGE_PIXELS: 3361836000
RITO_SLACK_TOKEN: get this value from DROPBOX/nat/RITO_SLACK_TOKEN.txt

->DONE

== extra_questions

If something is important enough, you can ask Dr. Jones for Nat's contact info and reach out to Nat for more help.

Nat may ask for reasonable personal compensation via PayPal, Venmo, or Patreon depending on the extent of labor given.

-> DONE