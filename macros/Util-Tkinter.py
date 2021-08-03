import tkinter as tk
from tkinter import messagebox
from tkinter import simpledialog
from typing import Optional

root = tk.Tk()
root.lift()
root.withdraw()

def YesNoBox(prompt:str) -> bool:
    return messagebox.askyesno("SerialEM", prompt, parent=root)

def EnterString(prompt:str, default:Optional[str]=None) -> str:
    return simpledialog.askstring("SerialEM", prompt, parent=root, initialvalue=default)

# Can provide minvalue, maxvalue via kwargs
def EnterInt(prompt:str, default:Optional[int]=None, **kwargs:int) -> int:
    return simpledialog.askinteger("SerialEM", prompt, parent=root, initialvalue=default, **kwargs)

# Can provide minvalue, maxvalue via kwargs
def EnterFloat(prompt:str, default:Optional[float]=None, **kwargs:float) -> float:
    return simpledialog.askfloat("SerialEM", prompt, parent=root, initialvalue=default, **kwargs)

