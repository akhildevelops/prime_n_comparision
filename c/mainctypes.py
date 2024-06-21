from ctypes import cdll

lib = cdll.LoadLibrary("./libprime.so")

def main():
    n_primes = 100000
    print(f"{n_primes}th prime is {lib.gen_primes(100000)}")

main()