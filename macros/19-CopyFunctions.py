#!Python3.9
#MacroName CopyFunctions - Python
#include Notifications - Python
import serialem as sem
import shutil
from os import path

# The argument values need to be updated to match the SerialEM computer's current network mappings and a folder in the data drive before testing
# Test the functions without side effects:
#print(CheckSpaceForCopyDir("E:/", "Y:/Dropbox/TEMXCopy", "0976")) # TEM1
# print(CheckSpaceForCopyDir("G:/", "Y:/TEMXCopy", "0975")) # TEM2

# Test with major side effects:
# CallFunction CopyFunctions::CopyDir
#CopyDir("E:/", "Y:/Dropbox/TEMXCopy", "0976") # TEM1
# print(CheckSpaceForCopyDir("G:/", "Y:/TEMXCopy", "0975")) # TEM2

# Check to make sure there is enough space, then copy the contents CopySource recursively to CopyTarget.
def CopyDir(CopySource, CopyTarget, TargetDirName):
   if CheckSpaceForCopyDir(CopySource, CopyTarget, TargetDirName):
      CopyDirWithoutCheckingSpace(CopySource, CopyTarget, TargetDirName)
   else:
      SendMessage("Not enough space to auto-copy {} from {} to {}".format(TargetDirName, sem.GetVariable("ScopeName"), CopyTarget))

# INTERNAL, called by CopyDir
# Return True if there is space in the destination to copy the target directory.
def CheckSpaceForCopyDir(CopySource, CopyTarget, TargetDirName):
   _, _, FreeSpace = shutil.disk_usage(CopyTarget)
   _, TargetDirSize, _ = shutil.disk_usage(path.join(CopySource, TargetDirName))
   return FreeSpace >= TargetDirSize

# INTERNAL helper function that copies a directory to CopyTarget without checking for space first.
# (External-use functions all check before calling this one)
def CopyDirWithoutCheckingSpace(CopySource, CopyTarget, TargetDirName):
   try:
      shutil.copytree(path.join(CopySource, TargetDirName), CopyTarget)
   except Error as e:
      SendMessage("Error {} while attempting to copy {} from {} to {}".format(e, TargetDirName, sem.GetVariable("ScopeName"), CopyTarget))