# -*- coding: utf-8 -*-
'''long long sample
'''


def fac(n):
    if n == 1:
        return 1
    elif n == 0:
        return 1
    return n * fac(n - 1)


print(fac(100))
