import libsql_client
import os

db_url = os.getenv("LIBSQL_DB_URL", "file:app.db")
auth_token = os.getenv("LIBSQL_AUTH_TOKEN", None)

client = libsql_client.create_client_sync(db_url, auth_token=auth_token)
