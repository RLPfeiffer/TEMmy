'''
Send the overview from a built section's MosaicReport
'''

import sys
from rito.senders import slack_image
from os.path import join

import xml.etree.ElementTree as ET

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("send-first-mosaic-overview requires arg [mosaicreport_folder]")
        exit(1)
    mosaicreport_folder = sys.argv[1]

    with open(join(mosaicreport_folder, "MosaicReport.html"), "r") as html:
        html = html.read()
        img_tag_opening = '<img src="'
        img_tag_index = html.index(img_tag_opening)
        src_index = img_tag_index + len(img_tag_opening)
        src_path = html[src_index:html.index('"', src_index)]
        slack_image.send_message("tem-bot", join(mosaicreport_folder, src_path))

