def ThreeChoiceBox(header:str, choice1:str, choice2:str, choice3:str, prompt1:str="", prompt2:str="", prompt3:str="") -> str:
    sem.SetVariable("header", header)
    sem.SetVariable("prompt1", prompt1)
    sem.SetVariable("prompt2", prompt2)
    sem.SetVariable("prompt3", prompt3)
    sem.SetVariable("buttons", listToSEMarray([choice1, choice2, choice3])) # type: ignore
    sem.Call("ThreeChoiceBox")
    choice:int = int(sem.GetVariable("reportedValue1"))
    if choice == 1:
        return choice1
    if choice == 2:
        return choice2
    if choice == 3:
        return choice3
    
    raise ValueError("SerialEM didn't set reportedValue1 correctly")