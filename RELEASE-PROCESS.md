# Release Process

1. Change the version number in `README.md`.
2. Change the version number in `Cargo.toml`.
3. Commit and push the version number change.
4. Run `cargo test`
5. Run `cargo package`
6. Run `cargo publish.

## New Wordlist

For future reference, if I ever change the wordlist, the command to get the
stuff to copy and paste into `charsets.rs` is:

```
cat resources/wordlist.txt | sed 's/^/"/' | sed 's/$/",/'
```

Make sure to verify that the new wordlist doesn't contain duplicate words.
