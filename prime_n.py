#!/usr/bin/python3
import sys
import time
import math
from typing import Callable

def is_prime(n:int)->bool:
    if n==2:
        return True
    elif n<2 or n%2==0:
        return False
    else:
        sqrt = int(math.floor(math.sqrt(n)))+1
        return not any(map(lambda x: n%x==0, range(3,sqrt,2)))

def gen_primes(gen:int,callback:Callable[[int,int,int],None]):
    counter=0
    n=0
    while counter<=gen:
        if is_prime(n):
            counter+=1
            callback(counter,n,time.perf_counter_ns())
        n+=1


def main(cum:int,width:int):
    start = time.perf_counter_ns()
    with open("./benchmark_python","w") as file:
        def prime_callback(counter:int,prime_val:int,end_time:int):
            f_str = f"{counter},{prime_val},{end_time-start}\n"
            if not counter%width:
                file.write(f_str)
        gen_primes(cum,prime_callback)
        file.flush()
    

if __name__=="__main__":
    main(int(sys.argv[1]),int(sys.argv[2]))