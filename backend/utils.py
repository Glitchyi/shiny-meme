import random
import string

def random_api_key(client):
    """Generate and return a random API key"""
    key = ''.join(random.choices(string.ascii_letters + string.digits, k=32))
    client.execute("INSERT INTO api_keys (key) VALUES (?)", (key,))
    return key

def calculate_leaderboard(client):
    """Calculate leaderboard based on the data in the database"""
    leaderboard = client.execute("""
    SELECT users.name, COUNT(repos.id) as contributions
    FROM users
    INNER JOIN repos ON users.id = repos.user_id
    GROUP BY users.id
    ORDER BY contributions DESC
    """)
    return list(leaderboard)
