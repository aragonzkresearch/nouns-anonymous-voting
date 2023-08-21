#!/bin/sh

sed -i "5s/.*/global MAX_VOTERS: Field = $(head -1 max-num-voters);/" circuits/tally/src/main.nr
