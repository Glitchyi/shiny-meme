import requests
import os
from dotenv import load_dotenv

load_dotenv()

CLIENT_ID = os.getenv('GITHUB_CLIENT_ID')
CLIENT_SECRET = os.getenv('GITHUB_CLIENT_SECRET')
REDIRECT_URI = os.getenv('REDIRECT_URI', 'http://localhost:5000/callback')

def get_github_login_url():
    return (
        f"https://github.com/login/oauth/authorize"
        f"?client_id={CLIENT_ID}&redirect_uri={REDIRECT_URI}&scope=repo,user"
    )

def get_github_token(code):
    response = requests.post(
        "https://github.com/login/oauth/access_token",
        data={
            'client_id': CLIENT_ID,
            'client_secret': CLIENT_SECRET,
            'code': code,
            'redirect_uri': REDIRECT_URI,
        },
        headers={'Accept': 'application/json'},
    )
    response.raise_for_status()
    return response.json()['access_token']

def fetch_github_user(token):
    response = requests.get(
        "https://api.github.com/user",
        headers={'Authorization': f'token {token}'},
    )
    response.raise_for_status()
    return response.json()
