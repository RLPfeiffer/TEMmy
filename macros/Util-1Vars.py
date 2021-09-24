# Python variables that must be global on both microscopes
from os import linesep
from os.path import join
newline = linesep[1]

# Globally set the variable MessageDir
MessageDir = join(DropboxPath, "Notification", ScopeName)

# Globally set the variable MessagePath
MessagePath = join(MessageDir, "message.txt")

# Globally set the variable $CopyDir
CopyPath = join(DropboxPath, "TEMXcopy")

# Globally set the variable SecondsPerTile
SecondsPerTile = 15