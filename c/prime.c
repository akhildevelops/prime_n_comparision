#include <stdio.h>
#include <math.h>

int isprime(int x)
{
    if (x < 2)
    {
        return 1;
    }
    int factor = sqrt(x);
    for (int i = 2; i <= factor; i++)
    {
        if (x % i == 0)
        {
            return 1;
        }
    }
    return 0;
}

int gen_primes(int n)
{
    int counter = 0;
    int next_int = 0;
    while (counter < n)
    {
        if (isprime(next_int) == 0)
        {
            counter += 1;
        }
        next_int += 1;
    }
    return next_int - 1;
}