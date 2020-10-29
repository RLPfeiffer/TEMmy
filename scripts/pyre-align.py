'''Open the STOS file that will be used for aligning two RC3 sections'''

from sys import argv
from os.path import join, exists
from subprocess import run

if __name__ == "__main__":
    sections = list(map(int, argv[1].split("-")))
    # stos files are named lower-higher
    if sections[0] > sections[1]:
        lesser_section = sections[1]
        sections[1] = sections[0]
        sections[0] = lesser_section
    
    # W:\Volumes\RC3\TEM\Grid32
    auto_stos_dir = 'W:/Volumes/RC3/TEM/Grid32'
    manual_stos_dir = join(auto_stos_dir, 'Manual')

    stos_name = "{}-{}_ctrl-TEM_Leveled_map-TEM_Leveled.stos".format(sections[0], sections[1])

    # If a manual alignment already exists, open it
    stos_file = join(manual_stos_dir, stos_name)
    if not exists(stos_file):
        stos_file = join(auto_stos_dir, stos_name)
    
    run(["pyre", "-stos", stos_file])