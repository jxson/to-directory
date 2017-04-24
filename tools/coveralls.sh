if [[ $TRAVIS_RUST_VERSION = "stable" ]]; then
  cargo kcov --verbose --coveralls
fi
