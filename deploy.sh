#!/bin/bash

set -xe

wasm-pack build --release
(
	cd ./www
	npm install
	npm run build --release
)
git checkout deploy
mv ./www/dist/* .
(git rm *.js *.wasm *.html || echo "Could not (git rm)")
git add *.js *.wasm *.html
git commit -m "auto deploy"
git push
