import numpy as np
import matplotlib.pyplot as plt
import dsp

x = np.loadtxt('../pink.txt')
x = dsp.norm(x)
X, Xf, Xdb = dsp.fft(x, 48000, len(x),norm='ortho' )

ideal_db, ideal_f = dsp.generate_decade_line( -18, 100000 )

plt.semilogx(Xf,Xdb)
plt.semilogx(ideal_f, ideal_db)
plt.xlim(1,24000)
plt.show()

