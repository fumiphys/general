# -*- coding: utf-8 -*-

import matplotlib.pyplot as plt
import numpy as np

if __name__ == '__main__':
    x = np.random.random(10000)

    plt.hist(x, bins=100, log=False)
    plt.show()
