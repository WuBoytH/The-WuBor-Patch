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
shutil.move("../target/plugin.nro", "../build/ultimate/mods/wubor/plugin.nro")