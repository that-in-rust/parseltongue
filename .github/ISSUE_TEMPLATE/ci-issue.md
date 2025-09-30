---
name: CI/Build Issue
about: Report issues with continuous integration or build process
title: '[CI] '
labels: ['ci', 'build']
assignees: ''
---

## CI/Build Issue Description

**What happened?**
A clear description of the CI or build issue.

**Which CI job failed?**
- [ ] Test Suite
- [ ] Build Matrix (specify OS: Linux/macOS/Windows)
- [ ] Security Audit
- [ ] Other: ___________

**Error Details**
```
Paste the relevant error output here
```

**Environment**
- **OS**: [e.g., Ubuntu 20.04, macOS 12, Windows 11]
- **Rust Version**: [e.g., 1.70.0]
- **Branch**: [e.g., main, develop, feature/xyz]

**Steps to Reproduce Locally**
1. Run `./scripts/ci-check.sh`
2. Or run specific command: `cargo build --all-features`
3. See error

**Expected Behavior**
What should have happened instead.

**Additional Context**
- Is this a new issue or regression?
- Does it happen consistently or intermittently?
- Any recent changes that might be related?

**Checklist**
- [ ] I have run `./scripts/ci-check.sh` locally
- [ ] I have checked that my code is properly formatted (`cargo fmt`)
- [ ] I have run clippy checks (`cargo clippy --all-targets --all-features`)
- [ ] I have verified this isn't a known issue in existing tests