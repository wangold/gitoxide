#!/bin/bash
set -eu -o pipefail

mkdir basics;

function baseline() {
  {
    echo "$1"
    GIT_ATTR_NOSYSTEM=1 git -c core.attributesFile=$PWD/user.attributes check-attr -a "$1"
    echo
  } >> baseline

  {
    echo "$1"
    GIT_ATTR_NOSYSTEM=1 git -c core.attributesFile=$PWD/user.attributes check-attr info test -- "$1"
    echo
  } >> baseline.selected
}


(cd basics
  git init

  # based on https://github.com/git/git/blob/140b9478dad5d19543c1cb4fd293ccec228f1240/t/t0003-attributes.sh#L45
	mkdir -p a/b/d a/c b
	(
		echo "[attr]notest !test"
		echo "\" d \"	test=d"
		echo " e	test=e"
		echo " e\"	test=e"
		echo "f	test=f"
		echo "a/i test=a/i"
		echo "onoff test -test"
		echo "offon -test test"
		echo "no notest"
		echo "A/e/F test=A/e/F"
		echo "\!escaped test-escaped"
		echo "**/recursive test-double-star-slash"
		echo "a**f test-double-star-no-slash"
		echo "dir-slash/ never"
		echo "dir/** always"
	) > .gitattributes
	(
		echo "g test=a/g"
		echo "b/g test=a/b/g"
	) > a/.gitattributes
	(
		echo "h test=a/b/h"
		echo "d/* test=a/b/d/*"
		echo "d/yes notest"
	) > a/b/.gitattributes
	(
		echo "global test=global"
		echo "z/x/a global-no-wildcard-case-test"
		echo "z/x/* global-wildcard-case-test"
	) > user.attributes

	git add . && git commit -qm c1
	(
		echo "global test=global"
		echo "* info=attributes"
	) > .git/info/attributes


  baseline z/x/a
  baseline Z/x/a
  baseline z/x/A
  baseline Z/X/a
  baseline Z/x/a
  baseline " d "
  baseline e
  baseline f
  baseline dir-slash
  baseline dir-slash/a
  baseline dir
  baseline dir/a
  baseline recursive
  baseline a/recursive
  baseline a/b/recursive
  baseline a/b/c/recursive
  baseline "!escaped"
  baseline af
  baseline axf
  baseline a/b/d/no
  baseline a/e/f
  baseline a/f
  baseline a/b/d/g
  baseline a/B/D/g
  baseline b/g
  baseline a/c/f
  baseline "e\""
  baseline a/i
  baseline A/b/h
  baseline A/B/D/NO
  baseline subdir/a/i
  baseline onoff
  baseline offon
  baseline no
  baseline A/e/F
  baseline a/e/F
  baseline a/e/f
  baseline a/g
  baseline a/b/g
  baseline a/b/h
  baseline a/b/d/ANY
  baseline a/b/d/yes
  baseline global
)
