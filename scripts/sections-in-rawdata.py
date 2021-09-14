'''
Print a list of RC3 sections that are safely stored in RawData
'''
import os
import os.path
if __name__ == "__main__":
    rawdata_path = "\\\\OpR-Marc-Syn3\\Data\\RawData\\RC3"
    folders = os.listdir(rawdata_path)
    
    # Lazy rule of thumb: If there are more than 800 files in a section folder, it was a complete capture
    for folder in folders:
        full_path = os.path.join(rawdata_path, folder)
        if os.path.isdir(full_path) and len(os.listdir(full_path)) > 800:
            print(folder)