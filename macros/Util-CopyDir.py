import shutil
import os
from os import path

# Check to make sure there is enough space, then copy the contents CopySource recursively to CopyTarget.
def CopyDir(CopySource, CopyTarget, TargetDirName):
   if CheckSpaceForCopyDir(CopySource, CopyTarget, TargetDirName):
      CopyDirWithoutCheckingSpace(CopySource, CopyTarget, TargetDirName)
   else:
      SendMessage(f"Not enough space to auto-copy {TargetDirName} from {sem.GetVariable('ScopeName')} to {CopyTarget}")

# INTERNAL, called by CheckSpaceForCopyDir
def RecursiveFolderSize(folder):
    total_size = path.getsize(folder)
    for item in os.listdir(folder):
        itempath = path.join(folder, item)
        if path.isfile(itempath):
            total_size += path.getsize(itempath)
        elif path.isdir(itempath):
            total_size += RecursiveFolderSize(itempath)
    return total_size

# INTERNAL, called by CopyDir
# Return True if there is space in the destination to copy the target directory.
def CheckSpaceForCopyDir(CopySource, CopyTarget, TargetDirName):
   _, _, FreeSpace = shutil.disk_usage(CopyTarget)
   TargetDirSize = RecursiveFolderSize(path.join(CopySource, TargetDirName))
   return FreeSpace >= TargetDirSize

# INTERNAL helper function that copies a directory to CopyTarget without checking for space first.
# (External-use functions all check before calling this one)
def CopyDirWithoutCheckingSpace(CopySource, CopyTarget, TargetDirName):
   try:
      shutil.copytree(path.join(CopySource, TargetDirName), path.join(CopyTarget, TargetDirName))
   except:
      SendMessage(f"Error {sys.exc_info()[0]} while attempting to copy {TargetDirName} from {sem.GetVariable('ScopeName')} to {CopyTarget}")

# The argument values need to be updated to match the SerialEM computer's current network mappings and a folder in the data drive before testing
# Test the functions without side effects:
#print(CheckSpaceForCopyDir("E:/", "Y:/Dropbox/TEMXCopy", "0976")) # TEM1
# print(CheckSpaceForCopyDir("G:/", "Y:/TEMXCopy", "0975")) # TEM2

# Test with major side effects:
# CallFunction CopyFunctions::CopyDir
#CopyDir("E:/", "Y:/Dropbox/TEMXCopy", "core_kwanEmbryos_13374") # TEM1
# print(CheckSpaceForCopyDir("G:/", "Y:/TEMXCopy", "0975")) # TEM2