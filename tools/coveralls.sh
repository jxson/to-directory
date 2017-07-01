if [[ $TRAVIS_RUST_VERSION -eq "stable" ]]; then
  echo "== sending coverage info to coveralls"
  cargo kcov --verbose --coveralls
fi
