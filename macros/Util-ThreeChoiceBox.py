from typing import List

def ThreeChoiceBox(header:str, choice1:str, choice2:str, choice3:str, prompt1:str="", prompt2:str="", prompt3:str="") -> str:
    sem.SetVariable("header", header)
    sem.SetVariable("prompt1", prompt1)
    sem.SetVariable("prompt2", prompt2)
    sem.SetVariable("prompt3", prompt3)
    sem.SetVariable("buttons", listToSEMarray([choice1, choice2, choice3])) # type: ignore
    # Calls a traditional SerialEM macro to pass these values onward because ThreeChoiceBox is not implemented in the serialem module:
    sem.Call("ThreeChoiceBox")
    choice:int = int(sem.GetVariable("reportedValue1"))
    if choice == 1:
        return choice1
    if choice == 2:
        return choice2
    if choice == 3:
        return choice3
    
    raise ValueError("SerialEM didn't set reportedValue1 correctly")

def ManyChoiceBox(header:str, choices:List[str], prompt1:str="", prompt2:str="", prompt3:str="") -> str:
    '''Recursively call ThreeChoiceBox to present an infinite number of choices'''
    if len(choices) < 2:
        raise ValueError("it doesn't make sense to call ManyChoiceBox with < 2 choices.")
    for choice in choices:
        assert choice != "More", "'More' cannot be used as a choice in ManyChoiceBox"
    Choice1 = choices.pop(0)
    Choice2 = choices.pop(0)
    Choice3 = "More"   
    if len(choices) == 0:
        Choice3 = ""
    elif len(choices) == 1:
        Choice3 = choices.pop(0)

    ChosenValue = ThreeChoiceBox(header, Choice1, Choice2, Choice3, prompt1, prompt2, prompt3)
    if ChosenValue == "More":
        return ManyChoiceBox(header, choices, prompt1, prompt2, prompt3)
    else:
        return ChosenValue


