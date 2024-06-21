import math
def isprime(x:int):
    if x<2:
        return False
    factor = int(math.sqrt(x))
    for i in range(2,factor+1):
        if not x%i:
            return False
    return True

def gen_primes(n:int):
    counter=0
    next_int=0
    while counter<n:
        if isprime(next_int):
            counter+=1
        next_int+=1
    return next_int-1

def main():
    n_primes = 100000
    print(f"{n_primes}th prime is {gen_primes(n_primes)}")

main()