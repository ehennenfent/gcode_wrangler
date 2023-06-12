import pynecone as pc

class ServerConfig(pc.Config):
    pass

config = ServerConfig(
    app_name="server",
    backend_port=8001,
    api_url="http://calliope:8001",
    db_url="sqlite:///pynecone.db",
    env=pc.Env.DEV,
)