from typing import Callable
from typing import Any

# A Step is a function without arguments. Its return value will be ignored
Step = Callable[[], None]

# Types of protocol step for Tutorial.py:

# Display an instruction that must be carried out manually:
def TellOperator(message:str) -> Step:
    return lambda: OkBox(message)

# Do something through SerialEM Scripting and keep going when it's finished:
def DoAutomatically(func:Callable[[], None]) -> Step:
    def step() -> None:
        func()
        RunNextStep()
    return step

def DoNothing() -> None:
    pass

# Do a step differently depending on the scope being used
def DependingOnScope(tem1Step:Step, tem2Step:Step) -> Step:
    def step() -> None:
        if ScopeName == "TEM1":
            tem1Step()
        elif ScopeName == "TEM2":
            tem2Step()
        else:
            raise ValueError(f"ScopeName has unexpected value {ScopeName}")
    return step