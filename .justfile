# Like GNU `make`, but `just` rustier.
# https://just.systems/
# run `just` from this directory to see available commands

alias b := build
alias w := web
alias r := run
alias c := clean
alias f := format

# Default command when 'just' is run without arguments
default:
  @just --list

# Build the project
build:
  @echo "Building..."
  @cargo build

# Build the project for web
web:
  @echo "Building for web..."
  @trunk build

# Run a package
run *args='':
  @echo "Running..."
  cargo run {{args}}

# Remove build artifacts and non-essential files
clean:
  @echo "Cleaning..."
  @find . -type d -name "target" -exec rm -rf {} +
  @find . -type d -name "dist" -exec rm -rf {} +

# Run code quality tools
check:
  @echo "Running code quality tools..."
  @cppcheck --error-exitcode=1 --project=build/compile_commands.json -i build/_deps/

# Format the project
format:
  @echo "Formatting..."
  @cargo fmt
  @find . -name "*.nix" -type f -exec nixfmt {} \;
