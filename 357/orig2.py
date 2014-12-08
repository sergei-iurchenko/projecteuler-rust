# coding=utf-8
from time import time
 
def get_primes_below(n):
    '''
    В качестве функции get_primes_below - выберите любую отсюда http://stackoverflow.com/questions/2068372
    '''
    n, correction = n - n % 6 + 6, 2 - (n % 6 > 1)
    sieve = [True] * (n / 3)
    for i in xrange(1, int(n ** .5) / 3 + 1):
        if sieve[i]:
            k = 3 * i + 1 | 1
            sieve[k * k / 3::2 * k] = [False] * ((n / 6 - k * k / 6 - 1) / k + 1)
            sieve[k * (k - 2 * (i & 1) + 4) / 3::2 * k] = [False] * (
                (n / 6 - k * (k - 2 * (i & 1) + 4) / 6 - 1) / k + 1)
    return [2, 3] + [3 * i + 1 | 1 for i in xrange(1, n / 3 - correction) if sieve[i]]
 
 
def check(n):
    if not n % 10 in [0, 2, 8]:
        return False
    if not n / 2 + 2 in p:
        return False
    for d in xrange(3, int(n ** .5)):
        if n % d == 0 and not d + (n / d) in p:
            return False
    return True
 
 
t = time()
A = 10 ** 6
p = set(get_primes_below(A))
# p = get_primes_below(A)
R = 1 + 6 + sum(x-1 for x in p if check(x-1))
 
print 'result = {}, time = {}'.format(R, time() - t)