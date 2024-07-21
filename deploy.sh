#!/bin/bash

echo "Deployment process started..."

# Build the Rust application in release mode using Cargo
echo "Building the Rust application in release mode..."
cargo build --release

# Check if the build was successful
if [ $? -eq 0 ]; then
    echo "Build successful!"
else
    echo "Build failed! Aborting deployment..."
    exit 1
fi

echo "Submitting the build to Google Cloud Build..."

gcloud builds submit .

# Check if the submission was successful
if [ $? -eq 0 ]; then
    echo "Google Cloud Build submission successful!"
else
    echo "Google Cloud Build submission failed!"
    exit 1
fi

echo "Deployment process completed."
