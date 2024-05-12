#!/bin/bash

set -xe

wasm-pack build --release
(
	cd ./www
	npm install
	npm run build --release
)
git checkout deploy
(git rm *.js *.wasm *.html || echo "Could not (git rm)")
mv ./www/dist/* .
git add *.js *.wasm *.html
git commit -m "auto deploy"
git push
