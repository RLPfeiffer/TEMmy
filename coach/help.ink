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
* How do I manage builds through the control panel? -> builds_panel
* How do I run the builds manually to do something the control panel doesn't allow? -> builds_manual_fix
* When and how do I run gain reference? -> gain_ref
* What were the MOST up-to-date/useful protocols for changing a filament and running beam calibration at the time Nat left -> new_filament

== code_questions

* How do I install rust and compile Bob? -> compile_bob
* How do I change the SerialEM Python scripts and package them onto DROPBOX? -> package_temmy
* How do I define a new SerialEM protocol tutorial? -> new_tutorial
* How do I add endpoints to the Bob control panel? -> control_panel_endpoints
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

TODO
-> DONE

== launch_bob

TODO
-> DONE

== builds_panel

TODO
-> DONE

== builds_manual_fix

TODO
-> DONE

== gain_ref

TODO
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

== package_temmy

TODO
-> DONE

== new_tutorial

TODO
-> DONE

== control_panel_endpoints

TODO
-> DONE

== new_filament

TODO
->DONE

== rito

TODO
->DONE

== extra_questions

If something is important enough, you can ask Dr. Jones for Nat's contact info and reach out to Nat for more help.

Nat may ask for reasonable personal compensation via PayPal, Venmo, or Patreon depending on the extent of labor given.

-> DONE