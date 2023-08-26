import shutil
import typing as t
from enum import Enum, auto
from pathlib import Path
from time import sleep

from pydantic import BaseModel

from .client import GcodeClient, Handle

SERVER_DIR = Path(__file__).parent.parent


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

    def nested_dict(self):
        return {
            "dest": self.dest.dict(),
            "pen_down": self.pen_down
        }


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
        sleep(1)
        starting_progress = client.get_progress(self.handle)
        current_progress = starting_progress

        while current_progress > 0:
            current_progress = client.get_progress(self.handle)
            self.progress = int(((starting_progress - current_progress) / starting_progress) * 100)
            sleep(1)

        # Reset upon completion
        self.progress = None
        self.status = Status.READY

    @classmethod
    def from_posted(cls, posted: PostedJob) -> "GcodeJob":
        return cls(comment=posted.comment, username=posted.username, movements=posted.movements)

    def analyze(self) -> None:
        client = GcodeClient()
        self.handle = client.upload(self.movements)

        with open(SERVER_DIR.joinpath("rendered").joinpath(f"{self.handle}.png"), "wb") as pngfile:
            shutil.copyfileobj(client.get_rendered(self.handle), pngfile)

        self.image_path = f"/rendered/{self.handle}.png"
        self.status = Status.READY
