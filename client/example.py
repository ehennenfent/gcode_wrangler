from drawbot_client import Client, Seripinski

client = Client("drawbot.hennenfent.com", "SuperSecretSessionKey", "ehennenfent")

with client.session() as session:
    drawbot = session.drawbot()

    drawbot.penup()
    drawbot.goto(50, 50)
    drawbot.pendown()

    for _ in range(3):
        drawbot.forward(100)
        drawbot.left(90)

    drawbot.setheading(0)

    system = Seripinski(forward=drawbot.forward, left=drawbot.left, right=drawbot.right, size=10)

    system.evaluate(order=4)

    session.queue()
