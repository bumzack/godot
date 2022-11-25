import torch as t

a = t.tensor([1., 2], requires_grad=True)
b = t.tensor([1.], requires_grad=True)

f = a.add(b)
c = a + b

print(f"\n  a  shape {a.shape}  {a}")
print(f"\n  b  shape {b.shape}  {b}")

z = t.tensor([1.0, 1.0], requires_grad=True)


c.backward(z)

print("\n a.grad :\n", a.grad)
print("\n b.grad :\n", b.grad)
print("\n c.grad :\n", c.grad)
