import os

def fix_file(path: str, macro: str):
    file = open(path, "r")
    file_as_str = file.read()
    file.close()

    macro_start = file_as_str.find(macro)

    if macro_start < 0:
        return

    paren_open = file_as_str.find("!(", macro_start)
    paren_close = file_as_str.find(")", macro_start)
    subslice = file_as_str[(paren_open + 2):paren_close]
    no_whitespace = "".join(subslice.split())

    new_installs = ""
    for function_name in no_whitespace.split(','):
        new_installs += function_name + "::install();\n    "

    new_file_text = file_as_str[:macro_start] + new_installs.strip() + file_as_str[(paren_close + 2):]

    file = open(path, "w")
    file.write(new_file_text)

for root, dirs, files in os.walk("."):
    for file in files:
        if file.endswith("rs"):
            fix_file(os.path.join(root, file), "install_acmd_scripts!")
            fix_file(os.path.join(root, file), "install_status_scripts!")
            fix_file(os.path.join(root, file), "install_agent_init_callbacks!")
            fix_file(os.path.join(root, file), "install_agent_reset_callbacks!")
            fix_file(os.path.join(root, file), "install_agent_resets!")
            fix_file(os.path.join(root, file), "install_agent_frames!")