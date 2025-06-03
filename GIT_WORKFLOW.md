# Git Workflow and Release Management

This document outlines the comprehensive Git workflow and automated release management system for the Bagel Browser project.

## Branch Strategy

### Main Branches
- **`main`**: Production-ready, stable releases only
- **`dev`**: Development branch for feature integration and testing

### Feature Branches
- **`feature/*`**: New features (branch from `dev`, merge back to `dev`)
- **`bugfix/*`**: Bug fixes (branch from `dev`, merge back to `dev`)
- **`hotfix/*`**: Critical bug fixes that need immediate release (branch from `main`)
- **`release/*`**: Release preparation (branch from `dev`, merge to both `main` and `dev`)

## Conventional Commits

We use [Conventional Commits](https://www.conventionalcommits.org/) for consistent commit messages:

### Format
```
type(scope): description

[optional body]

[optional footer(s)]
```

### Types
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Test changes
- `chore`: Build, dependencies, or other maintenance

### Examples
```
feat: add dark mode toggle
feat(ui): add navigation sidebar
fix: resolve memory leak in renderer
docs: update installation instructions
chore: bump dependencies to latest versions
```

## Semantic Versioning

We follow [Semantic Versioning](https://semver.org/) (SemVer):

### Version Format: `MAJOR.MINOR.PATCH`
- **MAJOR**: Breaking changes or complete rewrites
- **MINOR**: New features, backwards compatible
- **PATCH**: Bug fixes, security patches

### Automated Version Bumping
Conventional commits trigger automatic version updates:
- `feat:` → Minor version bump
- `fix:` → Patch version bump  
- `feat!:` or `BREAKING CHANGE:` → Major version bump

## Development Workflow

### 1. Starting New Work

```bash
# Start from dev branch
git checkout dev
git pull origin dev

# Create feature branch
git checkout -b feature/your-feature-name
```

### 2. Making Changes

```bash
# Make your changes
git add .
git commit -m "feat: add new navigation feature"

# Push feature branch
git push origin feature/your-feature-name
```

### 3. Creating Pull Request

1. Go to GitHub and create a PR from your feature branch to `dev`
2. Fill out the PR template with required information
3. Request reviews from team members
4. Address any feedback and update your branch

### 4. Merging

Once approved:
- PRs are merged using "Squash and merge" to maintain clean history
- Delete the feature branch after merging

## Release Process

### 1. Prepare Release

```bash
# Create release branch from dev
git checkout dev
git pull origin dev
git checkout -b release/v1.2.0

# Update version in Cargo.toml
# Test thoroughly
# Update documentation if needed

git add .
git commit -m "chore: prepare release v1.2.0"
git push origin release/v1.2.0
```

### 2. Deploy Release

```bash
# Merge to main (triggers release)
git checkout main
git pull origin main
git merge release/v1.2.0 --no-ff
git push origin main

# Create and push tag
git tag -a v1.2.0 -m "Release version 1.2.0"
git push origin main --tags

# Merge back to dev
git checkout dev
git merge release/v1.2.0 --no-ff
git push origin dev

# Clean up
git branch -d release/v1.2.0
git push origin --delete release/v1.2.0
```

### 3. Automated Release (Alternative)

For automatic releases, commit to `dev` with `[release]` in the commit message:

```bash
git commit -m "feat: major new feature [release]"
git push origin dev
```

This triggers the auto-merge workflow that:
1. Merges `dev` to `main`
2. Creates a tag based on the version in `Cargo.toml`
3. Triggers the release workflow

## GitHub Actions Workflows

### Continuous Integration (`.github/workflows/ci.yml`)
- Runs on pushes and PRs to `main` and `dev`
- Executes: formatting check, clippy lints, tests, security audit
- Uses caching for faster builds

### Release (`.github/workflows/release.yml`)
- Triggers on version tags (`v*`)
- Builds binaries for multiple platforms
- Creates GitHub release with changelog and assets
- Generates SHA256 checksums for verification

### Auto-merge (`.github/workflows/auto-merge.yml`)
- Triggers on pushes to `dev` with `[release]` in commit message
- Automatically merges `dev` to `main`
- Creates version tags from `Cargo.toml`

### Changelog (`.github/workflows/changelog.yml`)
- Generates changelog from conventional commits
- Updates `CHANGELOG.md` on new releases
- Categorizes changes by type (features, fixes, etc.)

## Git Hooks

### Setup
Run the setup script to install git hooks:
```bash
./.githooks/setup.sh
```

### Available Hooks

#### Pre-commit (`.githooks/pre-commit`)
- Runs `cargo fmt --check`
- Runs `cargo clippy -- -D warnings`
- Runs `cargo test`
- Checks for potential security issues

#### Commit-msg (`.githooks/commit-msg`)
- Validates conventional commit format
- Checks commit message length
- Suggests imperative mood usage

#### Pre-push (`.githooks/pre-push`)
- Prevents direct pushes to `main` branch
- Runs full test suite
- Checks for merge conflict markers
- Warns about large changesets

### Bypassing Hooks
If needed, you can bypass hooks temporarily:
```bash
git commit --no-verify
git push --no-verify
```

## Branch Protection Rules

### Main Branch
- Require PR reviews (minimum 1)
- Require status checks to pass
- Require branches to be up to date
- Restrict direct pushes
- Require signed commits (recommended)

### Dev Branch
- Require status checks to pass
- Allow force pushes (for rebasing)
- Auto-delete head branches after merge

## Best Practices

### Commits
- Use conventional commit format
- Keep commits atomic and focused
- Write clear, descriptive commit messages
- Use imperative mood ("add feature" not "added feature")

### Branches
- Use descriptive branch names
- Prefix with type: `feature/`, `bugfix/`, `hotfix/`
- Keep branches short-lived
- Rebase feature branches on dev before merging

### Pull Requests
- Fill out the PR template completely
- Include tests for new features
- Update documentation when needed
- Request appropriate reviewers
- Address all review feedback

### Security
- Never commit secrets or API keys
- Use environment variables for configuration
- Run security audits regularly
- Keep dependencies updated

## Troubleshooting

### Common Issues

#### Hook Failures
If git hooks fail:
1. Check error messages carefully
2. Run the failing command manually
3. Fix issues and try again
4. Use `--no-verify` only as last resort

#### Merge Conflicts
```bash
# During merge conflict resolution
git checkout dev
git pull origin dev
git checkout your-feature-branch
git rebase dev
# Resolve conflicts
git add .
git rebase --continue
```

#### Failed CI
1. Check the workflow logs in GitHub Actions
2. Run the same commands locally
3. Fix issues and push updates
4. CI will re-run automatically

### Getting Help
- Check this documentation first
- Look at existing PRs for examples
- Ask team members for guidance
- Create an issue for workflow improvements

## Maintenance

### Regular Tasks
- Review and update dependencies monthly
- Check for security vulnerabilities
- Update workflow actions to latest versions
- Review and improve branch protection rules
- Archive old feature branches

### Monitoring
- Monitor workflow success rates
- Review release metrics
- Check for outdated dependencies
- Analyze commit patterns for improvements