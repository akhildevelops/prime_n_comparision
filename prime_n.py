#!/usr/bin/python3
import sys
import time
import math
from typing import Callable

def is_prime(n:int)->bool:
    if n<2:
        return False
    for i in range(2,n//2+1):
        if n%i==0:
            return False
    return True

def gen_primes(gen:int,callback:Callable[[int,int,int],None]):
    counter=0
    n=0
    while counter<=gen:
        if is_prime(n):
            counter+=1
            callback(counter,n,time.perf_counter_ns())
        n+=1



def main(cum:int,width:int):
    n_time=[]
    start = time.perf_counter_ns()
    callable = lambda counter,prime_val,end: counter%width or n_time.append(f"{counter},{prime_val},{math.log10(end-start)}\n")
    gen_primes(cum,callable)
    with open("./benchmark_python","w") as file:
        file.writelines(n_time)

if __name__=="__main__":
    print(sys.argv)
    main(int(sys.argv[1]),int(sys.argv[2]))