from os.path import join, exists
from shutil import rmtree

def ClearDataDrive() -> None:
    lines = []
    with open(join(CopyPath, "rawdata.txt"), "r") as f:
        lines = f.readlines()

    new_lines = []
    sections_to_delete = []
    for line in lines:
        if len(line.strip()) != 0:
            section = line.split(" ")[0]
            path = join(DataPath, section).strip()
            if exists(path):
                sections_to_delete.append(path)
            else:
                new_lines.append(section)

    if YesNoBox(f"Delete {len(sections_to_delete)} sections which have been copied to RawData?"):
        list(map(rmtree, sections_to_delete))
    else:
        new_lines.extend(sections_to_delete)
    
    with open(join(CopyPath, "rawdata.txt"), "w") as f:
        f.writelines(new_lines)