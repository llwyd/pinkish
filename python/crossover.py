import numpy as np
import matplotlib.pyplot as plt
from matplotlib.widgets import Slider, Button, RadioButtons
from scipy import signal
from scipy.io.wavfile import write
import dsp
import fp_dsp as fp
import q_arithmetic as qmath
from tqdm import tqdm

num_bands = 4
fs = 48000
sig_len = fs 
order = 1
fig, ax = plt.subplots(figsize=(8,6))

plt.xlim(1,fs/2)
plt.ylim(-30,5)
plt.xlabel('Frequency (Hz)')
plt.ylabel('Magnitude (dB)')

freqs = dsp.calculate_bands(num_bands,20,fs)
freqs = freqs[1:4]
num_bands -= 2
eq_bands = []
for i in range(0,num_bands):
    eq_bands.append(dsp.EQButterBand2(freqs[i],freqs[i+1],fs,order, 0.0))

h = signal.unit_impulse(sig_len)
H, Hf, Hdb = dsp.fft(h, fs, sig_len)    
ly, = ax.semilogx(Hf,Hdb)

f = []
Fdb = []
Ff = []
F = []
axfreq = []
slider = []
y = np.zeros(sig_len)
x_pos = 0.2
x_inc = 0.05
y_pos = 0.01
y_inc = 0.0

for i in range(0, num_bands):
    h = signal.unit_impulse(sig_len)
    m = signal.sosfilt(eq_bands[i].sos,h)
    y += m
    M, Mf,Mdb = dsp.fft(m, fs, sig_len)    
    f.append(m)
    F.append(M)
    Fdb.append(Mdb)
    Ff.append(Mf)
    
    ax.semilogx(Mf,Mdb)

# Cross over so it can be subtracted
z = signal.unit_impulse(sig_len)
for i in range(0, num_bands):
    z = signal.sosfilt(eq_bands[i].sos,z)
    

Z, Zf,Zdb = dsp.fft(z, fs, sig_len)    
ax.semilogx(Zf,Zdb)

y +=z 

Y, Yf,Ydb = dsp.fft(y, fs, sig_len)    
ly.set_ydata(Ydb)

plt.show()
