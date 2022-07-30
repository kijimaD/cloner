Literate git clone.

## Usage

↓ make project.toml (example file name)
```toml
[config]
host = "github"
dest_dir = "/home/user/Project" # WARN: need absolute path!
repos = [
          "emacs-mirror/emacs",
          "rust-lang/rust"
        ]
```

```sh
$ cloner ./project.toml
```

```
home
 └── user                                                  ```
      └── Project
          └── emacs
          └── rust
```
