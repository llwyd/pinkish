import numpy as np
import matplotlib.pyplot as plt
from matplotlib.widgets import Slider, Button, RadioButtons
from scipy import signal
from scipy.io.wavfile import write
import dsp
import q_arithmetic as qmath
from tqdm import tqdm
import random as r

class CrossoverFilter():
    def set_fs(self,new_fs):
        self.fs = new_fs
        
    def set_cutoff(self,new_cutoff):
        self.cutoff = new_cutoff
    def regenerate(self):
        self.lpf = signal.butter(self.order,self.cutoff,'lowpass',fs=self.fs,output='sos')
        self.hpf = signal.butter(self.order,self.cutoff,'highpass',fs=self.fs,output='sos')

    def __init__(self,cutoff, fs, order):
        self.cutoff = cutoff
        self.order = order
        self.fs = fs
        
        self.lpf = signal.butter(self.order,self.cutoff,'lowpass',fs=self.fs,output='sos')
        self.hpf = signal.butter(self.order,self.cutoff,'highpass',fs=self.fs,output='sos')
       
def calculate_bands(filters, start,fs):
    step = (np.log(fs/2) - np.log(start))/ (filters + 1)
    cutoff = np.zeros(filters)
    cutoff[0] = np.exp(step) * start

    for i in range(1,filters):
        cutoff[i] = np.exp(step) * cutoff[i-1]

    return cutoff

num_bands = 6
#gain = np.zeros(num_bands)
gain = np.array(dsp.db_gain([1.00000000,0.28183829,0.16032454,0.07852356,0.04841724,0.03427678]))
fs = 48000
sig_len = fs 
order = 1
axcolor = 'lightgoldenrodyellow'
fig, ax = plt.subplots(figsize=(9.6,8))

plt.title("Pinkish Filter Design Tool")
plt.subplots_adjust(bottom=0.35)
plt.vlines(20,-100,10)
plt.xlim(1,fs/2)
plt.ylim(-30,5)
ax.grid(which='both')
plt.xlabel('Frequency (Hz)')
plt.ylabel('Magnitude (dB)')


def export_filters(event):

    ex_gains = np.zeros(num_bands)
    for i,eq in enumerate(eq_bands):
        ex_gains[i] = np.power(10, gain[i] / 20)
    
    white_gains = np.array2string(ex_gains,separator=',',floatmode='fixed')
    pink_gains = np.array2string(ex_gains,separator=',',floatmode='fixed')

    
    f = open (f'../src/filter_coeffs_{fs}.rs','w',encoding="utf-8")

    pink_string = f"pub const PINK_GAIN: [f32;6] = {pink_gains};"
    
    print(pink_string)

    f.write(pink_string)
    f.write("\n")

    for i,eq in enumerate(lr_filters):
        lpf_string = np.array2string(eq.lpf,separator=',',floatmode='fixed')
        hpf_string = np.array2string(eq.hpf,separator=',',floatmode='fixed')
        num_lpf_biquads, num_lpf_coeffs = eq.lpf.shape
        lpf_string = f"pub const FILTER_{i}_LPF: [[f32;{num_lpf_coeffs}];{num_lpf_biquads}] = {lpf_string};"
        num_hpf_biquads, num_hpf_coeffs = eq.hpf.shape
        hpf_string = f"pub const FILTER_{i}_HPF: [[f32;{num_hpf_coeffs}];{num_hpf_biquads}] = {hpf_string};"
        print(lpf_string)
        print(hpf_string)
        f.write(lpf_string)
        f.write(hpf_string)
        f.write("\n")
    
    f.close()

ideal_db, ideal_f = dsp.generate_decade_line( 13, 100000 )
ax.semilogx(ideal_f, ideal_db )


axradio = fig.add_axes([0.75, 0.1, 0.13, 0.175])
radiobutton = RadioButtons(axradio,('8000','16000','32000','44100','48000','96000'),active=4)

exbutton = fig.add_axes([0.60, 0.1, 0.1, 0.075])
exportbutton = Button(exbutton,'Export')
exportbutton.on_clicked(export_filters)


num_crossovers = num_bands - 1
lr_filters = []
lr_freqs = calculate_bands(num_crossovers,10,fs)
for i in range(0, num_crossovers):
    lr_filters.append(CrossoverFilter(lr_freqs[i],fs,order))

