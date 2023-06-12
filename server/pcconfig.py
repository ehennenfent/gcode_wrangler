import pynecone as pc

class ServerConfig(pc.Config):
    pass

config = ServerConfig(
    app_name="server",
    db_url="sqlite:///pynecone.db",
    env=pc.Env.DEV,
)