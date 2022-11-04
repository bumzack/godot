import random

import numpy as np
from micrograd.engine import Value
from micrograd.nn import Neuron, Layer,MLP

np.random.seed(1337)
random.seed(1337)

yb = [4.7, 5.9, 11.5]
scores = [Value(-2.4), Value(-4.6), Value(9.2)]

parameters = [Value(1.3), Value(2.3), Value(4.5)]

# forward the model to get scores
# print(f"scores[0:20] {scores[0:20]}")

n = Neuron(2)
xinp = [2.0, 3.0]
y_neuron = n(xinp)
print(f"Neuron.parameters {n.parameters()}")

np.random.seed(1337)
random.seed(1337)

l = Layer(2, 1)
y_layer = l(xinp)
print(f"Layer.parameters {n.parameters()}")


np.random.seed(1337)
random.seed(1337)

mlp = MLP(2, [1])
y_mlp = mlp(xinp)
print(f"MLP.parameters {mlp.parameters()}")



print(f"xinp {xinp}")
print(f"Neuron {y_neuron}")
print(f"Layer {y_layer}")
print(f"Network {y_mlp}")

print("####################################################")
print("backward pass")
print("####################################################")
y_neuron.backward()
print(f"Neuron {y_neuron}")
print(f"Neuron.parameters {n.parameters()}")
print("####################################################")

y_layer.backward()
print(f"Layer {y_layer}")
print(f"Layer.parameters {l.parameters()}")
print("####################################################")

y_mlp.backward()
print(f"NN {y_mlp}")
print(f"NN.parameters {mlp.parameters()}")
print("####################################################")



#
# # svm "max-margin" loss
# losses = [(1 + -yi * scorei).relu() for yi, scorei in zip(yb, scores)]
#
# print(f"losses {losses}")
#
# data_loss = sum(losses) * (1.0 / len(losses))
# # L2 regularization
# alpha = 1e-4
# reg_loss = alpha * sum((p * p for p in parameters))
# total_loss = data_loss + reg_loss
# print(f"reg_loss {reg_loss.data}  data_loss {data_loss.data}   total_loss     {total_loss.data}")
