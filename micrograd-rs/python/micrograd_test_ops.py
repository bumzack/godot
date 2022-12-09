from mgrad.engine import Value

a = Value(-5)
b = a.relu()
a.backward()
b.grad = 3
print(f"a  {a},  relu(a) {b}")

a = Value(0)
b = a.relu()
b.grad = 2
a.backward()
print(f"a  {a},  relu(a) {b}")

a = Value(23)
b = a.relu()
b.grad = 4
a.backward()
print(f"a  {a},  relu(a) {b}")

print("#########################################")

a = Value(23)
b = Value(45)
c = a + b
print(f"c = a + b = {a} + {b}  = {c}")

c.backward()
print(f"a.grad:  {a.grad}  ")
print(f"b.grad:  {b.grad}  ")
print(f"c.grad:  {c.grad}  ")
print("#########################################")

print("#########################################")

a = Value(23)
b = Value(45)
c = a - b
print(f"c = a - b = {a} - {b}  = {c}")

c.backward()
print(f"a.grad:  {a.grad}  ")
print(f"b.grad:  {b.grad}  ")
print(f"c.grad:  {c.grad}  ")
print("#########################################")

a = Value(12)
b = Value(23)
c = a * b
print(f"c = a * b = {a} *  {b}  = {c}")

c.backward()
print(f"a.grad:  {a.grad}  ")
print(f"b.grad:  {b.grad}  ")
print(f"c.grad:  {c.grad}  ")
print("#########################################")

a = Value(7)
b = Value(2)
c = a / b
print(f"c = a / b = {a}  / {b}  = {c}")

c.backward()
print(f"a.grad:  {a.grad}  ")
print(f"b.grad:  {b.grad}  ")
print(f"c.grad:  {c.grad}  ")
print("#########################################")

a = Value(7)
c = a.relu()
print(f"c = a.relu = {a}.relu() = {c}")

c.backward()
print(f"a.grad:  {a.grad}  ")
print(f"c.grad:  {c.grad}  ")
print("#########################################")

a = Value(0)
c = a.relu()
print(f"c = a.relu = {a}.relu() = {c}")

c.backward()
print(f"a.grad:  {a.grad}  ")
print(f"c.grad:  {c.grad}  ")
print("#########################################")

a = Value(-1)
c = a.relu()
print(f"c = a.relu = {a}.relu() = {c}")

c.backward()
print(f"a.grad:  {a.grad}  ")
print(f"c.grad:  {c.grad}  ")
print("#########################################")

a = Value(-1)
b = 2
c = a ** b
print(f"c = a ** b = {a} ** {b}  = {c}")

c.backward()
print(f"a.grad:  {a.grad}  ")
print(f"c.grad:  {c.grad}  ")
print("#########################################")

a = Value(3)
b = 3
c = a ** b
print(f"c = a ** b = {a} ** {b}  = {c}")

c.backward()
print(f"a.grad:  {a.grad}  ")
print(f"c.grad:  {c.grad}  ")
print("#########################################")

a = Value(3)
b = 1.5
c = a ** b
print(f"c = a ** b = {a} ** {b}  = {c}")

c.backward()
print(f"a.grad:  {a.grad}  ")
print(f"c.grad:  {c.grad}  ")
print("#########################################")

# def tanh(self):
#     x = self.data
#     t = (math.exp(2 * x) - 1) / (math.exp(2 * x) + 1)
#     out = Value(t, (self,), 'tanh')
#
#     def _backward():
#         self.grad += (1 - t ** 2) * out.grad
#
#     out._backward = _backward
#
#     return out


a = Value(2)
c = a.tanh()
print(f"c = a.tanh() = {a}.tanh()  = {c}")

c.backward()
print(f"a.grad:  {a.grad}  ")
print(f"c.grad:  {c.grad}  ")
print("#########################################")

a = Value(-1)
c = a.tanh()
print(f"c = a.tanh() = {a}.tanh()  = {c}")

c.backward()
print(f"a.grad:  {a.grad}  ")
print(f"c.grad:  {c.grad}  ")
print("#########################################")

a = Value(0)
c = a.tanh()
print(f"c = a.tanh() = {a}.tanh()  = {c}")

c.backward()
print(f"a.grad:  {a.grad}  ")
print(f"c.grad:  {c.grad}  ")
print("#########################################")


a = Value(-5)
c = a*a
print(f"c = a * a  = {a} * {a} = {c}")

c.backward()
print(f"a.grad:  {a.grad}  ")
print("#########################################")



a = Value(-5)
c = -a
print(f"c = -a  = -{a}  = {c}")

c.backward()
print(f"a.grad:  {a.grad}  ")
print("#########################################")




a = Value(-4)
b = Value(2)

c = (b-a).relu()
print(f"c = -(b-a).relu() = ({b} - {a}).relu() =  {c}")

c.backward()
print(f"a.grad:  {a.grad}  a.data:  {a.data} ")
print(f"b.grad:  {b.grad}  b.data:  {b.data} ")
print(f"c.grad:  {c.grad}  c.data:  {c.data} ")



topo = c.topoo

print("######## TOPO  #########################################")
for t in topo:
    print(f"{t.label} data {t.data }   grad = {t.grad:.8f}")
print("######## END TOPO  #########################################")



print("#########################################")



