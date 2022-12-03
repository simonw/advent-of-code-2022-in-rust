# Recursively search for Justfiles and run just against each one

# Use the find command to search for Justfiles in the current directory and its subdirectories
justfiles=$(find . -name "Justfile")
cargos=$(find . -name "Cargo.toml")

# Save the current directory so we can change back to it later
orig_dir=$(pwd)

# Loop through each Justfile
for justfile in $justfiles; do
  # Get the directory containing the Justfile
  dir=$(dirname $justfile)

  # Change to the directory containing the Justfile
  pushd $dir

  # Run just without any arguments
  just

  # Change back to the original directory
  popd
done

# Loop through each Cargo.toml
for cargo in $cargos; do
  # Get the directory containing the Cargo.toml
  dir=$(dirname $cargo)

  # Change to the directory containing the Cargo.toml
  pushd $dir

  # Run cargo
  cargo run

  # Change back to the original directory
  popd
done

# Change back to the original directory
cd $orig_dir
