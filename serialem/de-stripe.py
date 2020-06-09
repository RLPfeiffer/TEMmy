'''
Adjust brightness of mosaic tiles so they appear seamlessly connected.

Setup:
```
git clone https://github.com/jamesra/nornir-buildmanager
(cd nornir-buildmanager && python setup.py install)
git clone https://github.com/jamesra/nornir-shared
(cd nornir-shared && python setup.py install)
```

Run examples:
~ python de-stripe.py V:\RawData\RC3 1              # de-stripe section 0001
~ python de-stripe.py V:\RawData\RC3 1,2,3          # de-stripe sections 0001, 0002, 0003
~ python de-stripe.py V:\RawData\RC3 1-21           # de-stripe all sections that exist between 0001 and 0022 (skips 0021 which is missing)
~ python de-stripe.py V:\RawData\RC3 1-21,26-30     # de-stripe multiple ranges of sections

'''

import sys
# To be more Lisp-like, make print() return its argument.
def print_decorator(p):
    def wrapped_print(*args,**kwargs):
        p(*args,**kwargs)
        if len(args) == 1:
            return args[0]
    return wrapped_print

print = print_decorator(print) 

def parseSections(arg):
    '''
    Parse a list of section numbers from the given arg which may have comma-separated ranges and individual values
    '''
    sections = []

    if ',' in arg:
        for list_arg in arg.split(','):
            sections += parseSections(list_arg)
    else:
        if '-' in arg:
            lower, upper = [int(num) for num in arg.split('-')]
            return range(lower, upper+1)
        else:
            return [int(arg)]

    # Eliminate duplicates
    return list(set(sections))



if __name__ == "__main__":
    volume_dir = ""
    sections = []
    if len(sys.argv) > 1:
        volume_dir = sys.argv[1]
    else:
        print("First arg must specify the directory of a volume's raw data.")
        exit()
    if len(sys.argv) > 2:
        sections = parseSections(sys.argv[2])
    if len(sections) == 0:
        print("Second arg must specify one or more sections to correct.")
        exit()

    print(sections)
