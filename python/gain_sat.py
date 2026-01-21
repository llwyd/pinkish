import numpy as np
import matplotlib.pyplot as plt
from matplotlib.widgets import Slider, Button, RadioButtons
from scipy import signal
from scipy.io.wavfile import write
import dsp
import q_arithmetic as qmath
from tqdm import tqdm
import random as r


def logistic_func(L, k, delta):
    return L / ( 1 + (np.e ** (-k * delta)))


sig_len = 1000
t = np.linspace(0,5,sig_len)
max_gain = 10
x = logistic_func(max_gain, 1, 5 - t) - (max_gain / 2)



plt.plot(t,x)
plt.xlabel("Gain in")
plt.ylabel("GAIN out")
plt.show()
