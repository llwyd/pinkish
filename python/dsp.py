import numpy as np
import matplotlib.pyplot as plt
from scipy import signal
from scipy.io import wavfile
from scipy import stats

class LPF():
    def __init__( self, order, cutoff, raw_gain, fs, sig_len ):
        self.fs = fs
        self.cutoff = cutoff
        self.order = order
        self.gain = raw_gain
        self.sig_len = sig_len
        self.filter = signal.butter( self.order, self.cutoff, 'lp', fs = self.fs, output = 'sos' )

        dirac = signal.unit_impulse( self.sig_len )
        self.ir = signal.sosfilt( self.filter, dirac ) * gain( raw_gain )

        self.FFT, self.FFTf, self.FFTdb = fft( self.ir, self.fs, self.sig_len )

class SinglePoleLPF():
    def __init__( self, order, cutoff, raw_gain, fs, sig_len ):
        self.fs = fs
        self.cutoff = cutoff
        self.alpha = get_alpha( self.cutoff, self.fs )
        self.order = order
        self.gain = raw_gain
        self.sig_len = sig_len

        self.ir = ewma( self.alpha, self.sig_len) * gain( self.gain )
        self.FFT, self.FFTf, self.FFTdb = fft( self.ir, self.fs, self.sig_len )

    def update_freq( self, cutoff ):
        self.cutoff = cutoff
        self.alpha = get_alpha( self.cutoff, self.fs )
        
        self.ir = ewma( self.alpha, self.sig_len) * gain( self.gain )
        self.FFT, self.FFTf, self.FFTdb = fft( self.ir, self.fs, self.sig_len )

    def update_gain( self, raw_gain ):
        self.gain = raw_gain
        self.ir = ewma( self.alpha, self.sig_len) * gain( self.gain )
        self.FFT, self.FFTf, self.FFTdb = fft( self.ir, self.fs, self.sig_len )

class SinglePoleHPF():
    def __init__( self, order, cutoff, raw_gain, fs, sig_len ):
        self.fs = fs
        self.cutoff = cutoff
        self.alpha = get_alpha( self.cutoff, self.fs )
        self.order = order
        self.gain = raw_gain
        self.sig_len = sig_len

        self.ir = self.kernel() * gain( self.gain )
        self.FFT, self.FFTf, self.FFTdb = fft( self.ir, self.fs, self.sig_len )

    def update_freq( self, cutoff ):
        self.cutoff = cutoff
        self.alpha = get_alpha( self.cutoff, self.fs )
        
        self.ir = self.kernel() * gain( self.gain )
        self.FFT, self.FFTf, self.FFTdb = fft( self.ir, self.fs, self.sig_len )

    def update_gain( self, raw_gain ):
        self.gain = raw_gain
        self.ir = self.kernel() * gain( self.gain )
        self.FFT, self.FFTf, self.FFTdb = fft( self.ir, self.fs, self.sig_len )
        
    def kernel(self):
        a_0 = (1 + self.alpha) / 2
        a_1 = -(1 + self.alpha) / 2
        b_1 = self.alpha
    
        dirac = signal.unit_impulse( self.sig_len )
        y = np.zeros( self.sig_len )
        for i in range( self.sig_len ):
            y[i] = (dirac[i] * a_0) + (dirac[i-1] * a_1) + (b_1 * y[i-1])
        return y

class EQBand():
    def __init__(self,lower_cutoff,upper_cutoff,fs,order):
        self.order = order
        self.fs = fs
        self.gain = 0.0
        if lower_cutoff == 0.0:
            self.filter = signal.butter(self.order,upper_cutoff,'lowpass',fs=fs,output='sos')
        elif upper_cutoff == fs / 2:
            self.filter = signal.butter(self.order,lower_cutoff,'highpass',fs=fs,output='sos')
        else:
            self.filter = signal.butter(self.order,[lower_cutoff, upper_cutoff],'bandpass',fs=fs,output='sos')

class EQButterBand():
    def gain_raw(self):
        return np.power(10, self.gain / 20)
    def __init__(self,lower_cutoff, upper_cutoff,fs, order, gain_db):
        self.lower_cutoff = lower_cutoff
        self.upper_cutoff = upper_cutoff
        self.order = order
        self.fs = fs
        self.gain = gain_db
        self.filter2 = None 
        if lower_cutoff == 0.0:
            self.filter = signal.butter(self.order,upper_cutoff,'lowpass',fs=fs,output='sos')
        elif int(np.round(upper_cutoff)) == int(int(fs)/2):
            self.filter = signal.butter(self.order,lower_cutoff,'highpass',fs=fs,output='sos')
        else:
            self.filter = signal.butter(self.order,upper_cutoff,'lowpass',fs=fs,output='sos')
            self.filter2 = signal.butter(self.order,lower_cutoff,'highpass',fs=fs,output='sos')
       
        self.sos = self.filter
        if self.filter2 is not None:
            self.sos = np.append(self.sos,self.filter2,axis=0)
        
        print(f'EQBand: {lower_cutoff} <-> {upper_cutoff} g: {self.gain} fs:{self.fs}')


def fft(x,fs,fft_len,norm=None):
    F = np.fft.fft(x,fft_len,norm=norm)
    F = np.abs(F)
    Ff = (fs/2)*np.linspace(0,1,int(fft_len/2))
    Fdb = 20*np.log10(F[:int(len(F)/2)]);
    
    return F, Ff, Fdb

def fft_norm(x,fs,fft_len):
    F = np.fft.fft(x,fft_len)
    F = np.abs(F)
    F = norm(F)
    Ff = (fs/2)*np.linspace(0,1,int(fft_len/2))
    Fdb = 20*np.log10(F[:int(len(F)/2)]);
    
    return F, Ff, Fdb

def norm( n ):
    return n/np.max(np.abs(n))

def gain( g ):
    return np.power(10, g / 20 )

def db_gain( g ):
    return -20*np.log10(g)

def get_alpha( fc, fs ):
    alpha = np.exp( -2 * np.pi * ( fc / fs ) )
    return alpha

def ewma( alpha, length ):
    dirac = signal.unit_impulse( length )
    y = np.zeros( length )
    for i in range( length ):
        y[i] = dirac[i] - alpha*(dirac[i] - y[i-1])
    return y

def generate_decade_line(start_mag, end_freq):
    iterations = int(np.log10( end_freq ) )
    iterations += 1

    mags = np.zeros(iterations)
    freqs = np.zeros(iterations)

    for i in range( iterations ):
        mags[i] = start_mag
        freqs[i] = ( 10 ** i )

        start_mag -= 10

    return mags, freqs

def get_fslope( Xf, Xdb ):
    slope, _, _, _, _ = stats.linregress( np.log10( Xf, where=Xf > 0 ), np.log10( gain( Xdb ) ) )
    return slope

def calculate_bands(bands, fs):
    step = (np.log(fs/2) - np.log(20)) / (bands)
    cutoff = np.zeros(bands)
    cutoff[0] = np.exp(step)*20

    for i in range(1,bands):
        cutoff[i] = np.exp(step) * cutoff[i-1]
    cutoff = np.pad(cutoff,(1,0),'constant',constant_values=0) 

    return cutoff

