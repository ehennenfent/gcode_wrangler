import typing as t

from .turtle_wrapper import Drawbot


class SessionManager:
    def __enter__(self) -> "SessionManager":
        return self

    def __exit__(self, *_args: t.Any) -> None:
        pass

    def drawbot(self) -> Drawbot:
        return Drawbot()

    def queue(self) -> t.Any:
        pass


class Client:
    def __init__(self, host: str, sessionkey: str, username: str) -> None:
        self.host = host
        self.sessionkey = sessionkey
        self.username = username

    def session(self) -> SessionManager:
        return SessionManager()
