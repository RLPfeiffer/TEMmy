# To be more Lisp-like, make print() return its argument.
def print_decorator(p):
    def wrapped_print(*args,**kwargs):
        p(*args,**kwargs)
        if len(args) == 1:
            return args[0]
    return wrapped_print

print = print_decorator(print)

# Quick check for which TEM captured the given section
def whichTEM(idoc):
    if "OneView" in idoc.Note:
        return "TEM2"
    else:
        return "TEM1"