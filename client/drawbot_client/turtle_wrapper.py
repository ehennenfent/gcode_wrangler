try:
    from turtle import Screen, RawTurtle, Vec2D, _Screen
except ModuleNotFoundError as e:
    print("=" * 64)
    print("Tk not found! You need to install a version of Python with Tk support")
    print("https://docs.python.org/3/library/tkinter.html")
    print("=" * 64, "\n" * 2)

    raise e

from dataclasses import dataclass
from time import sleep
import typing as t

X_WIDTH = 400
Y_HEIGHT = 400

AnonFunc: t.TypeAlias = t.Callable[[], None]


@dataclass(frozen=True)
class MoveRecord:
    dest: Vec2D
    pen_down: bool


@dataclass(frozen=True)
class SleepRecord:
    duration: float


@dataclass(frozen=True)
class TurtleState:
    position: Vec2D
    heading: float
    pen_down: bool


def _init_screen() -> _Screen:
    screen = Screen()
    screen.setworldcoordinates(0, 0, X_WIDTH, Y_HEIGHT)
    screen.screensize(X_WIDTH * 2, Y_HEIGHT * 2)
    screen.title("Drawbot Planner")

    return screen


class _TempPenState:
    def __init__(self, turtle: "Drawbot", pen_down: bool):
        self.turtle = turtle
        self.pen_down = pen_down
        self.old_pen_down = self.turtle.isdown()

    def __enter__(self) -> None:
        self.old_pen_down = self.turtle.isdown()
        self.turtle.set_pen_down(self.pen_down)

    def __exit__(self, *_args: t.Any) -> None:
        self.turtle.set_pen_down(self.old_pen_down)


class Drawbot:
    def __init__(self, *, hide_inactive_moves: bool = False, speed: int = 0) -> None:
        self._screen: _Screen = _init_screen()
        self._turtle: RawTurtle = RawTurtle(canvas=self._screen, undobuffersize=0)
        self._turtle.speed(speed)

        self.hide_inactive_moves: bool = hide_inactive_moves

        self._pendown: bool = self._turtle.isdown()
        self._movements: t.List[t.Union[MoveRecord, SleepRecord]] = [MoveRecord(self.position(), False)]
        self._state_stack: t.List[TurtleState] = []

    # Motion
    def forward(self, distance: float) -> None:
        self._turtle.forward(distance)
        self._movements.append(MoveRecord(self.position(), self.isdown()))

    fd = forward

    def backward(self, distance: float) -> None:
        self._turtle.backward(distance)
        self._movements.append(MoveRecord(self.position(), self.isdown()))

    bk = backward
    back = backward

    def goto(self, x: t.Union[float, Vec2D], y: t.Optional[float] = None) -> None:
        self._turtle.goto(x, y)
        self._movements.append(MoveRecord(self.position(), self.isdown()))

    setpos = goto
    setposition = goto

    def setx(self, x: float) -> None:
        return self.goto(x, self.ycor())

    def sety(self, y: float) -> None:
        return self.goto(self.xcor(), y)

    def home(self) -> None:
        self.goto(0, 0)
        self.setheading(0)

    # Angle manipulation
    def right(self, angle: float) -> None:
        self._turtle.right(angle)

    rt = right

    def left(self, angle: float) -> None:
        self._turtle.left(angle)

    lt = left

    def degrees(self, fullcircle: float = 360.0) -> None:
        self._turtle.degrees(fullcircle)

    def radians(self) -> None:
        self._turtle.radians()

    def setheading(self, to_angle: float) -> None:
        self._turtle.setheading(to_angle)

    seth = setheading

    # Pen Controls
    def penup(self) -> None:
        self._pendown = False
        self._turtle.pencolor("#FFFFBB")
        if self.hide_inactive_moves:
            self._turtle.penup()

    pu = penup
    up = penup

    def pendown(self) -> None:
        self._penup = True
        self._turtle.pencolor("#000000")
        if self.hide_inactive_moves:
            self._turtle.pendown()

    pd = pendown
    down = pendown

    def isdown(self) -> bool:
        return self._pendown

    def set_pen_down(self, new_pen_down: bool) -> None:
        if new_pen_down:
            self.pendown()
        else:
            self.penup()

    # Read turtle state
    def position(self) -> Vec2D:
        return self._turtle.position()

    pos = position

    def towards(self, x: t.Union[float, Vec2D], y: t.Optional[float] = None) -> float:
        return self._turtle.towards(x, y)

    def xcor(self) -> float:
        return self._turtle.xcor()

    def ycor(self) -> float:
        return self._turtle.ycor()

    def heading(self) -> float:
        return self._turtle.heading()

    def distance(self, x: t.Union[float, Vec2D], y: t.Optional[float] = None) -> float:
        return self._turtle.distance(x, y)

    # Screen controls
    def bgpic(self, new_pic: t.Optional[str] = None) -> t.Optional[str]:
        return self._screen.bgpic(new_pic)

    # Supplemental
    def wait(self, time: float) -> None:
        self._movements.append(SleepRecord(time))
        sleep(time)

    def push_state(self) -> None:
        self._state_stack.append(
            TurtleState(
                position=self.position(),
                heading=self.heading(),
                pen_down=self.isdown(),
            )
        )

    def pop_state(self, position: bool = True, heading: bool = True, pen_down: bool = False) -> None:
        last_state: TurtleState = self._state_stack.pop()
        if position:
            self.goto(last_state.position)
        if heading:
            self.setheading(last_state.heading)
        if pen_down:
            self.set_pen_down(last_state.pen_down)

    # Wrappers for forcing pen state during certain operations
    def TempPenState(self, pen_down: bool) -> _TempPenState:
        return _TempPenState(self, pen_down)

    def _wrap_with_pen_state(self, pen_down: bool, func: t.Callable[[t.Any], t.Any], *args: t.Any) -> t.Any:
        with self.TempPenState(pen_down):
            return func(*args)

    def jog(self, distance: float) -> None:
        self._wrap_with_pen_state(False, self.forward, distance)

    def teleport(self, x: t.Union[float, Vec2D], y: t.Optional[float] = None) -> None:
        self._wrap_with_pen_state(False, self.goto, x, y)
