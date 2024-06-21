import ctypes
from ctypes import cdll

libc = cdll.LoadLibrary("libc.so.6")

x = b"Hello"
libc.strchr.restype=ctypes.c_char_p
libc.strchr.argtypes=[ctypes.c_char_p,ctypes.c_char]

print(libc.strchr(x,b'l'))  

print(libc.rand())