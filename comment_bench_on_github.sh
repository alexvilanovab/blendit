#!/bin/sh

set -e

before=`cat before`
after=`cat after`
body=$(echo -n '### Benchmark Results\n\nBefore:\n```none\n'"$before"'\n```\n\nAfter:\n```none\n'"$after"'\n```' | sed -e ':a' -e 'N' -e '$!ba' -e 's/\n/\\n/g')

if [ -n "$CIRCLE_PULL_REQUEST" ]; then
	location=$(echo $CIRCLE_PULL_REQUEST | grep -Po 'github.com/\K[^/]+/[^/]+')
	curl -s -u ${GH_AUTH_TOKEN:?}:x-oauth-basic --data '{"body":"'"$body"'"}' \
		https://api.github.com/repos/$location/issues/${CIRCLE_PULL_REQUEST##*/}/comments
else
	curl -s -u ${GH_AUTH_TOKEN:?}:x-oauth-basic --data '{"body":"'"$body"'"}' \
		https://api.github.com/repos/${CIRCLE_PROJECT_USERNAME:?}/${CIRCLE_PROJECT_REPONAME:?}/commits/${CIRCLE_SHA1:?}/comments
fi
