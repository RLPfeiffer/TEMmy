import sys
import shutil
import os
from os import path

# Check to make sure there is enough space, then copy the contents CopySource recursively to CopyTarget.
def CopyDir(CopySource:str, CopyTarget:str, TargetDirName:str) -> bool:
   if CheckSpaceForCopyDir(CopySource, CopyTarget, TargetDirName):
      return CopyDirWithoutCheckingSpace(CopySource, CopyTarget, TargetDirName)
   else:
      SendMessage(f"Not enough space to auto-copy {TargetDirName} from {ScopeName} {CopySource} to {CopyTarget}")
      return False

# INTERNAL, called by CheckSpaceForCopyDir
def RecursiveFolderSize(folder:str) -> int:
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
def CheckSpaceForCopyDir(CopySource:str, CopyTarget:str, TargetDirName:str) -> bool:
   _, _, FreeSpace = shutil.disk_usage(CopyTarget)
   TargetDirSize = RecursiveFolderSize(path.join(CopySource, TargetDirName))
   return FreeSpace >= TargetDirSize

# INTERNAL helper function that copies a directory to CopyTarget without checking for space first.
# (External-use functions all check before calling this one)
def CopyDirWithoutCheckingSpace(CopySource:str, CopyTarget:str, TargetDirName:str) -> bool:
   try:
      shutil.copytree(path.join(CopySource, TargetDirName), path.join(CopyTarget, TargetDirName))
      return True
   except:
      SendMessage(f"Error {sys.exc_info()[0]} while attempting to copy {TargetDirName} from {ScopeName} to {CopyTarget}")
      return False

# The argument values need to be updated to match the SerialEM computer's current network mappings and a folder in the data drive before testing
# Test the functions without side effects:
#print(CheckSpaceForCopyDir("E:/", "Y:/Dropbox/TEMXCopy", "0976")) # TEM1
# print(CheckSpaceForCopyDir("G:/", "Y:/TEMXCopy", "0975")) # TEM2

# Test with major side effects:
# CallFunction CopyFunctions::CopyDir
#CopyDir("E:/", "Y:/Dropbox/TEMXCopy", "core_kwanEmbryos_13374") # TEM1
# print(CheckSpaceForCopyDir("G:/", "Y:/TEMXCopy", "0975")) # TEM2