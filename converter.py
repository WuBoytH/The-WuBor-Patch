# -*- coding: utf-8 -*-
"""
Created on Wed May  5 14:44:20 2021

@author: Sensei-NDES
"""
### ======================================================================== ###
### ======================================================================== ###
### Written with Python 3.7                                                    #
###                                                                            #
### USAGE:                                                                     #
### 1. Place in folder containing ACMD script from data viewer in .txt file    #
### 2. Run script                                                              #
###    - Output will be "*.txt_output" files                                   #
### ======================================================================== ###
### ======================================================================== ###

import os
import re
import sys
import shutil

# Testing, ignore this

def convert(the_dir: str, output_folder: str):
    # List all files in a directory using os.listdir
    basepath = the_dir + "\src"
    print(basepath)
    txt_files = [] # Blank list to hold list of all rust files
    for entry in os.listdir(basepath):
        if os.path.isfile(os.path.join(basepath, entry)):
            print(entry)
            if "acmd.rs" in entry and not ".rs_output" in entry:
                txt_files.append(os.path.join(basepath, entry))
            if "status.rs" in entry and not ".rs_output" in entry:
                txt_files.append(os.path.join(basepath, entry))

    print(txt_files)
    for file in txt_files:
        with open(file, "r+") as f_in:
            data = f_in.read()
            f_in.seek(0, 0)
            f_in.write('use super::*;\n\n' + data)