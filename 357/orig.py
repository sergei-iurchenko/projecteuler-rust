# coding=utf-8
from time import time


def get_primes_below(n):
    """ Returns  a list of primes < n """
    sieve = [True] * n
    q = int(n**0.5)+1
    for i in xrange(3, q, 2):
        if sieve[i]:
            sieve[i*i::2*i]=[False]*((n-i*i-1)/(2*i)+1)
    result = [2] + [i for i in xrange(3,n,2) if sieve[i]]
    return result

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
A = 10 ** 8
p1 = get_primes_below(A)
print('time = {}'.format(time() - t))
p = set(p1)

print('time = {}'.format(time() - t))

R = 1 + 6 + sum(x-1 for x in p if check(x-1))
print('time = {}'.format(time() - t))

# print 'result = {}, time = {}'.format(1, time() - t)
