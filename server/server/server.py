import typing as t

from pcconfig import config

import pynecone as pc

docs_url = "https://pynecone.io/docs/getting-started/introduction"
filename = f"{config.app_name}/{config.app_name}.py"


class State(pc.State):
    """The app state."""

    strings: t.List[str] = [
        "Foo",
        "Bar",
        "Baz",
        "Qux",
    ]

    def next_item(self):
        print(self.strings.pop())

    
def leftpane():
    return pc.vstack(
        pc.image(src="https://placekitten.com/450/600", width = "450px", height = "600px"),
        pc.hstack(
            pc.button("Go"),
            pc.button("Pause"),
            pc.button("Next", on_click=lambda: State.next_item()),
        ),
    )

def queue_pane(text):
    return pc.text(text)

def rightpane():
    return pc.vstack(
        pc.foreach(State.strings, queue_pane)
    )


def index() -> pc.Component:
    return pc.center(
        pc.hstack(
            leftpane(),
            rightpane(),
            justify_content = "space-evenly"
        ),
        padding_top="10%",
    )


# Add state and page to the app.
app = pc.App(state=State)
app.add_page(index)
app.compile()
