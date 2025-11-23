# list all available subcommands
_default:
  @just --list

# cargo nextest the given test case(s) and output logs
t CASE:
  cargo nextest run {{CASE}} --no-capture

# cargo nextest the given test file(s)
tf TEST:
  cargo nextest run --test {{TEST}}

# cargo nextest the given test file(s) and output logs
tfl TEST:
  cargo nextest run --test {{TEST}} --no-capture
