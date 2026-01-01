import numpy as np
import matplotlib.pyplot as plt
from matplotlib.widgets import Slider, Button, RadioButtons
from scipy import signal
from scipy.io.wavfile import write
import dsp
import fp_dsp as fp
import q_arithmetic as qmath
from tqdm import tqdm

num_bands = 6
fs = 48000
sig_len = fs 
order = 1
axcolor = 'lightgoldenrodyellow'
fig, ax = plt.subplots(figsize=(8,6))

plt.subplots_adjust(bottom=0.35)
plt.hlines(0.0,0,fs/2)
plt.xlim(1,fs/2)
plt.ylim(-30,5)
plt.xlabel('Frequency (Hz)')
plt.ylabel('Magnitude (dB)')

def generate(event):
    print("Generating filtered noise...")
    fp_bands = []
    q = 30
    print(f"Convert to Q{q} format...")

    for i,eq in enumerate(eq_bands):
        fp_bands.append(fp.QBand(eq.lower_cutoff,eq.upper_cutoff,fs,q, eq.gain))

    out_len = fs * 5
    #h = signal.unit_impulse(out_len)

    h = (np.random.rand(out_len) - 0.5) * 2
    h_q = qmath.to_q32(h, q)

    y_out = np.zeros(out_len,dtype=np.int32)
    for eq in tqdm(fp_bands):
        h_q = qmath.to_q32(h, q)
        ir = fp.sosfilt_rt(eq.sos_q, h_q, eq.gain_q,q)
        y_out += ir
    write('test.wav',fs,y_out)
    print("Fin")

ideal_db, ideal_f = dsp.generate_decade_line( 15, 100000 )
ax.semilogx(ideal_f, ideal_db )

axbutton = fig.add_axes([0.75, 0.1, 0.1, 0.075])
genbutton = Button(axbutton,'Generate')
genbutton.on_clicked(generate)

freqs = dsp.calculate_bands(num_bands,20,fs)
eq_bands = []
bp_gain = (num_bands - 2) * 0.465
gain = 0.0
for i in range(0,num_bands):
    eq_bands.append(dsp.EQButterBand(freqs[i],freqs[i+1],fs,order, gain))

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

def update():
    z = np.zeros(sig_len)
    
    for i in range(0, num_bands):
        h = signal.unit_impulse(sig_len)
        gain = eq_bands[i].gain_raw()
        w = signal.sosfilt(eq_bands[i].sos, h)
        w *= gain
        z += w

    return z

def update_graph(val):
    for i in range(0, num_bands):
        eq_bands[i].gain = slider[i].val

    y = update()
    Y,Yf,Ydb = dsp.fft(y,fs,sig_len)
    
    ly.set_ydata(Ydb)

for i in range(0, num_bands):
    h = signal.unit_impulse(sig_len)
    m = signal.sosfilt(eq_bands[i].sos,h)
    m *= eq_bands[i].gain_raw()
    y += m
    M, Mf,Mdb = dsp.fft(m, fs, sig_len)    
    f.append(m)
    F.append(M)
    Fdb.append(Mdb)
    Ff.append(Mf)
    
    ax.semilogx(Mf,Mdb)
    init_gain = eq_bands[i].gain
    axfreq.append(plt.axes([x_pos, y_pos, 0.03, 0.25], facecolor=axcolor))
    slider.append(Slider(axfreq[i], f'{i}', -100, 10, valinit=init_gain, valstep=0.1, orientation='vertical'))
    slider[i].on_changed(update_graph)
    x_pos += x_inc
    y_pos += y_inc

Y, Yf,Ydb = dsp.fft(y, fs, sig_len)    
ly.set_ydata(Ydb)

plt.show()

