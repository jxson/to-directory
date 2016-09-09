
HAS_FLAGS_REGEX="^-([a-z-]+)$";
VERBOSE=false;
CHANGE_DIRECTORY=true;

# No args should show help.
to() {
  # Allowed flags for changing a directory: -v. --verbose

  # TODO(jxson): If there is a verbose flag set filter out the log output
  # from the command. Allow log prefixed lines to pass through. eval any
  # other line.

  # to-directory "$@"
  for arg in "$@"; do
    # Verbose flag is the only extra option passed during a cd.
    if [[ $arg == "-v" ]] || [[ $arg == "--verbose" ]]; then
      VERBOSE=true
      continue
    fi

    if [[ $arg == "-h" ]] || [[ $arg == "--help" ]]; then
      echo "to shell => help flag, bailing"
      to-directory "$@"
    fi

    if [[ $arg =~ $HAS_FLAGS_REGEX ]]; then
      CHANGE_DIRECTORY=false
      break
    fi
  done

  echo "to shell => args: $@"
  echo "to shell => verbose: ${VERBOSE}"
  echo "to shell => should cd: ${CHANGE_DIRECTORY}"

  if [[ CHANGE_DIRECTORY == false ]]; then
    echo "to shell => changing"
    to-directory "$@"
  else
    echo "to shell => not changing"
    # Capture output, if to-directory exits 0 cd to stdout, otherwise show
    # output and exit.
    if directory=$(to-directory "$@"); then
      cd ~
    else
      # Still needs work to exit with stderror.
      echo "error"
      exit $?
    fi
  fi
}
