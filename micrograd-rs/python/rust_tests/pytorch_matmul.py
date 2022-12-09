import torch as t

X = t.tensor([[2., 1., -3], [-3, 4, 2]], requires_grad=True)
W = t.tensor([[3., 2., 1., -1], [2, 1, 3, 2], [3, 2, 1, -2]], requires_grad=True)

Y = t.matmul(X, W)
print(Y)

print(f" ################################ ")

print(X.shape, W.shape, Y.shape)
print(f" ################################ ")

dL_over_dy = t.tensor([[2, 3, -3, 9], [-8, 1, 4, 6]])
print(dL_over_dy, dL_over_dy.shape)

print(f" ################################ ")

Y.backward(dL_over_dy)
print(X.grad)
