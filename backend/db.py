import os
import libsql_client
import logging

# Load the database URL from environment variables
db_url = os.getenv('LIBSQL_DB_URL', 'file:./app.db')

# Create the database client
try:
    client = libsql_client.create_client_sync(db_url)
    logging.debug(f"Connected to database at {db_url}")
except Exception as e:
    logging.error(f"Error connecting to database: {e}")

def setup_database():
    logging.debug("Setting up database.")
    client.execute("""
    CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY,
        github_id TEXT UNIQUE NOT NULL,
        name TEXT NOT NULL,
        email TEXT,
        token TEXT NOT NULL
    );
    """)
    logging.debug("Created/Checked users table.")
    
    client.execute("""
    CREATE TABLE IF NOT EXISTS api_keys (
        id INTEGER PRIMARY KEY,
        key TEXT NOT NULL
    );
    """)
    logging.debug("Created/Checked api_keys table.")
    
    client.execute("""
    CREATE TABLE IF NOT EXISTS repos (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        is_private BOOLEAN DEFAULT FALSE,
        user_id INTEGER,
        FOREIGN KEY(user_id) REFERENCES users(id)
    );
    """)
    logging.debug("Created/Checked repos table.")
    
    client.execute("""
    CREATE TABLE IF NOT EXISTS pull_requests (
        id INTEGER PRIMARY KEY,
        repo_id INTEGER,
        status TEXT NOT NULL,
        points INTEGER DEFAULT 0,
        FOREIGN KEY(repo_id) REFERENCES repos(id)
    );
    """)
    logging.debug("Created/Checked pull_requests table.")

def save_user_to_db(github_user, token):
    logging.debug(f"Attempting to save user {github_user['id']} to database.")
    client.execute("""
    INSERT OR IGNORE INTO users (github_id, name, email, token)
    VALUES (?, ?, ?, ?)
    """, (github_user['id'], github_user['name'], github_user.get('email', ''), token))
    logging.debug(f"User {github_user['id']} saved.")

def get_all_users():
    res= client.execute("SELECT * FROM users")
    return list(res)
