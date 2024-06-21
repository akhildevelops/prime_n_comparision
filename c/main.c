#include <prime.h>
#include <stdio.h>

int main()
{
    int n_primes = 100000;
    printf("%dth prime is %d\n", n_primes, gen_primes(n_primes));
    // gen_primes(100000);

    return 0;
}