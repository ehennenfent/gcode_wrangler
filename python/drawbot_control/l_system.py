import turtle
from random import randint
from functools import partial

SIZE = 10

turtle.speed(0)


class LSystem:
    def __init__(self, alphabet, rules, axiom):
        self.alphabet = alphabet
        self.rules = rules
        self.axiom = axiom

    def evaluate(self, order=0, axiom=None):
        axiom = axiom if axiom is not None else self.axiom

        if order == 0:
            for op in axiom:
                self.alphabet[op]()
        else:
            for op in axiom:
                self.evaluate(order - 1, axiom=self.rules.get(op, op))


seripinski = LSystem(
    alphabet={
        "F": partial(turtle.forward, SIZE),
        "G": partial(turtle.forward, SIZE),
        "-": partial(turtle.right, 120),
        "+": partial(turtle.left, 120),
    },
    rules={
        "F": "F-G+F+G-F",
        "G": "GG",
    },
    axiom="F-G-G",
)

gosper = LSystem(
    alphabet={
        "A": partial(turtle.forward, SIZE),
        "B": partial(turtle.forward, SIZE),
        "-": partial(turtle.right, 60),
        "+": partial(turtle.left, 60),
    },
    rules={
        "A": "A-B--B+A++AA+B-",
        "B": "+A-BB--B-A++A+B",
    },
    axiom="A",
)

seripinski_arrowhead = LSystem(
    alphabet={
        "A": partial(turtle.forward, SIZE),
        "B": partial(turtle.forward, SIZE),
        "-": partial(turtle.right, 60),
        "+": partial(turtle.left, 60),
    },
    rules={
        "A": "B-A-B",
        "B": "A+B+A",
    },
    axiom="A",
)

dragon = LSystem(
    alphabet={
        "F": partial(turtle.forward, SIZE),
        "G": partial(turtle.forward, SIZE),
        "-": partial(turtle.right, 90),
        "+": partial(turtle.left, 90),
    },
    rules={
        "F": "F+G",
        "G": "F-G",
    },
    axiom="F",
)

saved = []


def push():
    saved.append((turtle.pos(), turtle.heading()))


def pop():
    turtle.penup()
    coords, heading = saved.pop()
    turtle.goto(coords)
    turtle.setheading(heading)
    turtle.pendown()


fern = LSystem(
    alphabet={
        "X": lambda: None,
        "F": partial(turtle.forward, SIZE),
        "-": partial(turtle.right, 25),
        "+": partial(turtle.left, 25),
        "[": push,
        "]": pop,
    },
    rules={
        "X": "F+[[X]-X]-F[-FX]+X",
        "F": "FF",
    },
    axiom="++X",
)

turtle.speed(0)
# seripinski.evaluate(4)
# seripinski_arrowhead.evaluate(4)
fern.evaluate(5)
input()
