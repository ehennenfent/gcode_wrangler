from drawbot_client import Drawbot

class SessionManager():

    def __enter__(self):
        return self

    def __exit__(self):
        pass

    def drawbot(self):
        return Drawbot()
    
    def queue(self):
        pass

class Client:
    
    def __init__(self, host, sessionkey, username):
        self.host = host
        self.sessionkey = sessionkey
        self.username = username

    def session():
        return SessionManager()
    

