cargo doc --release --no-deps
rm -rf ./docs
echo "<meta http-equiv=\"refresh\" content=\"0; url=twothousand_forty_eight\">" > target/doc/index.html
cp -r target/doc ./docs