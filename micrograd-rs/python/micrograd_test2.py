from graphviz import Digraph
from mgrad.engine import Value


def trace(root):
    nodes, edges = set(), set()

    def build(v):
        if v not in nodes:
            nodes.add(v)
            for child in v._prev:
                edges.add((child, v))
                build(child)

    build(root)
    return nodes, edges


def draw_dot(root, format='svg', rankdir='LR'):
    """
    format: png | svg | ...
    rankdir: TB (top to bottom graph) | LR (left to right)
    """
    assert rankdir in ['LR', 'TB']
    nodes, edges = trace(root)
    dot = Digraph(format=format, graph_attr={'rankdir': rankdir})  # , node_attr={'rankdir': 'TB'})

    for n in nodes:
        dot.node(name=str(id(n)), label="{ %s | data %.4f | grad %.4f }" % (n.label, n.data, n.grad), shape='record')
        if n._op:
            dot.node(name=str(id(n)) + n._op, label=n._op)
            dot.edge(str(id(n)) + n._op, str(id(n)))

    for n1, n2 in edges:
        dot.edge(str(id(n1)), str(id(n2)) + n2._op)

    return dot


a = Value(-4.0)
a.label = "a"
b = Value(2.0)
b.label = "b"

c = a + b
c.label = "c"

dot = draw_dot(c)
dot
dot.render('gout_c1')

c += c + 1

dot = draw_dot(c)
dot
dot.render('gout_c2')

c += 1 + c + (-a)

dot = draw_dot(c)
dot
dot.render('gout_c3')

c.backward()

print(f'{c.data:.8f}')

print(f'a.data = {a.data:.8f}')
print(f'b.data = {b.data:.8f}')
print(f'c.data = {c.data:.8f}')

print(f'a.grad = {a.grad:.8f}')
print(f'b.grad = {b.grad:.8f}')
print(f'c.grad = {c.grad:.8f}')

dot = draw_dot(c)
dot
dot.render('gout_with_grads')



topo = c.topoo

print("#################################################")
for t in topo:
    print(f"{t.label} data {t.data }   grad = {t.grad:.8f}")
print("#################################################")
