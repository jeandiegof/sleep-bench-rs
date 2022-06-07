import pandas as pd
import matplotlib.pyplot as plt

df = pd.read_csv('output3.csv')

d = df['d']
cpu_time = df['cpu_time_us']
package_energy = df['package_energy']
core_energy = df['core_energy']

# title, axis and run with more points without other peripherals
fig1, ax1 = plt.subplots()
ax1.set_title('CPU active time [uS]')
ax1.plot(d, cpu_time)

fig2, (ax2, ax3) = plt.subplots(2);
ax2.set_title('Package energy [uJ]')
ax2.plot(d, package_energy);
ax3.set_title('Core energy [uJ]')
ax3.plot(d, core_energy);

plt.show()