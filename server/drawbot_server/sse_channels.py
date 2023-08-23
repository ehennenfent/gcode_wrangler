import typing as t
from asyncio import Queue
from random import getrandbits


class QueueManager:
    class EventQueue:
        def __init__(self, parent: "QueueManager"):
            self._parent = parent
            self.handle = None

        def __enter__(self) -> "EventQueue":
            self.handle = getrandbits(64)
            self._parent._create_channel(self.handle)
            return self

        def __exit__(self, *_args: t.Any) -> None:
            self._parent._close_channel(self.handle)
            self._drawbot = None

        def read(self):
            return self._parent._read(self.handle)

    def __init__(self, maxsize=0):
        self._queues: t.Dict[str, Queue] = {}
        self._maxsize = maxsize

    def join(self) -> EventQueue:
        return QueueManager.EventQueue(self)

    def _create_channel(self, handle: int):
        print("Creating event channel", handle)
        self._queues[handle] = Queue(maxsize=self._maxsize)

    def _close_channel(self, handle: int):
        print("Closing event channel", handle)
        self._queues.pop(handle)

    def _read(self, handle):
        return self._queues[handle].get()

    def broadcast(self, message: str) -> t.Any:
        for _, q in self._queues.items():
            q.put_nowait(message)
