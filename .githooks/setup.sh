#!/bin/bash
#
# Setup script for git hooks
# Run this script to install the git hooks for the Bagel Browser project
#

set -e

HOOKS_DIR=".githooks"
GIT_HOOKS_DIR=".git/hooks"

echo "🔧 Setting up git hooks for Bagel Browser..."

# Check if we're in a git repository
if [ ! -d ".git" ]; then
    echo "❌ Not in a git repository. Please run this script from the project root."
    exit 1
fi

# Check if hooks directory exists
if [ ! -d "$HOOKS_DIR" ]; then
    echo "❌ Hooks directory '$HOOKS_DIR' not found."
    exit 1
fi

# Make hooks executable
echo "📝 Making hooks executable..."
chmod +x "$HOOKS_DIR"/*

# Configure git to use our hooks directory
echo "⚙️  Configuring git to use custom hooks directory..."
git config core.hooksPath "$HOOKS_DIR"

echo "✅ Git hooks setup complete!"
echo ""
echo "The following hooks are now active:"
echo "  • pre-commit:  Runs formatting, linting, and tests"
echo "  • commit-msg:  Validates conventional commit format"
echo "  • pre-push:    Prevents direct pushes to main and runs tests"
echo ""
echo "To disable hooks temporarily, use:"
echo "  git commit --no-verify"
echo "  git push --no-verify"
echo ""
echo "To disable hooks permanently:"
echo "  git config --unset core.hooksPath"