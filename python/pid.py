import numpy as np
import matplotlib.pyplot as plt
from matplotlib.widgets import Slider, Button, RadioButtons
from scipy import signal
from scipy.io.wavfile import write
import dsp
import q_arithmetic as qmath
from tqdm import tqdm
import random as r

class PID():
    def __init__(self, p_gain, i_gain, d_gain):
        self.p_gain = p_gain
        self.i_gain = i_gain
        self.d_gain = d_gain

        self.integral = 0.0
        self.previous_error = 0.0

    def next(self, set_point, x):
        error = set_point - x
        self.integral += error
        delta = error - self.previous_error

        if self.integral > 1.0:
            self.integral = 1.0
        elif self.integral < 0:
            self.integral = 0
        
        p = error * self.p_gain
        i = self.integral * self.i_gain
        d = delta * self.d_gain

        self.previous_error = error

        return p + i + d



fs = 48000
sig_len = 1024 * 4
f = 100

t = np.linspace(0,sig_len,sig_len)
x = np.sin(2 * np.pi * f * (t/fs))
z = np.zeros(sig_len)

cutoff = 1.0
alpha = dsp.get_alpha(cutoff,fs)

pid = PID(1.0,0.001,0.1)


gain = 1.0
set_point = 0.3
y = np.zeros(sig_len)
rms_plot = np.zeros(sig_len) 
gain_plot = np.zeros(sig_len) 
rms = 0.0

for i in range(sig_len):
    gain_plot[i] = gain
    z[i] = x[i]
    x[i] *= gain
    rms = x[i] - alpha*(x[i] - rms)
    rms_plot[i] = rms
    gain = pid.next(set_point, rms)
    y[i] = x[i]

plt.plot(z)
plt.plot(y)
plt.plot(rms_plot)
plt.plot(gain_plot)
plt.show()
