#! /usr/bin/env python

from os import listdir
from os.path import exists

def tr(*td_text):
    return f'<tr>{"".join(map(td,td_text))}</tr>'

def thr(*th_text):
    return f'<tr>{"".join(map(th,th_text))}</tr>'

def td(text):
    return f'<td>{text}</td>'
    
def th(text):
    return f'<th style="position: sticky; top: 0; background: white;">{text}</th>'

def blank_link(href, text=None):
    if href is None or len(href) == 0:
        return ''
    if text is None:
        text = href
    return f'<a target="_blank" href="{href}">{text}</a>'

def checkFrom(lowest_section, highest_section):
    temxcopy = listdir('Y:/DROPBOX/temxcopy')
    rawdata = listdir('V:/rawdata/rc3')
    volume = listdir('W:/volumes/rc3/tem')

    output = f'<!DOCTYPE html><div style="max-height: 100vh; overflow: auto;"><table border="1" style="position: relative; text-align: left;">{thr("sec#", "in volume", "in rawdata", "in temxcopy", "mosaicreport")}'

    for section in range(lowest_section, highest_section+1):
        section = str(section).rjust(4, '0')
        in_volume = section in volume
        in_rawdata = section in rawdata
        in_temxcopy = section in temxcopy or f'Jones_RC3_{section}' in temxcopy
        if in_volume and in_rawdata:
            pass
        else:
            mosaic_report = find_mosaic_report(section)
            output += tr(
                section,
                in_volume,
                in_rawdata,
                in_temxcopy,
                blank_link(f'/file/{mosaic_report}', mosaic_report),
                blank_link(f'/rc3build/{section}', 'build'),
                blank_link(f'/rc3rebuild/{section}', 'rebuild'),
                blank_link(f'/rc3fixmosaic/{section}', 'fix mosaic'),
                blank_link(f'/rc3merge/{section}', 'Merge'))

    output += '</table></div>'
    return output

def find_mosaic_report(section_str:str):
    for place in [r"W:/Volumes/", "D:/Volumes/"]:
        possible_path = f'{place}RC3{section_str}/MosaicReport.html'
        if exists(possible_path):
            return possible_path

    return ''

def tell_bob(command):
    print(command)
    with open('Y:/DROPBOX/Notification/BobUI/message.txt', 'a') as f:
        f.write(f'\n{command}\n')

def fixmosaic(section):
    tell_bob(f'RC3FixMosaic: {section}')
    return f'fixing mosaic for {section}. monitor the #tem-bot slack channel for results. you can close this window'

def merge(section):
    tell_bob(f'Merge: {section}')
    return f'merging {section} into rc3. monitor the #tem-bot slack channel for completion. you can close this window'
