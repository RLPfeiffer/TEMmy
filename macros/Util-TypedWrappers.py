def EnterDefaultedFloat(default:float, text:str, decimal_places:int = 2) -> float:
    return float(sem.EnterDefaultedNumber(default, decimal_places, text))

def EnterDefaultedInt(default:int, text:str) -> int:
    return int(EnterDefaultedFloat(default, text, -1))