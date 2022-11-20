import torch as t

# both arguments 2D
a = t.tensor([1., 2, 3, 4], requires_grad=True)

sum = a.sum()

print("\nsum :\n", sum)

sum.backward()

print("\n a.grad :\n", a.grad)
