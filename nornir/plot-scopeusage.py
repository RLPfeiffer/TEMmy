import sys
from util import *
from process_sections import *

def plotScopeUsage(section, idoc, log):
    return [whichTEM(idoc), log.StartupDateTime.strftime("%H:%M"), log.FinishDateTime.strftime("%H:%M")]

if __name__ == "__main__":
    usageString = "Usage: python plot-scopeusage.py [volume dir] [section range]"
    assert len(sys.argv) > 2, usageString
    assert exists(sys.argv[1]), usageString
    
    volumeDir = sys.argv[1]
    sectionsRange = sys.argv[2]

    print(processSections(volumeDir, sectionsRange, plotScopeUsage))