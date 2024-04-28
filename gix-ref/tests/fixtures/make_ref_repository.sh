#!/bin/bash
set -eu -o pipefail

git init -q

git checkout -q -b main
git commit -q --allow-empty -m c1
git branch dt1
git branch d1

mkdir -p .git/refs/remotes/origin

cp .git/refs/heads/main .git/refs/remotes/origin/
cp .git/refs/heads/main .git/refs/d1

echo "ref: refs/remotes/origin/main" > .git/refs/remotes/origin/HEAD
echo "notahexsha" > .git/refs/broken

echo "ref: refs/heads/multi-link-target1" > .git/refs/multi-link
echo "ref: refs/tags/multi-link-target2" > .git/refs/heads/multi-link-target1
echo "ref: refs/remotes/origin/multi-link-target3" > .git/refs/tags/multi-link-target2
git rev-parse HEAD > .git/refs/remotes/origin/multi-link-target3


echo "ref: refs/loop-b" > .git/refs/loop-a
echo "ref: refs/loop-a" > .git/refs/loop-b

cat <<EOF >> .git/FETCH_HEAD
9064ea31fae4dc59a56bdd3a06c0ddc990ee689e		branch 'main' of https://github.com/Byron/gitoxide
1b8d9e6a408e480ae1912e919c37a26e5c46639d	not-for-merge	branch 'faster-discovery' of https://github.com/Byron/gitoxide
43f695a9607f1f85f859f2ef944b785b5b6dd238	not-for-merge	branch 'fix-823' of https://github.com/Byron/gitoxide
96267708958ead2646aae8766a50fa060739003c	not-for-merge	branch 'fix-bare-with-index' of https://github.com/Byron/gitoxide
1397e19375bb98522f951b8a452b08c1b35ffbac	not-for-merge	branch 'gix-archive' of https://github.com/Byron/gitoxide
db71ec8b7c7f2730c47dde3bb662ab56ae89ae7d	not-for-merge	branch 'index-from-files' of https://github.com/Byron/gitoxide
9f0c71917e57653d2e7121eae65d9385a188a8df	not-for-merge	branch 'moonwalk' of https://github.com/Byron/gitoxide
44d2b67de5639d4ea3d08ab030ecfe4bdfc8cbfb	not-for-merge	branch 'release-gix' of https://github.com/Byron/gitoxide
37c3d073b15dafcb52b2040e4b92a413c69a726d	not-for-merge	branch 'smart-release-without-git2' of https://github.com/Byron/gitoxide
af3608ad397784795c3758a1ac99ec6a367de9be	not-for-merge	branch 'walk-with-commitgraph' of https://github.com/Byron/gitoxide
EOF

git tag t1
git tag -m "tag object" dt1
