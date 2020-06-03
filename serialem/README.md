# TEMmy SerialEM Tools

## SerialEM dev environment setup

* Download and run the SerialEM [GenericFramework.exe](https://bio3d.colorado.edu/ftp/SerialEM/Frameworks/GenericFramework.exe)
* Copy the three text files from Y:/DROPBOX/SerialEMSettingsFiles/TEM1 to the new folder called C:/ProgramData/SerialEM
* Download and run the SerialEM [application/plugin installer](https://bio3d.colorado.edu/SerialEM/download.html#Available) for your system
* In C:/Program Files/SerialEM/SerialEM_{VERSION}, open SerialEM.chm and follow the steps in General Topics/Settings up SerialEM
    * The only important step seems to be running `install.bat` as an administrator.
* Open SerialEMproperties.txt (you should be able to use the shortcut in this folder)
* Above the line starting with `NumberOfCameras`, add:
    ```
    NoScope					1
    NoCameras				1
    ```
* TODO I must be missing a plugin that's required, because there are errors on launch that make it use default config (and fail)