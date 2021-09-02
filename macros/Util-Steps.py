from typing import Callable
from typing import Any

# A Step is a function without arguments. Its return value will be ignored
Step = Callable[[], None]

# Types of protocol step for Tutorial.py:
def TellOperator(message:str) -> Step:
    return lambda: OkBox(message)

def DoAutomatically(func:Callable[[], None]) -> Step:
    def step() -> None:
        func()
        RunNextStep()
    return step