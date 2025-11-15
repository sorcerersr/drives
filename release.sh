#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if dry-run flag is set
DRY_RUN=false
if [[ "$1" == "--dry-run" ]]; then
    DRY_RUN=true
    echo -e "${YELLOW}Dry run mode enabled. No changes will be made.${NC}"
fi

# Function to print status messages
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

# Function to print error messages
print_error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

# Function to run commands with optional dry-run
run_command() {
    if [ "$DRY_RUN" = true ]; then
        echo -e "${YELLOW}Would run:${NC} $*"
    else
        "$@"
    fi
}

# Check if we are on the main branch
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
if [[ "$CURRENT_BRANCH" != "main" ]]; then
    print_error "This script must be run from the main branch. You are currently on '$CURRENT_BRANCH'."
    exit 1
fi

print_status "Running cargo build..."
if ! run_command cargo build; then
    print_error "Cargo build failed. Aborting."
    exit 1
fi

print_status "Running cargo test..."
if ! run_command cargo test; then
    print_error "Cargo tests failed. Aborting."
    exit 1
fi

# Read current version from Cargo.toml
CURRENT_VERSION=$(grep -E '^version\s*=' Cargo.toml | cut -d '"' -f 2)
print_status "Current version: $CURRENT_VERSION"

# Parse version numbers
IFS='.' read -r major minor patch <<< "$CURRENT_VERSION"

# Prompt user for new version type
echo ""
echo "Choose version bump type:"
echo "1) Major (e.g., 0.1.8 -> 1.0.0)"
echo "2) Minor (e.g., 0.1.8 -> 0.2.0)"
echo "3) Patch (e.g., 0.1.8 -> 0.1.9)"
echo "4) Custom version"
read -p "Enter your choice (1-4): " choice

case $choice in
    1)
        NEW_VERSION="$((major + 1)).0.0"
        ;;
    2)
        NEW_VERSION="${major}.$((minor + 1)).0"
        ;;
    3)
        NEW_VERSION="${major}.${minor}.$((patch + 1))"
        ;;
    4)
        read -p "Enter custom version (e.g., 1.2.3): " NEW_VERSION
        ;;
    *)
        print_error "Invalid choice. Exiting."
        exit 1
        ;;
esac

print_status "New version will be: $NEW_VERSION"

# Update Cargo.toml with new version
run_command sed -i "s/version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" Cargo.toml

print_status "Updated Cargo.toml to version $NEW_VERSION"

# Create git commit
COMMIT_MSG="bump version to $NEW_VERSION"
run_command git add Cargo.toml
run_command git commit -m "$COMMIT_MSG"
print_status "Created commit: $COMMIT_MSG"

# Create git tag
TAG_NAME="v$NEW_VERSION"
run_command git tag "$TAG_NAME"
print_status "Created tag: $TAG_NAME"

# Ask for permission to push
echo ""
read -p "Do you want to push the commit and tag? (y/N): " -n 1 -r
echo ""
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    print_status "Push cancelled by user. You can still push manually with:"
    echo "git push origin main"
    echo "git push origin $TAG_NAME"
    exit 0
fi

# Push commit and tag
print_status "Pushing commit and tag..."
run_command git push origin main
run_command git push origin "$TAG_NAME"

print_status "Release process completed successfully!"
echo -e "${GREEN}New version: $NEW_VERSION${NC}"
echo -e "${GREEN}Tag: $TAG_NAME${NC}"
