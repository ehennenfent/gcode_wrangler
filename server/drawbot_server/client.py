import typing as t
from urllib.parse import urljoin

import requests

from .config import drawbot_host

Handle = int


class GcodeClient:
    def __init__(self) -> None:
        self.drawbot_host = drawbot_host

    def _get_endpoint(self, extension, *args, **kwargs):
        return requests.get(f"http://{self.drawbot_host}/{extension}", *args, **kwargs)

    def _post_endpoint(self, extension, *args, **kwargs):
        return requests.post(f"http://{self.drawbot_host}/{extension}", *args, **kwargs)

    def start_run(self, handle: Handle):
        return self._post_endpoint(f"run/{handle}")

    def get_progress(self, handle: Handle):
        return self._get_endpoint(f"run/{handle}").json()

    def get_rendered(self, handle: Handle):
        request = self._get_endpoint(f"rendered/{handle}", stream=True)
        print("Got", len(request.content), "bytes")
        print(request)
        return request.raw

    def upload(self, movements: t.List["Movement"]) -> Handle:
        print("Uploading", len(movements), "to server")
        handle = self._post_endpoint(f"movements", json=[m.nested_dict() for m in movements]).text
        print("-->", handle)
        return handle

    def pause(self):
        print("Pausing...")
        return self._post_endpoint(f"pause")

    def resume(self):
        print("Resuming...")
        return self._post_endpoint(f"resume")
    
    def resume(self):
        print("Stopping...")
        return self._post_endpoint(f"stop")

    def get_machine(self):
        details = self._get_endpoint("machine").json()
        print("Got machine details:", details)
        return details
