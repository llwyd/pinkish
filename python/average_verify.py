import numpy as np
import matplotlib.pyplot as plt
import dsp

fs = 48000
num_samples = fs
x = np.loadtxt('../pink.txt')

split_num = np.int32(len(x) / fs)

print(f'num tests: {split_num}')
x = dsp.norm(x)

y = np.split(x,split_num)

Ydb = np.zeros(int(num_samples/2))
for z in y:
    Z, Zf, Zdb = dsp.fft(z, fs, len(z),norm='ortho' )
    Ydb = np.add(Ydb,Zdb)

Avgdb = Ydb/split_num


ideal_db, ideal_f = dsp.generate_decade_line( 19, 100000 )

plt.semilogx(Zf,Avgdb)
plt.semilogx(ideal_f, ideal_db)
plt.xlabel("Frequency (Hz)");
plt.ylabel("Magnitude (dB)");
plt.legend(["Average of 1000 runs", "Ideal 1/f"])
plt.xlim(1,24000)
plt.show()

