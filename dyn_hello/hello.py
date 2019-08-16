from ctypes import cdll
import os

if os.name == 'nt':
    dyn_lib_name = 'dyn_hello.dll'
else:
    pass
    
clib = cdll.LoadLibrary(
    os.path.join(os.path.dirname(os.path.abspath(__file__)) , 
    '..' ,
    'target' , 
    'debug' ,
    dyn_lib_name))

clib.hello()