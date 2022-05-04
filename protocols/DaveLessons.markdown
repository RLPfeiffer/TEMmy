Dave Lessons
------------

To learn the basics and get authoritative information on the SerialEM scripting system, go here:
    https://bio3d.colorado.edu/SerialEM/stableHlp/html/about_scripts.htm
    https://bio3d.colorado.edu/SerialEM/stableHlp/html/script_commands.htm

These are lessons and best practices for scripting SerialEM in the language we call "Dave", that go beyond what's in the manual.

---

## Don't assign ReportedValue variables

Built-in SerialEM commands usually return values by setting the global variables $ReportedValue1, $ReportedValue2, etc.

However, this is not a proper way to return values from your own functions. If you write a function that assigns a value to a ReportedValue variable, your function will assign the variable as expected, but **any subsequent built-in command call will not overwrite the value you assigned.** This means you will be unable to access any built-in command return value that would normally be in the variable you set.

The best way I have found for returning values from functions, is actually to assign a differently named global variable, and expect the caller to retrieve the value from that variable using its name.

## Test in a new SerialEM window

Because unencapsulated use of global variables is typical in SerialEM scripting, variable values get stuck in SerialEM's state the first time you run your test. If you later move a variable assignment somewhere else, resulting in an incorrect access to an unassigned variable, **your test will still work because the first test's state is preserved.** This can lead you to believe your script is correct when it isn't, and the next time you run your script in a fresh SerialEM window, it can halt your capture.

The best practice is, once your script passes its test correctly, you must Save Package, close SerialEM, open it again, and test again. You are likely to find new errors.

## Different constants on multiple scopes

If you run SerialEM on more than one microscope, you may find that your scripts need to use different argument values depending on which scope is running it.

**Do not maintain two versions of the same script.** Instead, extract all the values that are different between your microscopes into a separate script which sets those values.

For example, on one microscope:

```
MacroName ScopeUtil

# Globally set the variable $ScopeName
ScopeName = TEM1
```

On another microscope:

```
MacroName ScopeUtil

# Globally set the variable $ScopeName
ScopeName = TEM2
```

On both microscopes:

```
MacroName Messages

## Incorrect:
# Call ScopeUtil
## Won't work because other SerialEM_Scripts calling Messages::SendMessage will not run code outside of the SendMessage function

Function SendMessage 0 1 Text
   # Correct:
   Call ScopeUtil
   OkBox Message from $ScopeName: $Text
EndFunction
```