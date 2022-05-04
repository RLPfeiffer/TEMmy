'''
Add elements to Mosaic.VikingXML that are required for Viking to connect to a volume on the X: drive
'''

# Once you move it to X: you need to edit the Mosaic.VikingXML file to add the host
# and the path perameters to the top of the file. It should look something like this:
# <Volume
#       InputChecksum=""
#       Name="1106"
#       host="http://storage1.connectomes.utah.edu/"
#       num_sections="4"
#       num_stos="0"
#       path="http://storage1.connectomes.utah.edu/Jeanne/1106">

import sys
from shutil import copyfile
from datetime import datetime
from os.path import join
from code import interact

import xml.etree.ElementTree as ET

if __name__ == "__main__":
    if len(sys.argv) != 5:
        print("add-volume-path requires args [path to Mosaic.VikingXML] [name] [host] [backup directory]")
        exit(1)
    viking_xml_path = sys.argv[1]
    name = sys.argv[2]
    host = sys.argv[3]
    backup_dir = sys.argv[4]

    # Before automatically doing anything to a volume's files, make a backup
    timestamp = datetime.now().strftime("%d-%b-%Y-%H-%M-%S")
    copyfile(viking_xml_path, join(backup_dir, 'Mosaic.VikingXML.{}.xml'.format(timestamp)))

    # Get the Volume element from Mosaic.VikingXML
    viking_xml_tree = ET.parse(viking_xml_path)
    viking_xml_root = viking_xml_tree.getroot()

    # Set the Volume element's host and path attribute
    path = "{}{}".format(host, name)
    viking_xml_root.attrib['Name'] = name
    viking_xml_root.attrib['host'] = host
    viking_xml_root.attrib['path'] = path

    # Save the volume element
    viking_xml_tree.write(viking_xml_path, "utf-8")
