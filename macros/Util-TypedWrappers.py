def EnterDefaultedFloat(default:float, text:str, decimal_places:int = 2) -> float:
    return float(sem.EnterDefaultedNumber(default, decimal_places, text))

def EnterDefaultedInt(default:int, text:str) -> int:
    return int(EnterDefaultedFloat(default, text, -1))

def EnterString(prompt:str) -> str:
    sem.EnterString("RESERVEDVAR", prompt)
    return str(sem.GetVariable("RESERVEDVAR"))

def YesNoBox(prompt:str) -> bool:
    return int(sem.YesNoBox(prompt)) == 1