
Use case: A developer sends to us a PR in the form of a branch from our main. We want to checkout it from the trunk repo.

Example, PR number #36.

# Step 1: get the PR's last commit hash

go to https://github.com/aindustriosa/RustyBugA/pull/36/commits and read the last commit hash, in our case it is `e02da84f0330067981b349d6f264f0fc94a208db`

# Step 2: Get reference branch from the remote repo 

From our trunk repo:
```commandline
RustyBugA  git pull
RustyBugA  git checkout main
RustyBugA  git ls-remote --refs origin | grep e02da84f
e02da84f0330067981b349d6f264f0fc94a208db	refs/pull/36/head

RustyBugA  git fetch origin pull/36/head:36-update-tools
[...]
 * [nueva referencia] refs/pull/36/head -> 36-update-tools

RustyBugA  git checkout 36-update-tools
```

