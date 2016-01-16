#!/bin/sh

git fetch

LOCAL=$(git rev-parse @)
REMOTE=$(git rev-parse @{u})

if [ $LOCAL != $REMOTE ]; then
	git pull && cargo build
fi

./target/debug/sousvide

if [ $? == 0 ]; then
	sudo shutdown
else
	sudo reboot
fi
