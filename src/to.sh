
TO_HAS_FLAGS_REGEX="^-([a-z-]+)$";

to() {
  local VERBOSE=false;
  local CHANGE_DIRECTORY=true;


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

    if [[ $arg =~ $TO_HAS_FLAGS_REGEX ]]; then
      CHANGE_DIRECTORY=false
    fi
  done

  if [[ $CHANGE_DIRECTORY == "true" ]]; then
    local directory
    directory=$(to-directory "$@")
    local -r status=$?

    if [[ $status -ne 0 ]]; then
      return $status
    else
      cd $directory
    fi
  else
    to-directory "$@"
  fi
}
