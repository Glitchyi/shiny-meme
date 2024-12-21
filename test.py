from github import Github

g = Github()
repo = g.get_repo("Glitchyi/super-barnacle")
commit = repo.get_commit("da44c18641b8bec6b0f091c5d1aeaaf551569fb9")

updates = commit.stats.raw_data

print(updates)