freqs = dsp.calculate_bands(num_bands,10,fs)
eq_bands = []
init_gain = 0.0
for i in range(0,num_bands):
    eq_bands.append(None)

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
y_pos = 0.04
y_inc = 0.0

def update():
    z = np.zeros(sig_len)
    high = signal.unit_impulse(sig_len)
#    for i in range(sig_len):
#        high[i] = (r.random() * 2.0) - 1
    for i in range(0, num_crossovers):
        low = signal.sosfilt(lr_filters[i].lpf, high)
        raw_gain = np.power(10, gain[i] / 20)
        z += (low * raw_gain)
        high = signal.sosfilt(lr_filters[i].hpf, high)

    raw_gain = np.power(10, gain[-1] / 20)
    z += (high * raw_gain)

    return z

def update_graph(val):
    for i in range(0, num_bands):
        gain[i] = slider[i].val

    y = update()
    print(f'max y: {np.max(np.abs(y))}')
    Y,Yf,Ydb = dsp.fft(y,fs,sig_len)
    
    ly.set_ydata(Ydb)
    hearing_range = [20, 20000]
    l_hr = int((hearing_range[0] / (fs / 2)) * len(Yf))
    u_hr = int((hearing_range[1] / (fs / 2)) * len(Yf))

    hr_slope = dsp.get_fslope( Yf[l_hr:u_hr], Ydb[l_hr:u_hr] )
    hearing_range_text.set_text(f'HR gradient: {hr_slope:.6f}')

def switch_fs(event):
    new_fs = int(event) 
    global fs
    fs = new_fs
    new_freqs = calculate_bands(num_crossovers,10,new_fs)
    print(f'{new_freqs}')
    for i,eq in enumerate(lr_filters):
        eq.set_cutoff(new_freqs[i])
        eq.set_fs(new_fs)
        eq.regenerate()
    for i, sl in enumerate(slider):
        sl.set_val(gain[i])
    #for i,g in enumerate(gain):
    #    slider[i].set_val(0.0)
    y = update()
    Y,Yf,Ydb = dsp.fft(y,fs,sig_len)
    
    ly.set_ydata(Ydb)
    ly.set_xdata(Yf)
    hearing_range = [20, 20000]
    l_hr = int((hearing_range[0] / (fs / 2)) * len(Yf))
    u_hr = int((hearing_range[1] / (fs / 2)) * len(Yf))

    hr_slope = dsp.get_fslope( Yf[l_hr:u_hr], Ydb[l_hr:u_hr] )
    hearing_range_text.set_text(f'HR gradient: {hr_slope:.6f}')

lr_sum = np.zeros(sig_len);

high = signal.unit_impulse(sig_len)

for i in range(0, num_crossovers):
    low = signal.sosfilt(lr_filters[i].lpf, high)
    lr_sum += low
    high = signal.sosfilt(lr_filters[i].hpf, high)
    
    M, Mf,Mdb = dsp.fft(low, fs, sig_len)    
    f.append(low)
    F.append(M)
    Fdb.append(Mdb)
    Ff.append(Mf)
    ax.semilogx(Mf,Mdb)
    
    M, Mf,Mdb = dsp.fft(high, fs, sig_len)    

f.append(high)
F.append(M)
Fdb.append(Mdb)
Ff.append(Mf)
ax.semilogx(Mf,Mdb)
lr_sum += high


y = lr_sum
for i in range(0, num_bands):
    axfreq.append(plt.axes([x_pos, y_pos, 0.03, 0.21], facecolor=axcolor))
    slider.append(Slider(axfreq[i], f'{i}', -100, 10, valinit=gain[i], valstep=0.1, orientation='vertical'))
    slider[i].on_changed(update_graph)
    x_pos += x_inc
    y_pos += y_inc

radiobutton.on_clicked(switch_fs)
Y, Yf,Ydb = dsp.fft(y, fs, sig_len)    
ly.set_ydata(Ydb)

hearing_range = [20, 20000]
l_hr = int((hearing_range[0] / (fs / 2)) * len(Yf))
u_hr = int((hearing_range[1] / (fs / 2)) * len(Yf))

hr_slope = dsp.get_fslope( Yf[l_hr:u_hr], Ydb[l_hr:u_hr] )
ideal_slope = dsp.get_fslope( ideal_f, ideal_db )

ideal_slope_text =ax.text(100,10,f'Ideal gradient: {ideal_slope:.6f}')
hearing_range_text =ax.text(100,8,f'20-20kHz gradient: {hr_slope:.6f}')

plt.show()

