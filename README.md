Literate git clone.

## Usage

↓ make project.toml (file name is example)
```toml
[config]
host = "github"
dest_dir = "~/Project"
repos = [
          "emacs-mirror/emacs",
          "rust-lang/rust"
        ]
```

run
```sh
$ cloner ./project.toml
```

result
```
~                                                  ```
 └─ Project
     └─ emacs
     └─ rust
```

## TODO

- hosting service options
- `dest_dir` accept relative path
- clone way options(ssh or https)
- multiple groups
