echo $TRAVIS_RUST_VERSION
if [[ $TRAVIS_RUST_VERSION -eq "stable" ]]; then
  echo "Sending info to coveralls"
  cargo kcov --verbose --coveralls
fi
