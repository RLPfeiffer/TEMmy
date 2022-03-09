#! /usr/bin/env python

from os import listdir


def checkFrom(lowest_section, highest_section):
    temxcopy = listdir('Y:/DROPBOX/temxcopy')
    rawdata = listdir('V:/rawdata/rc3')
    volume = listdir('W:/volumes/rc3/tem')

    output = '<table><tr><td>sec#</td><td>in volume</td><td>in rawdata</td><td>in temcopy</td></tr>'

    for section in range(lowest_section, highest_section+1):
        section = str(section).rjust(4, '0')
        in_volume = section in volume
        in_rawdata = section in rawdata
        in_temxcopy = section in temxcopy or f'Jones_RC3_{section}' in temxcopy
        if in_volume and in_rawdata:
            pass
        else:
            output += f'<tr><td>{section}</td><td>{in_volume}</td><td>{in_rawdata}</td><td>{in_temxcopy}</td></tr>'
    output += '</table>'
    return output