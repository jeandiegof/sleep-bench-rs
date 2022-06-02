import pandas as pd
import matplotlib.pyplot as plt

df = pd.read_csv('output.csv')
print(df)
x = df['d']
y = df['cpu_time_us']

plt.plot(x, y)
plt.show()