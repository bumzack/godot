import numpy as np
import py
a = np.array((1, 2, 3, 4,5,6)).reshape(2,3);
a_t = a.T
print(f" a  shape {a.shape} (2x3) \n  {a}")

print(f" a_t  shape {a_t.shape} (3x2) \n  {a_t}")


print(f" a([0, 0])   {a[0, 0]} ")
print(f" a([0, 1])   {a[0, 1]} ")
print(f" a([0, 2])   {a[0, 2]} ")

print(f" a([1, 0])   {a[1, 0]} ")
print(f" a([1, 1])   {a[1, 1]} ")
print(f" a([1, 2])   {a[1, 2]} ")



print(f" a_t([0, 0])   {a_t[0, 0]} ")
print(f" a_t([0, 1])   {a_t[0, 1]} ")


a = np.array((1, 2, 3, 4,5,6)).reshape(2,3)
b = np.array((11.0, 12.0, 13.0, 14.0, 15.0, 16.0)).reshape(3, 2)

c = np.matmul(a, b)

print(f" c  {c.shape}  \n   {c} ")







