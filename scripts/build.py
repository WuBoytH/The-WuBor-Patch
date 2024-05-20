#!/usr/bin/python3
import shutil, os, sys, characters

build = True
dev = False
dev_chars = ''
only_dev = False
non_dev_chars = ''

arg_len = sys.argv.__len__()
dev_index = -1
if "-dev" in sys.argv:
    dev_index = sys.argv.index("-dev")
elif "--d" in sys.argv:
    dev_index = sys.argv.index("--d")
ip_index = -1
ip = ''
if "-ip" in sys.argv:
    ip_index = sys.argv.index("-ip")
elif "--i" in sys.argv:
    ip_index = sys.argv.index("--i")

full_args = ''
if "pr" in sys.argv:
    full_args = ' --no-default-features --features=pr'
elif "dev" in sys.argv:
    full_args = ' --no-default-features --features=dev'
elif "main" in sys.argv:
    full_args = ' --no-default-features --features=main'

if sys.argv.count("-help") > 0 or sys.argv.count("--h") > 0:
    print("This is the WuBor Patch build script.")
    print("Please pass arguments into the script to build WuBor.")
    print("Arguments:")
    print("    -dev, --d: Indicate you're building a Dev plugin.")
    print("               List a set of characters afterwards to build a dev plugin for them.")
    print("               Pass in \"only\" to only push the dev plugin.")
    print("               ex. build.py --d only sonic captain")
    print("    -ip,  --i: Indicate the IP of the console you're sending the plugin to.")
    print("               ex. build.py --i 192.168.1.11")
    build = False
else:
    if len == 1:
        print("This script can be given arguments! Add -help or --h for more information.")
    if dev_index != -1:
        dev = True
        dev_index += 1
        while dev_index < arg_len:
            if sys.argv[dev_index] == 'only':
                only_dev = True
            elif sys.argv[dev_index] in characters.characters:
                dev_chars += (sys.argv[dev_index])
            else:
                break
            dev_index += 1
            if dev_index < arg_len:
                dev_chars += ','
        for character in characters.characters:
            if character not in sys.argv:
                if non_dev_chars != '':
                    non_dev_chars += ','
                non_dev_chars += character

if build:
    if os.path.isfile("../target/plugin.nro"):
        os.remove("../target/plugin.nro")

    if os.path.isfile("../target/development.nro"):
            os.remove("../target/development.nro")

    if not dev:
        build_command = 'cargo skyline build --release' + full_args
        # print(build_command)
        print("Building Full Plugin!")

        if os.path.isfile("../target/aarch64-skyline-switch/release/libwubor.nro"):
            os.remove("../target/aarch64-skyline-switch/release/libwubor.nro")

        os.system(build_command)

        if os.path.isfile("../target/aarch64-skyline-switch/release/libwubor.nro"):
            print("Plugin built successfully! Copying to the target folder as plugin.nro")
            shutil.copy(
                "../target/aarch64-skyline-switch/release/libwubor.nro",
                "../target/plugin.nro"
            )
        else:
            print("Something went wrong! Please read any compiler errors you see.")
        if ip_index != -1:
            ip = sys.argv[ip_index + 1]
            os.system('curl -T ../target/plugin.nro ftp://' + ip + ':5000/ultimate/mods/wubor/plugin.nro')
    else:
        if not only_dev:
            os.environ["CARGO_TARGET_DIR"] = os.path.join("../target", "dev_base")
            build_command = 'cargo skyline build --release --no-default-features --features=local,main_nro,' + non_dev_chars
            # print(build_command)
            print("Building Base Dev Plugin!")

            if os.path.isfile("../target/dev_base/aarch64-skyline-switch/release/libwubor.nro"):
                os.remove("../target/dev_base/aarch64-skyline-switch/release/libwubor.nro")

            os.system(build_command)

            if os.path.isfile("../target/dev_base/aarch64-skyline-switch/release/libwubor.nro"):
                print("Plugin built successfully! Copying to the target folder as plugin.nro")
                shutil.copy(
                    "../target/dev_base/aarch64-skyline-switch/release/libwubor.nro",
                    "../target/plugin.nro"
                )
            else:
                print("Something went wrong! Please read any compiler errors you see.")

        os.environ["CARGO_TARGET_DIR"] = os.path.join("../target", "dev")
        build_command = 'cargo skyline build --release --no-default-features --features=' + dev_chars
        # print(build_command)
        print("Building development.nro")

        if os.path.isfile("../target/dev/aarch64-skyline-switch/release/libwubor.nro"):
            os.remove("../target/dev/aarch64-skyline-switch/release/libwubor.nro")

        os.system(build_command)

        if os.path.isfile("../target/dev/aarch64-skyline-switch/release/libwubor.nro"):
            print("Plugin built successfully! Copying to the target folder as development.nro")
            shutil.copy(
                "../target/dev/aarch64-skyline-switch/release/libwubor.nro",
                "../target/development.nro"
            )
        else:
            print("Something went wrong! Please read any compiler errors you see.")

        if ip_index != -1:
            ip = sys.argv[ip_index + 1]
            if not only_dev:
                os.system('curl -T ../target/plugin.nro ftp://' + ip + ':5000/ultimate/mods/wubor/plugin.nro')

            switch_rom_path = "atmosphere/contents/01006a800016e000/romfs"
            development_subpath = "smashline/"

            os.system('curl -T ../target/development.nro ftp://' + ip + ':5000/' + switch_rom_path + '/' + development_subpath + 'development.nro')