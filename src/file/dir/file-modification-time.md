## File names that have been modified in the last 24 hours

[![std-badge]][std] [![cat-filesystem-badge]][cat-filesystem]

Gets the current working directory by calling [`env::current_dir`],
then for each entries in [`fs::read_dir`], extracts the
[`DirEntry::path`] and gets the metadata via [`fs::Metadata`]. The
[`Metadata::modified`] returns the [`SystemTime::elapsed`] time since
last modification. [`Duration::as_secs`] converts the time to seconds and
compared with 24 hours (24 * 60 * 60 seconds). [`Metadata::is_file`] filters
out directories.

```rust
{{#include examples/file-modification-time.rs}}
```

[`DirEntry::path`]: https://doc.rust-lang.org/std/fs/struct.DirEntry.html#method.path
[`Duration::as_secs`]: https://doc.rust-lang.org/std/time/struct.Duration.html#method.as_secs
[`env::current_dir`]: https://doc.rust-lang.org/std/env/fn.current_dir.html
[`fs::Metadata`]: https://doc.rust-lang.org/std/fs/struct.Metadata.html
[`fs::read_dir`]: https://doc.rust-lang.org/std/fs/fn.read_dir.html
[`Metadata::is_file`]: https://doc.rust-lang.org/std/fs/struct.Metadata.html#method.is_file
[`Metadata::modified`]: https://doc.rust-lang.org/std/fs/struct.Metadata.html#method.modified
[`SystemTime::elapsed`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html#method.elapsed
