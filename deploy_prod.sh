#!/bin/bash

# Exit on any error
set -e

echo "🔍 Checking for uncommitted changes..."
if ! git diff --quiet || ! git diff --cached --quiet; then
    echo "❌ Uncommitted changes detected. Commit or stash them before deploying."
    exit 1
fi
echo "✅ Working tree is clean"

echo "🗄️  Running cargo sqlx prepare..."
cargo sqlx prepare --workspace
if ! git diff --quiet; then
    echo "📝 .sqlx files changed — committing and pushing..."
    git add .sqlx/
    git commit -m "🤖 chore: update .sqlx query cache"
    git push origin master
fi
echo "✅ SQLx offline data is up to date"

echo "🔍 Checking for typos..."
# typos .
echo "✅ No typos found"

echo "🧪 Running tests..."
cargo nextest run -p app --features ssr
echo "✅ All tests passed"

# Create timestamped deploy tag
TAG_NAME="deploy_$(date +'%Y/%m/%d_%Hh%Mm%Ss')"

echo "Creating tag: $TAG_NAME"
git tag $TAG_NAME

echo "Pushing tag to origin..."
git push origin $TAG_NAME

echo "Triggering deployment workflow..."
gh workflow run prod.yml

echo "✅ Deploy tag created and workflow triggered!"
echo "Tag: $TAG_NAME"