import typing as t
import requests
from urllib.parse import urljoin


from .turtle_wrapper import Drawbot


class SessionManager:
    def __init__(self, client: "Client"):
        self.client = client
        self._drawbot: t.Optional[Drawbot] = None

    def __enter__(self) -> "SessionManager":
        return self

    def __exit__(self, *_args: t.Any) -> None:
        self._drawbot = None

    def drawbot(self) -> Drawbot:
        self._drawbot = Drawbot()
        return self._drawbot

    def queue(self, comment: str = "") -> t.Any:
        self._client.submit(self._drawbot._movements, comment)


class Client:
    def __init__(self, host: str, sessionkey: str, username: str) -> None:
        self.host = host
        self.sessionkey = sessionkey
        self.username = username

    def session(self) -> SessionManager:
        return SessionManager(self)

    def _submit(self, movements, comment: str = ""):
        return requests.post(urljoin(self.host, "submit"), data={
            "username": self.username,
            "comment": comment,
            "movements": movements,
        })