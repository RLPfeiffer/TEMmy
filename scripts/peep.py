''' Check (peep) all the places where abnormal information about the given RC3 sections would be found '''

import sys
from util import *
from process_sections import *

def peep(section, idoc, log):
    # TODO check the section in the section directory excel file
    # TODO check that any previous recaptures are in RawData using another processSections call
    # TODO check if there is a manual stos file
    # TODO check the notes file
    # TODO open the overview
    print(section)

if __name__ == "__main__":
    usageString = "Usage: python peep.py [section range]"
    assert len(sys.argv) > 1, usageString
    
    sectionsRange = sys.argv[1]

    processSections("W:\\Volumes\\RC3\\TEM", sectionsRange, peep)