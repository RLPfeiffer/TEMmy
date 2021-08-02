#! /usr/bin/env python

# Build/push all the TEMmy tools to where they belong

if __name__ == "__main__":
    from datetime import datetime
    import os
    import shutil

    # Get a timestamp for reuse:
    timestamp = datetime.now().strftime("%Y-%m-%d-%H-%M-%S")

    # Package TEM macros
    def write_macro(macro, content, file):
        num, name, *_ = macro.split('-')
        is_python = name.endswith(".py")
        name = name.replace('.py', '')
        name = name.replace('.txt', '')
        try:
            # Macros with number prefix will be put through this function first because of lexicographical ordering
            num = int(macro.split("-")[0])
            file.write(f"Macro\t{num}\n")
            if not is_python:
                file.write(f"MacroName {name}\n")
            if is_python and name != "Util":
                # Write the python macro boilerplate
                file.write("#!Python3.9\n")
                file.write(f"#MacroName {name}\n")
                file.write("#include Util\n")
                file.write("import serialem as sem\n")
            file.write(f"{content}\n")
            if name != "Util":
                file.write("EndMacro\n")
        except:
            # Macros with the Util prefix will come through last
            file.write(f"############# {macro} #############\n\n{content}\n\n")

    macros = sorted(os.listdir('macros'))
    tem1_package_name = f"tem1package-{timestamp}.txt"
    tem2_package_name = f"tem2package-{timestamp}.txt"
    with open(tem1_package_name, "w") as macro_package_tem1, open(tem2_package_name, "w") as macro_package_tem2:
        macro_package_tem1.write("MaxMacros\t40\n")
        macro_package_tem2.write("MaxMacros\t40\n")
        for macro in macros:
            if macro.endswith(".txt") or macro.endswith(".py"):
                with open(f'macros/{macro}', "r") as macro_file: 
                    macro_content = macro_file.read()
                    if "tem1" in macro.lower():
                        write_macro(macro, macro_content, macro_package_tem1)
                    elif "tem2" in macro.lower():
                        write_macro(macro, macro_content, macro_package_tem2)
                    else:
                        write_macro(macro, macro_content, macro_package_tem1)
                        write_macro(macro, macro_content, macro_package_tem2)
        # Util needs EndMacro statement
        macro_package_tem1.write("EndMacro\n")
        macro_package_tem2.write("EndMacro\n")

    shutil.copyfile(tem1_package_name, f"Y:/DROPBOX/nat/{tem1_package_name}")
    shutil.copyfile(tem2_package_name, f"Y:/DROPBOX/nat/{tem2_package_name}")