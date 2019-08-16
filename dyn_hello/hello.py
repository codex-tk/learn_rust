from ctypes import cdll
import os

print("Start")

clib = cdll.LoadLibrary("D:\\works\\learn_rust\\target\\debug\\dyn_hello.dll")

print("End")
clib.hello()