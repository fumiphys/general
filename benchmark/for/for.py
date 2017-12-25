# -*- coding: utf-8 -*-

import time

if __name__ == '__main__':
    sum = 0
    N = 1000000
    start = time.time()
    for i in range(N):
        sum += i
    end = time.time()

    print("python", sum, end-start, "sec.")
