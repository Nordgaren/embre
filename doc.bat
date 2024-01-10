cargo doc --no-deps -p "embre*"
rmdir /s ./docs
robocopy target/doc docs /s
echo|set /p="<meta http-equiv="refresh" content="0; url=PROJECT_NAME/index.html">" > docs/index.html