#! /usr/bin/env python

from os import listdir
from os.path import exists

def tr(*td_text):
    return f'<tr>{"".join(map(td,td_text))}</tr>'

def td(text):
    return f'<td>{text}</td>'

def checkFrom(lowest_section, highest_section):
    temxcopy = listdir('Y:/DROPBOX/temxcopy')
    rawdata = listdir('V:/rawdata/rc3')
    volume = listdir('W:/volumes/rc3/tem')

    output = f'<table>{tr("sec#", "in volume", "in rawdata", "in temxcopy", "mosaicreport")}'

    for section in range(lowest_section, highest_section+1):
        section = str(section).rjust(4, '0')
        in_volume = section in volume
        in_rawdata = section in rawdata
        in_temxcopy = section in temxcopy or f'Jones_RC3_{section}' in temxcopy
        if in_volume and in_rawdata:
            pass
        else:
            mosaic_report = find_mosaic_report(section)
            output += tr(section, in_volume, in_rawdata, in_temxcopy, f'<a target="_blank" href="/file/{mosaic_report}">{mosaic_report}</a>')
    output += '</table>'
    return output

def find_mosaic_report(section_str:str):
    for place in [r"W:/Volumes/", "D:/Volumes/"]:
        possible_path = f'{place}RC3{section_str}/MosaicReport.html'
        if exists(possible_path):
            return possible_path

    return ''