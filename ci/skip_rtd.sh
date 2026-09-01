#!/usr/bin/env bash

CODE_SKIP=183

# never skip on main or on tags
current_branch=$(git branch --show-current)

([ "$current_branch" == "main" ] || git describe --tags --exact-match 2>/dev/null >/dev/null) && exit 0

n_modifying_commits=$(git --no-pager log --pretty="tformat:%s" main..HEAD -- .readthedocs.yml Cargo.toml Cargo.lock ci/rattler-recipe healpix-geo healpix-geo-python | wc -l)
most_recent_commit_msg=$(git --no-pager log --pretty="tformat:%s" -1)

if echo "$most_recent_commit_msg" | grep -qF "[run-rtd]"; then
    # always run, regardless of other conditions
    exit 0
fi

# always skip if in the message
(echo "$most_recent_commit_msg" | grep -vqF "[skip-rtd]") || exit $CODE_SKIP

# skip if there are no commits modifying relevant files
[ $n_modifying_commits -eq 0 ] && exit $CODE_SKIP

# explicitly set exit code
exit 0
