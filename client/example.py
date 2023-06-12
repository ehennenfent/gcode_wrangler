from drawbot_client import Client
from drawbot_client.l_system import Seripinski

client = Client("drawbot.hennenfent.com", "SuperSecretSessionKey", "ehennenfent")

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

    if input("Submit? (Y/N)").lower().startswith("y"):
        session.queue()
