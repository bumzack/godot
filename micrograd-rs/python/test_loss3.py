from micrograd.engine import Value

yb = [ -4.7, -5.9, -11.5]
scores = [Value(2.4), Value(4.6), Value(-9.2)]

parameters = [Value(1.3), Value(2.3), Value(4.5)]

# forward the model to get scores
# print(f"scores[0:20] {scores[0:20]}")


print(f"scores {scores}")
print(f"yb {yb}")


# svm "max-margin" loss
losses = [(1 + -yi * scorei).relu() for yi, scorei in zip(yb, scores)]

print(f"losses {losses}")

data_loss = sum(losses) * (1.0 / len(losses))
# L2 regularization
alpha = 1e-4
reg_loss = alpha * sum((p * p for p in parameters))
total_loss = data_loss + reg_loss
print(f"reg_loss {reg_loss.data}  data_loss {data_loss.data}   total_loss     {total_loss.data}")


accuracy_bool_array = [(yi > 0) == (scorei.data > 0) for yi, scorei in zip(yb, scores)]
print(f"accuracy_bool_array {accuracy_bool_array} ")
accuracy= sum(accuracy_bool_array) / len(accuracy_bool_array)
print(f"accuracy {accuracy} ")
