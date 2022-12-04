#!/usr/bin/python3.9
import shutil, os, sys, pathlib

# check if romfs exists
if not os.path.exists("../romfs.zip"):
  print("ERROR: no romfs zip!")
  exit("romfs.zip was missing!")

# if distribution folder exists, delete it
if "build" in os.listdir('..'):
  shutil.rmtree('../build')
os.makedirs('../build/')
shutil.unpack_archive("../romfs.zip", "../build/")
shutil.move("../plugin/libwubor.nro", "../build/ultimate/mods/The WuBor Patch/plugin.nro")

# zip each folder in the staging dir
shutil.make_archive("The_WuBor_Patch", 'zip', '../build')
shutil.move("The_WuBor_Patch.zip", "../The_WuBor_Patch.zip")
