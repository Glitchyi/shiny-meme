# Github stats collector

### TODO

- [x] Number of lines updates
- [X] Github Webhook
- [ ] Add DB [libSQL](https://github.com/tursodatabase/libsql)
- [ ] Websocket
- [ ] PR Count
- [ ] Tracking user PR's
- [ ] Leaderboard


Info about commit 
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
