# Globally set the variable ScopeName
ScopeName = "TEM1"

# Globally set the variable MessageDir
MessageDir = f"Y:/DROPBOX/Notification/{ScopeName}"

# Globally set the variable $MessagePath
MessagePath = f"{MessageDir}/message.txt"

# Globally set the variable $DropboxPath
DropboxPath = "Y:/DROPBOX"

# Globally set the variable $CopyDir
CopyPath = f"{DropboxPath}/TEMXcopy"

# Globally set the variable $DataPath
DataPath = "E:"

# Globally set the variable PrecookMontageD
PrecookMontageD = 0

# Globally set the variable SecondsPerTile
SecondsPerTile = 15

# Filament heat-up time
FilamentHeatupSec = 45

##################################
# Change MaxPercentChangeOverCapture to adjust sensitivity of filament stability.  
# Must be a value 0 to 1.0.
#################################
MaxPercentChangeOverCapture = 0.20