import torch as t

# both arguments 2D
a = t.tensor([[1., 2],
              [3, 4]], requires_grad=True)

b = t.tensor([[11., 12]], requires_grad=True)

sum = a + b

print(f"\n  a.shape {a.shape}    a: {a} \n")
print(f"\n  b.shape {a.shape}    b: {b} \n")

print("\nsum: \n", sum)

g = t.tensor([[6., 5],
              [4, 3]], requires_grad=False)

sum.backward(g)

print(f"\n  g.shape {g.shape}    g: {g} \n")

print(f"\n  a.grad.shape {a.grad.shape}    a.grad: {a.grad} \n")
print(f"\n  b.grad.shape {b.grad.shape}    b.grad: {b.grad} \n")
