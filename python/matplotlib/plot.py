# -*- coding: utf-8 -*-

import matplotlib.pyplot as plt
import numpy as np
import math

if __name__ == '__main__':
    x = np.arange(0, math.pi*2.2, 0.01)
    y = np.sin(x)

    plt.plot(x, y, label='sin(x)', color='green', linestyle='dashed')
    plt.legend()
    plt.show()
