cargo doc
FROM=$(git rev-parse --short HEAD)
cp -a target/doc .
git checkout gh-pages
cp -r doc/{*,.*} .
rm doc -rf
git add .
git commit -m "Update doc for ${FROM}"
git push
git checkout master
