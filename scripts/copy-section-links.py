'''
Append section links from a temporary volume to an existing volume's VolumeData.xml section links.
'''

import sys
from shutil import copyfile
from datetime import datetime
from os.path import join
from code import interact

import xml.etree.ElementTree as ET

if __name__ == "__main__":
    if len(sys.argv) != 4:
        print("copy-section-links requires args [volumedata.xml of the main volume] [volumedata.xml of the temporary volume] [backup directory]")
        exit(1)
    existing_volume_data = sys.argv[1]
    new_volume_data = sys.argv[2]
    backup_dir = sys.argv[3]

    # Before automatically doing anything to a volume's files, make a backup
    timestamp = datetime.now().strftime("%d-%b-%Y-%H-%M-%S")
    copyfile(existing_volume_data, join(backup_dir, 'VolumeData.{}.xml'.format(timestamp)))

    # Get the Section_Link elements from VolumeData.temp
    new_section_links = ET.parse(new_volume_data).getroot()[:]

    # Get a writable tree object for VolumeData.xml
    existing_volume_xml_tree = ET.parse(existing_volume_data)
    existing_volume_xml_root = existing_volume_xml_tree.getroot()

    # Find the index of the last section link in VolumeData.xml
    # (This must be done linearly because the XML API has no last_index_of(match))
    idx = len(existing_volume_xml_root) - 1
    while idx >= 0:
        if existing_volume_xml_root[idx].tag == "Section_Link":
            break
        idx -= 1

    # Insert the new links starting AFTER that index:
    for section_link in new_section_links:
        idx += 1
        existing_volume_xml_root.insert(idx, section_link)

    # Overwrite VolumeData.xml
    existing_volume_xml_tree.write(existing_volume_data, "utf-8")