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
        num = int(macro.split("-")[0])
        file.write("Macro\t{}\n".format(num))
        file.write("{}\n".format(content))
        file.write("EndMacro\n")
    macros = sorted(os.listdir('macros'))
    tem1_package_name = "tem1package-{}.txt".format(timestamp)
    tem2_package_name = "tem2package-{}.txt".format(timestamp)
    with open(tem1_package_name, "w") as macro_package_tem1, open(tem2_package_name, "w") as macro_package_tem2:
        macro_package_tem1.write("MaxMacros\t40\n")
        macro_package_tem2.write("MaxMacros\t40\n")
        for macro in macros:
            if macro.endswith(".txt"):
                with open('macros/{}'.format(macro), "r") as macro_file: 
                    macro_content = macro_file.read()
                    if "tem1" in macro.lower():
                        write_macro(macro, macro_content, macro_package_tem1)
                    elif "tem2" in macro.lower():
                        write_macro(macro, macro_content, macro_package_tem2)
                    else:
                        write_macro(macro, macro_content, macro_package_tem1)
                        write_macro(macro, macro_content, macro_package_tem2)
    shutil.copyfile(tem1_package_name, "Y:/DROPBOX/nat/{}".format(tem1_package_name))
    shutil.copyfile(tem2_package_name, "Y:/DROPBOX/nat/{}".format(tem2_package_name))