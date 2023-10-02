#!/usr/bin/python3
import sys
import time
import math


def is_prime(n:int)->bool:
    if n<2:
        return False
    for i in range(2,n//2+1):
        if n%i==0:
            return False
    return True

def gen_primes(gen:int):
    counter=0
    n=0
    p=None
    while counter<gen:
        if is_prime(n):
            p=n
            counter+=1
        n+=1
    return p

def main(cum:int,n_segs:int):
    n_time=[]
    step = cum//n_segs
    for i in range(1,cum+step+1,step):
        start = time.perf_counter_ns()
        last_p = gen_primes(i)
        end = time.perf_counter_ns()
        op = f"{i},{last_p},{math.log10(end-start)}\n"
        print(op,end="")
        n_time.append(op)
    with open("./benchmark_python","w") as file:
        file.writelines(n_time)

if __name__=="__main__":
    print(sys.argv)
    main(int(sys.argv[1]),int(sys.argv[2]))