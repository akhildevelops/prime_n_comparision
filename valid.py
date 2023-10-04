import os
from typing import List,Dict
import sys
master_file = "1m.csv"

from zipfile import ZipFile

def get_all_benchmark_files(pattern:str)->List[str]:
    return [obj for obj in os.listdir(None) if obj.startswith(pattern)]

def unzip(path:str):
    with ZipFile(path) as zfile:
        zfile.extractall()

def get_primes(file:str)->set:
    with open(file) as file_o:
        data = file_o.readlines()
    return {i.split(",")[0] for i in data}

def load_primes(file:str)->Dict[str,str]:
    primes = {}
    with open(file) as prime_file:
        data = prime_file.readlines()
    for i in data:
        n_prime, prime, _ = i.split(",")
        primes[n_prime]=prime
    return primes

def main():
    benchmark_lang = sys.argv[1] if len(sys.argv)>1 else None
    benchmark_pattern = "benchmark_"
    unzip("./1m.csv.zip")
    benchmark_files = (benchmark_lang and [f"{benchmark_pattern}{benchmark_lang}"]) or get_all_benchmark_files(benchmark_pattern)
    one_m_primes = load_primes("./1m.csv")
    for file in benchmark_files:
        is_valid=True
        lang = file.replace(benchmark_pattern,'')
        print(f"Validating {lang}")
        b_primes = load_primes(file)
        for n_prime, prime in b_primes.items():
            one_m_prime_val = one_m_primes.get(n_prime)
            if not one_m_prime_val:
                print("Only first one million primes can be validated. Skipping the file.")
                is_valid=False
                break
            if prime!=one_m_prime_val:
                print(f"{n_prime}th prime should be {one_m_prime_val} but got {prime}.")
                is_valid=False
                break
        if is_valid:
            print(f"Ok\n")
        else:
            print(f"Not Ok\n")
            sys.exit(1)

if __name__=="__main__":
    main()