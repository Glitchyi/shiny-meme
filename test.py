from github import Github

g = Github()
repo = g.get_repo("Glitchyi/shiny-meme")
commit = repo.get_commit("ce1f41d70e940bed49fb95248440f7482efe8ddd")

updates = commit.stats.raw_data

print(updates)


