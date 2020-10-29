''' Parse a range of sections and run a function on all of their idocs/logs with multiprocessing
'''

from os import listdir
from os.path import join, exists, dirname, basename, splitext, isdir
from multiprocessing import Pool, cpu_count
from functools import partial
from glob import glob
from nornir_buildmanager.importers.idoc import IDoc
from nornir_buildmanager.importers.serialemlog import SerialEMLog

def parseSections(arg):
    '''
    Parse a list of section numbers (as strings) from the given string arg which may have comma-separated ranges and individual values
    '''
    sections = []

    if ',' in arg:
        for list_arg in arg.split(','):
            sections += parseSections(list_arg)
    else:
        if '-' in arg:
            lower, upper = [num for num in arg.split('-')]
            return [str(i) for i in range(int(lower), int(upper)+1)]
        else:
            return [arg]

    # Eliminate duplicates
    sections = list(set(sections))

def completeSectionDirList(volumeDir, sections):
    # All section directories will be at least four chars long
    sections = [section.rjust(4, "0") for section in sections]
    
    directories = [dir for dir in listdir(volumeDir) if isdir(dir)]

    # Some sections will be recaptures. Include them
    for dir in directories:
        if len(dir) > 4 and dir[0:4] in sections:
            sections.append(dir)

    sections = [join(volumeDir, section) for section in sections]

    # Some sections will be missing. Just skip them
    sections = filter(exists, sections)
    return sections


def processSections(volumeDir, sectionsRange, process):
    ''' `process` should be a function that takes args (sectionName, idoc, log)
    '''
    sectionDirs = completeSectionDirList(volumeDir, parseSections(sectionsRange))

    rangeIsEmpty = True
    for dir in sectionDirs:
        rangeIsEmpty = False
        break

    assert not rangeIsEmpty, "The given section range returned no existing directories."

    # Make a Processing pool that will utilize up to all available cpu cores.
    pool = Pool(cpu_count())

    return list(filter(lambda result: result != None, pool.map(partial(processSection, process), sectionDirs)))

def processSection(process, sectionDir):
    ''' Run the given function 'process', passing it the section directory, idoc, and log object '''
    section = str(int(basename(sectionDir)[0:4])) # Remove 0's from in front of the section number

    try:
        idoc = IDoc.Load(join(sectionDir, "{}.idoc".format(section)), None, False)
    # Because some idoc files might be mis-named, do a glob if loading fails
    except:
        print("section {} has mis-named idoc file".format(section))
        idocMatches = glob(join(sectionDir, '*.idoc'))
        if len(idocMatches) == 0:
            print("section {} has no idoc file".format(section))
            return
        idoc = IDoc.Load(idocMatches[0], None, False)

    try:
        log = SerialEMLog.Load(join(sectionDir, "{}.log".format(section)))
    except:
        print("section {} has mis-named log file".format(section))
        logMatches = glob(join(sectionDir, "*.log*"))
        if len(logMatches) == 0:
            print("section {} has no log file".format(section))
            return
        log = SerialEMLog.Load(logMatches[0].replace(".pickle", ""))

    assert idoc.NumTiles == log.NumTiles, "For section {}, the log and IDoc file have a different number of tiles!".format(section)

    return process(basename(sectionDir), idoc, log)