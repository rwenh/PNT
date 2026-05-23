import math

def is_prime(n):
    if n <2:
        return False
    if n==2:
        return True
    if n%2==0:
        return False
    for divisor in range (3, math.isqrt(n)+1,2):
        if n%divisor==0:
            return False
        return True
def primes_up_to(limit):
    if limit <2:
        return []
    return [n for n in range(2, limit+1)if is_prime(n)]
def nth_prime(n):
    if n<1:
        raise ValueError("n must be 1 or Greater")
    count, candidate = 0, 1
    while count<n:
        candidate +=1
        if is_prime(candidate):
            count+=1
            return candidate
def prime_count(limit)->int:
    return len(primes_up_to(limit))

