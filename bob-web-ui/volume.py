from os import listdir
from os.path import exists
import yaml

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

def checkMark(status):
    return "✔" if status else ""

def conditional(text, cond):
    return text if cond is not None and cond == True or len(cond) > 0 else ""

def volume_info(volume_name):
    with open('bob-config.yaml', 'r') as f:
        config_yaml = yaml.safe_load(f.read())
        volumes = config_yaml['volumes']
        for volume in volumes:
            if volume['name'] == volume_name:
                return volume
    return None

def checkFrom(volume_name, lowest_section, highest_section):
    temxcopy = listdir(f'Y:/DROPBOX/temxcopy/{volume_name}')
    rawdata = listdir(f'V:/rawdata/{volume_name}')
    volume_path = volume_info(volume_name)['path']
    volume = listdir(f'{volume_path}/tem')

    output = f'<!DOCTYPE html><div style="max-height: 100vh; overflow: auto;"><table border="1" style="position: relative; text-align: left;">{thr("sec#", "in volume", "in rawdata", "in temxcopy", "mosaicreport")}'
    for section in range(lowest_section, highest_section+1):
        section = str(section).rjust(4, '0')
        in_volume = checkMark(section in volume)
        in_rawdata = checkMark(section in rawdata)
        in_temxcopy = checkMark(section in temxcopy)
        if in_volume and in_rawdata:
            pass
        else:
            mosaic_report = find_mosaic_report(volume_name, section)
            mosaic_report_link = f'file:///{mosaic_report}' if 'D:/' in mosaic_report else f'/file/{mosaic_report}'
            output += tr(
                section,
                in_volume,
                in_rawdata,
                in_temxcopy,
                blank_link(mosaic_report_link, mosaic_report),
                blank_link(f'/build/{volume_name}/{section}', conditional('build', in_temxcopy)),
                blank_link(f'/rebuild/{volume_name}/{section}', conditional('rebuild', in_rawdata)),
                blank_link(f'/fixmosaic/{volume_name}/{section}', conditional('fix mosaic', mosaic_report)),
                blank_link(f'/merge/{volume_name}/{section}', conditional('Merge', mosaic_report)))

    output += '</table></div>'
    return output

def find_mosaic_report(volume_name, section_str:str):
    for place in [f"W:/Volumes/{volume_name}_temp", f"D:/Volumes/{volume_name}_temp"]:
        possible_path = f'{place}/{section_str}/MosaicReport.html'
        if exists(possible_path):
            return possible_path

    return ''

def tell_bob(command):
    print(command)
    with open('Y:/DROPBOX/Notification/BobUI/message.txt', 'a') as f:
        f.write(f'\n{command}\n')

def build(volume, section):
    tell_bob(f'Build: {volume} {section}')
    return f'building {volume} {section}. monitor the #tem-bot slack channel for results. you can close this window'

def rebuild(volume, section):
    tell_bob(f'Rebuild: {volume} {section}')
    return f'rebuilding {volume} {section}. monitor the #tem-bot slack channel for results. you can close this window'

def fixmosaic(volume, section):
    tell_bob(f'FixMosaic: {volume} {section}')
    return f'fixing mosaic for {volume} {section}. monitor the #tem-bot slack channel for results. you can close this window'

def merge(volume, section):
    tell_bob(f'Merge: {volume} {section}')
    return f'merging {section} into {volume}. monitor the #tem-bot slack channel for completion. you can close this window'
