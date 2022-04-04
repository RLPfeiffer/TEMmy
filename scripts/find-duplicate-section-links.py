'''
Find and remove duplicate section links in a VolumeData.xml file
'''

import sys
from shutil import copyfile
from datetime import datetime
from os.path import join
from code import interact

import xml.etree.ElementTree as ET

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("find-duplicate-section-links requires args [volumedata.xml of the volume] [backup directory]")
        exit(1)
    existing_volume_data = sys.argv[1]
    backup_dir = sys.argv[2]

    # Before automatically doing anything to a volume's files, make a backup
    timestamp = datetime.now().strftime("%d-%b-%Y-%H-%M-%S")
    copyfile(existing_volume_data, join(backup_dir, 'VolumeData.{}.xml'.format(timestamp)))

    # Get a writable tree object for VolumeData.xml
    existing_volume_xml_tree = ET.parse(existing_volume_data)
    existing_volume_xml_root = existing_volume_xml_tree.getroot()
    
    section_link_dict = {}
    indices = list(range(len(existing_volume_xml_root)))
    indices.reverse()
    for idx in indices:
        element = existing_volume_xml_root[idx]
        if element.tag == "Section_Link":
            number = element.attrib['Number']
            if number in section_link_dict:
                conflicting_index = section_link_dict[number]
                conflicting_link = existing_volume_xml_root[conflicting_index]
                print(f'{ET.tostring(conflicting_link, encoding="utf8", method="xml")} conflicts with {ET.tostring(element, encoding="utf8", method="xml")}.')
                which = int(input('Use 1 or 2?').strip())
                index_to_remove = idx if which == 2 else conflicting_index
                del existing_volume_xml_root[index_to_remove]
            else:
                section_link_dict[number] = idx

    # Overwrite VolumeData.xml
    existing_volume_xml_tree.write(existing_volume_data, "utf-8")