from mgrad.engine import Value

a = Value(-5)
c = a * a
print(f"c = a + a  = {a} * {a}a  = {c}")

c.backward()
print(f"a.grad:  {a.grad}  ")
print("#########################################")
