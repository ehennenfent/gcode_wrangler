import typing as t

import requests

from .config import drawbot_host

Handle = int


class GcodeClient:
    def __init__(self) -> None:
        self.drawbot_host = drawbot_host

    def _get_endpoint(self, extension, *args, **kwargs):
        try:
            return requests.get(f"http://{self.drawbot_host}/{extension}", *args, **kwargs)
        except ConnectionError as e:
            print(f"Error calling {self.drawbot_host}: {e}")
            return None

    def _post_endpoint(self, extension, *args, **kwargs):
        try:
            return requests.post(f"http://{self.drawbot_host}/{extension}", *args, **kwargs)
        except ConnectionError as e:
            print(f"Error calling {self.drawbot_host}: {e}")
            return None

    def start_run(self, handle: Handle):
        return self._post_endpoint(f"run/{handle}")

    # def get_progress(self, handle: Handle):
    #     return self._get_endpoint(f"run/{handle}").json()

    def get_rendered(self, handle: Handle):
        maybe_image = self._get_endpoint(f"rendered/{handle}", stream=True)
        if maybe_image is not None:
            return maybe_image.content

    def upload(self, movements: t.List["Movement"]) -> Handle:
        print("Uploading", len(movements), "to server")
        maybe_handle = self._post_endpoint("movements", json=[m.nested_dict() for m in movements])
        if maybe_handle is not None:
            print("-->", (handle := maybe_handle.text))
            return handle

    def pause(self):
        print("Pausing...")
        return self._post_endpoint("pause")

    def resume(self):
        print("Resuming...")
        return self._post_endpoint("resume")

    def cancel(self):
        print("Stopping...")
        return self._post_endpoint("cancel")

    def get_machine(self):
        maybe_details = self._get_endpoint("machine")
        if maybe_details is not None:
            return maybe_details.json()
