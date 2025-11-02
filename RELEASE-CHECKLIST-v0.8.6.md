# v0.8.6 Release Checklist

**Status**: ✅ **READY TO PUSH AND RELEASE**

---

## ✅ Completed

- [x] Version updated in Cargo.toml (v0.8.6)
- [x] Version updated in main.rs (using `env!("CARGO_PKG_VERSION")`)
- [x] Binary rebuilt and verified (`parseltongue 0.8.6`)
- [x] All 8 commands tested and working
- [x] Documentation updated (README.md, PRDv2.md)
- [x] Release notes created (RELEASE-NOTES-v0.8.6.md)
- [x] Test artifacts preserved (demo-walkthroughs/v0.8.6-release-testing/)
- [x] All changes committed (25 files)
- [x] Git tag created (v0.8.6)

---

## 🚀 Next Steps (Execute These Commands)

### Step 1: Push to GitHub

```bash
# Push the commit
git push origin main

# Push the tag
git push origin v0.8.6
```

### Step 2: Create GitHub Release

**Option A: Using GitHub CLI (gh)**

```bash
gh release create v0.8.6 \
  --title "v0.8.6: Real CozoDB Integration" \
  --notes-file RELEASE-NOTES-v0.8.6.md \
  ./target/release/parseltongue
```

**Option B: Manual via GitHub Website**

1. Go to https://github.com/that-in-rust/parseltongue/releases/new
2. Select tag: `v0.8.6`
3. Release title: `v0.8.6: Real CozoDB Integration`
4. Copy content from `RELEASE-NOTES-v0.8.6.md` into description
5. Upload binary: `./target/release/parseltongue`
6. Click "Publish release"

---

## Binary Details

**File**: `./target/release/parseltongue`
**Size**: 26MB
**Version**: 0.8.6 (verified with `--version`)
**Platform**: macOS (build platform)

**Rename for release**:
```bash
# For Apple Silicon
cp ./target/release/parseltongue ./parseltongue-macos-arm64

# For x86_64 (if cross-compiled)
# cp ./target/x86_64-apple-darwin/release/parseltongue ./parseltongue-macos-x86_64
```

---

## Verification After Release

After creating the release, verify:

```bash
# Check the release exists
gh release view v0.8.6

# Download and test the binary
curl -L https://github.com/that-in-rust/parseltongue/releases/download/v0.8.6/parseltongue-macos-arm64 -o parseltongue-test
chmod +x parseltongue-test
./parseltongue-test --version
# Should output: parseltongue 0.8.6
```

---

## Release Announcement (Optional)

Post on relevant channels:

### GitHub Discussions / README

```markdown
🎉 **Parseltongue v0.8.6 Released!**

PT02 now works with real CozoDB!

✅ All 8 commands verified working
✅ Export dependency graphs (~5K tokens)
✅ Export entities with ISG (~30K tokens)
✅ Export with type system (~60K tokens)
✅ Total pipeline: <2 seconds

Download: https://github.com/that-in-rust/parseltongue/releases/tag/v0.8.6
```

### Twitter/Social Media

```
🚀 Parseltongue v0.8.6 is out!

LLM-friendly code analysis with progressive disclosure:
📊 Level 0: Dependency edges (2-5K tokens)
📦 Level 1: Entities + ISG (30K tokens)
🔬 Level 2: + Type system (60K tokens)

All working with real CozoDB now!

#Rust #LLM #CodeAnalysis
```

---

## Rollback Plan (If Needed)

If issues are discovered:

```bash
# Delete the tag locally
git tag -d v0.8.6

# Delete the tag remotely
git push origin :refs/tags/v0.8.6

# Delete the GitHub release
gh release delete v0.8.6

# Revert the commit
git revert 09e2ba2
```

---

## Success Criteria

Release is successful when:
- [ ] `git push origin main` completes
- [ ] `git push origin v0.8.6` completes
- [ ] GitHub release v0.8.6 exists
- [ ] Binary is downloadable from release page
- [ ] Downloaded binary reports version 0.8.6
- [ ] All 8 commands work on the downloaded binary

---

**Ready to execute!** 🚀
