import numpy as np
import matplotlib.pyplot as plt
from matplotlib.widgets import Slider, Button, RadioButtons
from scipy import signal
from scipy.io.wavfile import write
import dsp
import q_arithmetic as qmath
from tqdm import tqdm
import random as r


def ewma( dirac, alpha, length ):
    y = np.zeros( length )
    for i in range( length ):
        y[i] = dirac[i] - alpha*(dirac[i] - y[i-1])
    return y

fs = 48000
sig_len = fs * 20
f = 100



t = np.linspace(0,sig_len,sig_len)
x = np.sin(2 * np.pi * f * (t/fs))
z = np.zeros(sig_len)

x[fs*5:fs*10] *= 0.1


cutoff = 100 # default = 1
alpha = dsp.get_alpha(cutoff,fs)

#rms = ewma(np.abs(x),alpha,sig_len)
rms = np.zeros(sig_len)
rms[:] = 0.0

g = np.zeros(sig_len)
set_point = 0.3

# Gain needs to be reset upon slider transition
gain = 0.00001
delta_scale = 0.001
exp_scale = 0.0075 # default = 0.001
for i in range(sig_len):

    z[i] = x[i]
    z[i] *= gain
    
    delta = set_point - rms[i - 1]
    #gain = 1.0 + (delta * set_point_rec)
    #gain -= delta
   
    #gain *= 1.0 + (delta * delta_scale)
    gain *= 1.0 - (1 - (np.e ** (exp_scale *delta)))
    rms[i] = np.abs(z[i]) - alpha*(np.abs(z[i]) - rms[i-1])
    g[i] = gain
    #print(f'g: {gain}')


plt.plot(x)
plt.plot(z)
plt.plot(rms)
plt.plot(g)
plt.show()
