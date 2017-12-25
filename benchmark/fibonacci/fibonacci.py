# -*- coding: utf-8 -*-

import time

def fib(n):
    if n == 0:
        return 0
    if n == 1:
        return 1
    return fib(n-1)+fib(n-2) 

if __name__ == '__main__':
    N = 40
    start = time.time()
    fibonacci = fib(N)
    end = time.time()

    print("python", fibonacci, end-start, "sec.")
