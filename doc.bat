cargo doc --no-deps -p "embre*"
rmdir /s ./docs
robocopy target/doc docs /s
echo '<meta http-equiv="refresh" content="0; url=embre/index.html">' > docs/index.html