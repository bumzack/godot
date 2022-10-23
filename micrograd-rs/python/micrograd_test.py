from graphviz import Digraph
from micrograd.engine import Value


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

d = a * b + b ** 3
d.label = "d"

c += c + 1

dot = draw_dot(c)
dot
dot.render('gout_c2')

c += 1 + c + (-a)

dot = draw_dot(c)
dot
dot.render('gout_c3')

d += d * 2 + (b + a).relu()

dot = draw_dot(d)
dot
dot.render('gout_d1')

d += 3 * d + (b - a).relu()

dot = draw_dot(d)
dot
dot.render('gout_d2')

e = c - d
e.label = "e"
dot = draw_dot(e)
dot
dot.render('gout_e')

f = e ** 2
f.label = "f"
dot = draw_dot(f)
dot
dot.render('gout_f')

g = f / 2.0
g.label = "g"
dot = draw_dot(g)
dot
dot.render('gout_g1')

g += 10.0 / f
dot = draw_dot(g)
dot
dot.render('gout_g2')


print(f'{g.data:.4f}')  # prints 24.7041, the outcome of this forward pass
g.backward()
print(f'a.grad = {a.grad:.4f}')  # prints 138.8338, i.e. the numerical value of dg/da
print(f'b.grad = {b.grad:.4f}')  # prints 645.5773, i.e. the numerical value of dg/db
print(f'c.grad = {c.grad:.4f}')
print(f'd.grad = {d.grad:.4f}')
print(f'e.grad = {e.grad:.4f}')
print(f'f.grad = {f.grad:.4f}')

dot = draw_dot(g)
dot
dot.render('gout_with_grads')
