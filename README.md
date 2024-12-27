Freatures to add:
- Integrate GitHub OAuth for user login and token retrieval.
- Add SQLite schema for:
  - Users
  - API keys (for rate limit handling)
  - Repositories and pull requests
- Dashboard:
  - Total repositories and pull requests (open/closed).
  - User leaderboard with points for accepted pull requests.
- Dockerize the application for easy deployment.
Basic Steps:
1)GitHub OAuth Integration:
  -Set up OAuth using GitHub’s API to allow users to log in.
  -Save access tokens securely in the users table of your SQLite database.

2)SQLite Database Changes:
  -Create or update the schema to include tables for:
  -users: Stores GitHub user data.
  -api_keys: Stores GitHub API keys for rate-limiting.
  -repos: Tracks repositories users contribute to.
  -pull_requests: Tracks pull request details and statuses.

3)Dashboard API Endpoints:
  -Write API endpoints to:
  -Fetch user stats (lines committed, repos contributed, etc.).
  -Display leaderboard based on pull request points.

4)Random API Key Selection:
  -Implement logic to pick an API key randomly from the api_keys table for each request.

5)Dockerize the Application:
  -Write a Dockerfile to containerize the app for deployment.
  -Create a docker-compose.yml if needed for database or multi-container setups.

# Github stats collector

### TODO

- [ ] Better name suggestion for project (prolly SOC backend rebrand)
- [x] Number of lines updates
- [X] Github Webhook
- [ ] Add DB [libSQL](https://github.com/tursodatabase/libsql)
- [ ] Websocket
- [ ] PR Count
- [ ] Tracking user PR's
- [ ] Leaderboard


### API's used to fetch github stats

- File/line changes
    `https://api.github.com/repos/{Owner}/{repo}/commits/{sha}`
    
    ```txt
    
    a=0
    d=0
    t=0
    
    for i in json['files']:
        a += i['additions']
        d += i['deletions']
        t += i['changes']
    a = additions
    d = deletions
    t = total lines changed
    
    ```
