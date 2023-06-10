from turtle import Screen, RawTurtle, Vec2D
from dataclasses import dataclass
import time

X_WIDTH = 400
Y_HEIGHT = 400


@dataclass
class MoveRecord:
    dest: Vec2D
    pen_down: bool


@dataclass
class SleepRecord:
    duration: float


def _init_screen():
    screen = Screen()
    screen.setworldcoordinates(0, 0, X_WIDTH, Y_HEIGHT)
    screen.screensize(X_WIDTH * 2, Y_HEIGHT * 2)
    screen.title("Drawbot Planner")

    return screen


class Turtle:
    def __init__(self):
        self.screen = _init_screen()
        self.turtle = RawTurtle(canvas=self.screen, undobuffersize=0)
        self.turtle.speed(0)

        self._pendown = self.turtle.isdown()
        self._movements = [MoveRecord((0, 0), False)]

    # Motion
    def forward(self, distance):
        self.turtle.forward(distance)
        self._movements.append(MoveRecord(self.position(), self.isdown()))

    def backward(self, distance):
        self.turtle.backward(distance)
        self._movements.append(MoveRecord(self.position(), self.isdown()))

    def goto(self, *args):
        self.turtle.goto(*args)
        self._movements.append(MoveRecord(self.position(), self.isdown()))

    def setx(self, x):
        return self.goto(x, self.ycor())

    def sety(self, y):
        return self.goto(self.xcor(), y)

    def home(self, *args):
        self.goto(0, 0)
        self.setheading(0)

    # Angle manipulation
    def right(self, angle):
        self.turtle.right(angle)

    def left(self, angle):
        self.turtle.left(angle)

    def degrees(self, *args):
        self.turtle.degrees(*args)

    def radians(self, *args):
        self.turtle.radians(*args)

    def setheading(self, *args):
        self.turtle.setheading(*args)

    # Pen Controls
    def penup(self, *args):
        self._pendown = False
        self.turtle.pencolor("#FFFFBB")

    def pendown(self, *args):
        self._penup = True
        self.turtle.pencolor("#000000")

    def isdown(self, *args):
        return self._pendown

    # Read turtle state
    def position(self):
        return self.turtle.position()

    def towards(self, *args):
        return self.turtle.towards(*args)

    def xcor(self):
        return self.turtle.xcor()

    def ycor(self):
        return self.turtle.ycor()

    def heading(self):
        return self.turtle.heading()

    def distance(self, *args):
        return self.turtle.distance(*args)

    # Screen controls
    def bgpic(self, *args):
        self.screen.bgpic(*args)

    # Supplemental
    def wait(self, time):
        self._movements.append(SleepRecord(time))
        time.sleep(time)
