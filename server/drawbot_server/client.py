import typing as t
from urllib.parse import urljoin

import requests

from .config import drawbot_host

Handle = int


class GcodeClient:
    def __init__(self) -> None:
        self.drawbot_host = drawbot_host

    def _get_endpoint(self, extension, *args, **kwargs):
        return requests.get(f"http://{self.drawbot_host}/extension", *args, **kwargs)

    def _post_endpoint(self, extension, *args, **kwargs):
        return requests.post(f"http://{self.drawbot_host}/extension", *args, **kwargs)

    def start_run(self, handle: Handle):
        return self._post_endpoint(f"run/{handle}")

    def get_run(self, handle: Handle):
        return self._get_endpoint(f"run/{handle}")

    def analyze(self, handle: Handle):
        return self._get_endpoint(f"analysis/{handle}")

    def upload(self, movements: t.List["Movement"]) -> Handle:
        return self._post_endpoint(f"movements")

    def pause(self, handle: Handle):
        return self._post_endpoint(f"pause/{handle}")

    def get_machine(self):
        return self._get_endpoint("machine")
