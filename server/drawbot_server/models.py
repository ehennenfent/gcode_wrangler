import typing as t
from enum import Enum, auto
from time import sleep

from pydantic import BaseModel

from .client import GcodeClient, Handle


class Status(Enum):
    RECEIVED = auto()  # Before preprocessing has completed
    READY = auto()  # Preprocessing done, ready for execution
    RUNNING = auto()  # Triggered by user
    PAUSED = auto()  # Paused by user
    REJECTED = auto()  # Validtion failed; rejected


class Vec2D(BaseModel):
    x: float
    y: float


class Movement(BaseModel):
    dest: Vec2D
    pen_down: bool


class PostedJob(BaseModel):
    username: str = ""
    comment: str = ""
    movements: t.List[Movement]


class GcodeJob(PostedJob):
    image_path: str = "https://fakeimg.pl/800x600"
    progress: t.Optional[int] = None
    status: Status = Status.RECEIVED
    handle: t.Optional[Handle] = None

    def run(self) -> None:
        client: GcodeClient = GcodeClient()

        # State machine
        assert self.status == Status.READY
        self.status = Status.RUNNING

        assert self.handle is not None

        client.start_run(self.handle)

        # Monitor progress
        self.progress = 0
        for _ in range(100):
            # self.progress = client.get_run(self.handle)
            self.progress += 1
            sleep(1)

        # Reset upon completion
        self.progress = None
        self.status = Status.READY

    @classmethod
    def from_posted(cls, posted: PostedJob) -> "GcodeJob":
        return cls(comment=posted.comment, username=posted.username, movements=posted.movements)

    def analyze(self) -> None:
        self.handle = GcodeClient().upload(self.movements)
        self.status = Status.READY
