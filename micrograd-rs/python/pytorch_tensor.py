import torch as t

# both arguments 2D
mat_1 = t.tensor([[1., 2, 3],
                  [4, 3, 8],
                  [1, 7, 2]], requires_grad=True )

mat_2 = t.tensor([[2., 4],
                  [1, 3],
                  [2, 6]], requires_grad=True)

prod = t.matmul(mat_1, mat_2)
print("\n3x2 dimensional tensors :\n", prod)

out = prod.mean()
print("\nscalar: \n", out)

out.backward()

print("\nout.grad: \n", out.grad)
print("\nprod.grad: \n", prod.grad)

print("\nout: \n", out)
print("\nprod: \n", prod)

print("\nmat_1.grad: \n", mat_1.grad)
print("\nmat_2.grad: \n", mat_2.grad)

