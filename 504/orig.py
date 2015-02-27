#coding: utf-8
from itertools import product
import time
 
m = 31
 
squares = set([x * x for x in range(2 * m)])
 
delimeters = {1: set({})}  # Словарь делителей чисел
for x, y in product(xrange(2, m + 1), repeat=2):
    if x % y == 0:
        delimeters.setdefault(x, set({})).add(y)
 
diagonals = {}
diagonals[(1, 1)] = 1  # Кол-во точек на диагонали прямоугольника x,y -1.
for x, y in product(xrange(1, m + 1), repeat=2):
    if (x, y) not in diagonals:
        diagonals[(x, y)] = 1
        common_delimeters = delimeters[x].intersection(delimeters[y])
        for p in common_delimeters:
            if (x / p, y / p) in diagonals:
                diagonals[(x, y)] = p * diagonals[(x / p, y / p)]
                break
 
 
def count_tr(a, b):
    return (a * b - a - b - diagonals[a, b]) / 2 + 1
 
 
def count_q(a, b, c, d):
    count = a + b + c + d - 3  # точки на осях
    count += count_tr(a, b)  # точки внутри треугольника
    count += count_tr(b, c)
    count += count_tr(c, d)
    count += count_tr(d, a)
    return count
 
 
t1 = time.clock()
result = 0
for a, b, c, d in product(range(1, m + 1), repeat=4):
    if count_q(a, b, c, d) in squares:
        result += 1
 
print(result)
print(time.clock() - t1)
 
# 4   42     0
# 10  862    0.01
# 20  5582   0.3
# 40  45655  4.7
# 50  88013  11
# 100 694687
