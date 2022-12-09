import random
import time

import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
from micrograd.engine import Value
from micrograd.nn import MLP
from sklearn.datasets import make_moons

np.random.seed(1337)
random.seed(1337)

X, y = make_moons(n_samples=100, noise=0.1)

# for i in range(0,100):
#     print(f"(x {X[i,0]} /  {X[i,1]})    -->   {y[i]} ")


pd.DataFrame(X).to_csv("X.csv")
pd.DataFrame(y).to_csv("y.csv")

y = y * 2 - 1  # make y be -1 or 1



# visualize in 2D
plt.figure(figsize=(5, 5))
plt.scatter(X[:, 0], X[:, 1], c=y, s=20, cmap='jet')
# plt.show()

# initialize a model
model = MLP(2, [16, 16, 1])  # 2-layer neural network
print(model)
print("number of parameters", len(model.parameters()))


# loss function
def loss(batch_size=None):
    # inline DataLoader :)
    if batch_size is None:
        Xb, yb = X, y
    else:
        ri = np.random.permutation(X.shape[0])[:batch_size]
        Xb, yb = X[ri], y[ri]

    inputs = [list(map(Value, xrow)) for xrow in Xb]
    print(f"inputs.len   {len(inputs)}")

    # print(f"inputs {inputs}")

    # forward the model to get scores
    scores = list(map(model, inputs))

    # svm "max-margin" loss
    losses = [(1 + -yi * scorei).relu() for yi, scorei in zip(yb, scores)]

    data_loss = sum(losses) * (1.0 / len(losses))
    # L2 regularization
    alpha = 1e-4
    reg_loss = alpha * sum((p * p for p in model.parameters()))
    total_loss = data_loss + reg_loss
    print(f"reg_loss {reg_loss.data}  data_loss {data_loss.data}   total_loss     {total_loss.data}")

    # also get accuracy
    accuracy = [(yi > 0) == (scorei.data > 0) for yi, scorei in zip(yb, scores)]
    return total_loss, sum(accuracy) / len(accuracy)


total_loss, acc = loss()
print(f"before training {total_loss.data}     accuracy {acc * 100}% ")
start = time.time()

# optimization
for k in range(100):

    # forward
    total_loss, acc = loss()

    # backward
    model.zero_grad()
    total_loss.backward()

    # print("############################################################ ")
    # print("initial ")
    # print("############################################################ ")
    # for p in model.parameters():
    #     print(f"param.data  {p.data}   grad   {p.grad} ")
    # print("############################################################ ")

    # update (sgd)
    learning_rate = 1.0 - 0.9 * k / 100
    for p in model.parameters():
        p.data -= learning_rate * p.grad

    if k % 1 == 0:
        print(f"step {k} loss {total_loss.data}, accuracy {acc * 100}%  learning_rate {learning_rate} ")

    # if k > 1:
    #     break

# print("############################################################ ")
# print("updated ")
# print("############################################################ ")
#
# for p in model.parameters()[2]:
#     print(f"param.data  {p.data}   grad   {p.grad} ")
#
# print("############################################################ ")


# visualize decision boundary

end = time.time()
#
print(f"training took {end - start}")

h = 0.25
x_min, x_max = X[:, 0].min() - 1, X[:, 0].max() + 1
y_min, y_max = X[:, 1].min() - 1, X[:, 1].max() + 1
xx, yy = np.meshgrid(np.arange(x_min, x_max, h),
                     np.arange(y_min, y_max, h))
Xmesh = np.c_[xx.ravel(), yy.ravel()]
inputs = [list(map(Value, xrow)) for xrow in Xmesh]
scores = list(map(model, inputs))
Z = np.array([s.data > 0 for s in scores])
Z = Z.reshape(xx.shape)

fig = plt.figure()
plt.contourf(xx, yy, Z, cmap=plt.cm.Spectral, alpha=0.8)
plt.scatter(X[:, 0], X[:, 1], c=y, s=40, cmap=plt.cm.Spectral)
plt.xlim(xx.min(), xx.max())
plt.ylim(yy.min(), yy.max())
# plt.show()
