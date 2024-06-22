$data = cat .version

git tag $data; git push origin $data