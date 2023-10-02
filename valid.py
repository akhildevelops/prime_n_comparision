from functools import reduce
files = ["benchmark_python","benchmark_rust"]

def get_primes(file:str)->set:
    with open(file) as file_o:
        data = file_o.readlines()
    return {i.split(",")[0] for i in data}

assert reduce(lambda x,y: x-get_primes(y),files,set())==set()