import numpy as np
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import axes3d
fig = plt.figure()
ax = fig.add_subplot(111, projection='3d')

# Plane parameters
n_1=2
n_2=4
n_3=1

x_0=1
y_0=4
z_0=10

# Create a grid of x and y

x = np.arange(-5, 5, 0.1)
y = np.arange(-5, 5, 0.1)
xg, yg = np.meshgrid(x, y)

# Compute z = f(x,y)

z = -n_1/n_3*(xg-x_0)-n_2/n_3*(yg-y_0)+z_0

ax.plot_wireframe(xg, yg, z, rstride=10, cstride=10)
plt.xlabel('x')
plt.ylabel('y')
plt.title('Plane: z=f(x,y)')
plt.show()