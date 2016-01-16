#!/bin/sh

while true; do
	# check for updates
	git fetch

	LOCAL=$(git rev-parse @)
	REMOTE=$(git rev-parse @{u})

	# if there's an update, fetch and build it
	if [ $LOCAL != $REMOTE ]; then
		git pull && cargo build
	fi

	# run the sousvide
	./target/debug/sousvide
done
