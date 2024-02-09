#!/bin/bash

# Build the project
cargo build --release --target wasm32-unknown-unknown

# Create the dist folder
if [ ! -d "dist" ]; then
  git clone git@github.com:Robotechnic/newtonfractal.git dist --branch gh-page --single-branch 
fi

cp -r assets/* dist/
cp target/wasm32-unknown-unknown/release/newton_fractal.wasm dist/
cp index.html dist/

# push to the gh-page branch
cd dist
git add .
git commit -m "Deploy"
git push origin gh-page
