#!/bin/bash

# Name of your project
PROJECT_NAME="cm_to_nanoacres"

# Build the project
echo "Building $PROJECT_NAME..."
cargo build --release

# Specify the target directory for the binary
# Change this to wherever you want, e.g., "/usr/local/bin" or "$HOME/bin"
TARGET_DIR="/usr/local/bin"

# Move the binary to the target directory
echo "Installing $PROJECT_NAME to $TARGET_DIR..."
sudo mv target/release/$PROJECT_NAME $TARGET_DIR/$PROJECT_NAME

# Alternatively, create a symlink
# sudo ln -s $(pwd)/target/release/$PROJECT_NAME $TARGET_DIR/$PROJECT_NAME

echo "$PROJECT_NAME installed successfully!"
