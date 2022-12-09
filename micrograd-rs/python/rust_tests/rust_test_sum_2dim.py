import torch as t

# both arguments 2D
a = t.tensor([[1., 2], [3, 4]], requires_grad=True)

sum = a.sum()

z = 2 * sum
print("\n z  :\n", z)

sum.backward()

print("\n a.grad :\n", a.grad)
