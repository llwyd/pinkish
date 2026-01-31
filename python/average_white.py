import numpy as np
import matplotlib.pyplot as plt
import dsp

def generate_white_line(start_mag, end_freq):
    iterations = int(np.log10( end_freq ) )
    iterations += 1

    mags = np.zeros(iterations)
    freqs = np.zeros(iterations)

    for i in range( iterations ):
        mags[i] = start_mag
        freqs[i] = ( 10 ** i )

    return mags, freqs

fs = 48000
num_samples = fs
x = np.loadtxt('../white.txt')

split_num = np.int32(len(x) / fs)

print(f'num tests: {split_num}')
x = dsp.norm(x)

y = np.split(x,split_num)

Ydb = np.zeros(int(num_samples/2))
for z in y:
    Z, Zf, Zdb = dsp.fft(z, fs, len(z),norm='ortho' )
    Ydb = np.add(Ydb,Zdb)

Avgdb = Ydb/split_num


ideal_db, ideal_f = generate_white_line( -7.32, 100000 )

plt.semilogx(Zf,Avgdb)
plt.semilogx(ideal_f, ideal_db)
plt.title("Verify EQ White noise config")
plt.xlabel("Frequency (Hz)");
plt.ylabel("Magnitude (dB)");
plt.legend(["Average of 1000 runs","ideal white"])
plt.xlim(1,24000)
plt.ylim(-25,10)
plt.show()

