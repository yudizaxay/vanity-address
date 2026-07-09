## Summary

<!-- What does this PR do? One or two sentences. -->

## Type of change

- [ ] New blockchain / chain family
- [ ] Bug fix
- [ ] New feature
- [ ] Performance improvement
- [ ] Documentation only
- [ ] Refactor (no behavior change)

## Changes

<!-- Bullet list of main changes -->

-

## New chain checklist (if applicable)

- [ ] `ChainGrinder` implemented in `vanity-core/src/chains/`
- [ ] Registered in `mod.rs` (`Chain` enum, `from_id`, `MENU_CHAINS`, `dispatch!`)
- [ ] Address derivation verified against official spec / test wallet
- [ ] Pattern validation + at least one unit test
- [ ] README updated (chain ID, pattern rules)

## Testing

```bash
cargo fmt --all
cargo test
cargo clippy -- -D warnings
```

<!-- Describe manual testing, e.g. chain + pattern used -->

- [ ] Tests pass locally
- [ ] Tested interactive menu (if UI changed)
- [ ] Tested CLI flags (if CLI changed)

## Screenshots / output (optional)

<!-- Paste terminal output or screenshots for UX changes -->

## Notes for reviewers

<!-- Anything tricky: dependency choices, protocol edge cases, follow-ups -->

---

**Reminder:** Do not commit private keys, `vanity-results.txt`, or secrets.
