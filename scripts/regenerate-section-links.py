'''
Find and regenerate missing section links in a VolumeData.xml file
'''

import sys
from shutil import copyfile
from datetime import datetime
from os.path import join
from os import listdir
from os.path import isdir, dirname
from code import interact

import xml.etree.ElementTree as ET

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("regenerate-section-links requires args [volumedata.xml of the volume] [backup directory]")
        exit(1)
    existing_volume_data = sys.argv[1]
    backup_dir = sys.argv[2]

    # Before automatically doing anything to a volume's files, make a backup
    timestamp = datetime.now().strftime("%d-%b-%Y-%H-%M-%S")
    # 2021-10-28 21:24:16
    timestamp_for_new_links = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
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
            path = element.attrib['Path']
            section_link_dict[path] = idx

    tem_dir = dirname(existing_volume_data)

    section_folders = filter(lambda f: isdir(join(tem_dir,f)), listdir(tem_dir))

    # Find the index of the last section link in VolumeData.xml
    # (This must be done linearly because the XML API has no last_index_of(match))
    idx_to_insert = len(existing_volume_xml_root) - 1
    while idx_to_insert >= 0:
        if existing_volume_xml_root[idx].tag == "Section_Link":
            break
        idx_to_insert -= 1

    for folder in section_folders:
        if folder not in section_link_dict:
            try:
                int(folder[0:4])
                section_link_xml = f'<Section_Link CreationDate="{timestamp_for_new_links}" Name="{folder}" Number="{folder[0:4]}" Path="{folder}" Version="1.0" />'
                existing_volume_xml_root.insert(idx_to_insert, ET.fromstring(section_link_xml))
                print(f'made section link for {folder}')
            except:
                pass

    # Overwrite VolumeData.xml
    existing_volume_xml_tree.write(existing_volume_data, "utf-8")