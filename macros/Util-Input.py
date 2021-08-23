from typing import Optional

def YesNoBox(prompt:str) -> bool:
    return int(sem.YesNoBox(prompt)) == 1

def EnterString(prompt:str, default:Optional[str]=None, forbidden_chars:str="") -> str:
    if default is not None and YesNoBox(f"Use {default} as {prompt}?"):
        for char in forbidden_chars:
            if char in default:
                raise ValueError(f"forbidden char {char} is in given default string '{default}'")
        return default
    sem.EnterString("RESERVEDVAR", prompt)
    value = str(sem.GetVariable("RESERVEDVAR"))
    for char in forbidden_chars:
        if char in value:
            return EnterString(f"Value cannot contain {char}. {prompt}", default, forbidden_chars)
    return value

def EnterFloat(prompt:str, default:float=0, decimal_places:int = 2, minvalue:Optional[float]=None, maxvalue:Optional[float]=None) -> float:
    value = float(sem.EnterDefaultedNumber(default, decimal_places, prompt))
    if minvalue is not None and value < minvalue:
        return EnterFloat(f"Value must be >= {minvalue}. {prompt}", default, decimal_places, minvalue, maxvalue)
    if maxvalue is not None and value > maxvalue:
        return EnterFloat(f"Value must be <= {maxvalue}. {prompt}", default, decimal_places, minvalue, maxvalue)
    return value

def EnterInt(prompt:str, default:int=0, minvalue:Optional[int]=None, maxvalue:Optional[int]=None) -> int:
    return int(EnterFloat(prompt, default, -1, minvalue, maxvalue))

def OkBox(message:str) -> None:
    sem.OKBox(message)