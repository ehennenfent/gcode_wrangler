import typing as t
from time import sleep

from pydantic import BaseModel


class Vec2D(BaseModel):
    x: float
    y: float


class Movement(BaseModel):
    dest: Vec2D
    pen_down: bool


class PostedJob(BaseModel):
    username: str
    comment: str
    ephemeral_key: str
    movements: t.List[Movement]


class GcodeJob(BaseModel):
    image_path: str = "https://fakeimg.pl/800x600"
    comment: str = ""
    user: str = ""
    progress: t.Optional[int] = None
    gcode: t.List[Movement]

    def start_background_task(self):
        self.progress = 0
        for _ in range(100):
            self.progress += 1
            sleep(1)
        self.progress = None

    @classmethod
    def from_posted(cls, posted: PostedJob):
        return cls(comment=posted.comment, user=posted.username, gcode=posted.movements)

    def analyze(self):
        pass
