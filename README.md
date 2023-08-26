# gcode_wrangler

Control infrastructure for a robot that draws things.

## Setup Instructions
Client code can be found in [`/client`](https://github.com/ehennenfent/gcode_wrangler/tree/master/client). 
You'll need Python 3.9+
It's almost always easiest to set up a [virtual environment](https://docs.python.org/3/library/venv.html) before working on a new python project. 

1. Download the source code, ie `git clone https://github.com/ehennenfent/gcode_wrangler.git`
2. Activate your venv if using one
3. Install the client via pip: `cd gcode_wrangler/client; pip install -e .`


Once that's done, you can import `drawbot_client` in a Python file. See [example.py]() to get started. 

```python
from drawbot_client import Client
from drawbot_client.l_system import Seripinski

client = Client("server.drawbot.art", "tofu_security_is_best_security", "ehennenfent")

with client.session() as session:
    drawbot = session.drawbot()

    drawbot.penup()
    drawbot.goto(200, 200)

    for _ in range(4):
        drawbot.forward(160)
        drawbot.pendown()
        drawbot.right(90)

    drawbot.setheading(0)

    system = Seripinski(forward=drawbot.forward, left=drawbot.left, right=drawbot.right, size=10)

    system.evaluate(order=4)

    if input("Submit? (Y/N) > ").lower().startswith("y"):
        print(session.queue("example drawing"))
```

![drawbot](https://github.com/ehennenfent/gcode_wrangler/assets/7294647/fbd0bf2f-1271-4d84-b388-766e293e88ae)
