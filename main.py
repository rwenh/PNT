from trial_division import is_prime , primes_up_to, nth_prime, prime_count

'''test is_prime'''
test_cases = [(1, False),(2, True),(4,False),(9, False),(13,True),(97,True)]
for number, expected in test_cases:
    result= is_prime(number)
    status="PASS" if result ==expected else "FAIL"
    print(f"{status}|is_prime({number})={result}")
"""all primes up to a limit"""
print(primes_up_to(50))
for ceiling in[100,500, 1000]:
    print(f"primes up to {ceiling}:{prime_count(ceiling)}found")
'''nth prime'''
for n in[1, 5, 10, 50, 100]:
    print(f"prime#{n}:{nth_prime(n)}")
'''Gaps between primes'''
primes=primes_up_to(115)
for i, p in enumerate(primes):
    if i==0:
        print(f"{p}-first prime")
    else:
        print(f"{p} gap: +{p-primes[i-1]}")
