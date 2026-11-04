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

        self.integral_max = 0.01

    def next(self, set_point, x):
        error = set_point - x
        self.integral += error
        delta = (error - self.previous_error)

        if self.integral > self.integral_max:
            self.integral = self.integral_max
        elif self.integral < -self.integral_max:
            self.integral = -self.integral_max
        
        p = error * self.p_gain
        i = self.integral * self.i_gain
        d = delta * self.d_gain

        self.previous_error = error

        return p + i + d



fs = 48000
sig_len = fs * 4
f = 1000

t = np.linspace(0,sig_len,sig_len)
x = np.sin(2 * np.pi * f * (t/fs))
z = np.zeros(sig_len)

cutoff = 1.0
alpha = dsp.get_alpha(cutoff,fs)

x_100 = x**2 * 1.0
x_050 = x**2 * 0.5
x_025 = x**2 * 0.25

rms_100 = 0.0
rms_050 = 0.0
rms_025 = 0.0

for i in range(sig_len):
    rms_100 = x_100[i] - alpha*(x_100[i] - rms_100)
    rms_050 = x_050[i] - alpha*(x_050[i] - rms_050)
    rms_025 = x_025[i] - alpha*(x_025[i] - rms_025)


print(f'Exp RMS (1.00) {rms_100}')
print(f'Exp RMS (0.50) {rms_050}')
print(f'Exp RMS (0.25) {rms_025}')

pid = PID(0.00004,0.00,100)


gain = 1.0
set_point = 0.125 **2
y = np.zeros(sig_len)
rms_plot = np.zeros(sig_len) 
gain_plot = np.zeros(sig_len) 
out_plot = np.zeros(sig_len) 
rms = 0.0

for i in range(sig_len):
    gain_plot[i] = gain
    z[i] = x[i]
    x[i] *= gain
    rms = x[i]**2 - alpha*(x[i]**2 - rms)
    rms_plot[i] = rms
    out = pid.next(set_point, rms)
    
    gain += out
    out_plot[i] = out
    y[i] = x[i]


print(f'avg  err {np.average(out_plot)}')
print(f'avg gain {np.average(gain_plot)}')
print(f'avg  rms {np.average(rms_plot)} ({set_point})')

plt.plot(z)
plt.plot(y)
plt.plot(rms_plot)
plt.plot(gain_plot)
plt.plot(out_plot)
plt.legend(['original','output','rms','gain','out'])
plt.show()
