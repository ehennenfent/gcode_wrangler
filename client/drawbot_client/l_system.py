import typing as t
from functools import partial

from .turtle_wrapper import Drawbot

SIZE = 10


class LSystem:
    @staticmethod
    def do_nothing() -> None:
        pass

    def __init__(
        self,
        alphabet: t.Dict[str, t.Callable[[], None]],
        rules: t.Dict[str, str],
        axiom: str,
        *,
        allow_missing_alphabet: bool = False,
    ) -> None:
        self.alphabet: t.Dict[str, t.Callable[[], None]] = alphabet
        self.rules: t.Dict[str, str] = rules
        self.axiom: str = axiom

        def _assert_op_present(op: str) -> None:
            assert (
                op in self.alphabet
            ), f"No operation {op} found in alphabet! (Pass allow_missing_alphabet=True to silence this)"

        if not allow_missing_alphabet:
            for op in axiom:
                _assert_op_present(op)
            for op in rules:
                _assert_op_present(op)
            for rule in rules.values():
                for op in rule:
                    _assert_op_present(op)

    def evaluate(self, order: int = 0, axiom: t.Optional[str] = None) -> None:
        axiom = axiom if axiom is not None else self.axiom

        if order == 0:
            # Perform operations at final depth
            for op in axiom:
                self.alphabet.get(op, LSystem.do_nothing)()  # Defaults to no-op if undefined
        else:
            for op in axiom:
                # Recurse, replacing symbols if there's a rule for them
                self.evaluate(order - 1, axiom=self.rules.get(op, op))

    @staticmethod
    def standard_alphabet(drawbot: Drawbot, size: float, angle: float) -> t.Dict[str, t.Callable[[], None]]:
        return {
            "F": partial(drawbot.forward, size),
            "f": partial(drawbot.jog, size),
            "-": partial(drawbot.right, angle),
            "+": partial(drawbot.left, angle),
            "[": drawbot.push_state,
            "]": drawbot.pop_state,
            "(": drawbot.pendown,
            ")": drawbot.penup,
        }


class Seripinski(LSystem):
    def __init__(
        self,
        forward: t.Callable[[float], None],
        right: t.Callable[[float], None],
        left: t.Callable[[float], None],
        size: float = SIZE,
    ) -> None:
        super().__init__(
            alphabet={
                "F": partial(forward, size),
                "G": partial(forward, size),
                "-": partial(right, 120),
                "+": partial(left, 120),
            },
            rules={
                "F": "F-G+F+G-F",
                "G": "GG",
            },
            axiom="F-G-G",
        )


class SeripinskiArrowHead(LSystem):
    def __init__(
        self,
        forward: t.Callable[[float], None],
        right: t.Callable[[float], None],
        left: t.Callable[[float], None],
        size: float = SIZE,
    ) -> None:
        super().__init__(
            alphabet={
                "A": partial(forward, size),
                "B": partial(forward, size),
                "-": partial(right, 60),
                "+": partial(left, 60),
            },
            rules={
                "A": "B-A-B",
                "B": "A+B+A",
            },
            axiom="A",
        )


class Gosper(LSystem):
    def __init__(
        self,
        forward: t.Callable[[float], None],
        right: t.Callable[[float], None],
        left: t.Callable[[float], None],
        size: float = SIZE,
    ) -> None:
        super().__init__(
            alphabet={
                "A": partial(forward, size),
                "B": partial(forward, size),
                "-": partial(right, 60),
                "+": partial(left, 60),
            },
            rules={
                "A": "A-B--B+A++AA+B-",
                "B": "+A-BB--B-A++A+B",
            },
            axiom="A",
        )


class Dragon(LSystem):
    def __init__(
        self,
        forward: t.Callable[[float], None],
        right: t.Callable[[float], None],
        left: t.Callable[[float], None],
        size: float = SIZE,
    ) -> None:
        super().__init__(
            alphabet={
                "F": partial(forward, size),
                "G": partial(forward, size),
                "-": partial(right, 90),
                "+": partial(left, 90),
            },
            rules={
                "F": "F+G",
                "G": "F-G",
            },
            axiom="F",
        )


if __name__ == "__main__":
    turtle = Drawbot()

    # seripinski = Seripinski(turtle.forward, turtle.right, turtle.left, SIZE)
    # gosper = Gosper(turtle.forward, turtle.right, turtle.left, SIZE)
    # seripinski_arrowhead = SeripinskiArrowHead(turtle.forward, turtle.right, turtle.left, SIZE)
    # dragon = Dragon(turtle.forward, turtle.right, turtle.left, SIZE)

    fern = LSystem(
        alphabet={
            "X": LSystem.do_nothing,
            "F": partial(turtle.forward, SIZE),
            "-": partial(turtle.right, 25),
            "+": partial(turtle.left, 25),
            "[": turtle.push_state,
            "]": turtle.pop_state,
        },
        rules={
            "X": "F+[[X]-X]-F[-FX]+X",
            "F": "FF",
        },
        axiom="++X",
    )

    fern.evaluate(6)
