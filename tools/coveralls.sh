if [[ $TRAVIS_RUST_VERSION = "stable" ]]; then
  echo "Sending info to coveralls"
  cargo kcov --verbose --coveralls
fi
