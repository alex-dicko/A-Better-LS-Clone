# A simple clone of the "ls" command with custom functionality

## Usage 
```
alexdickens@Alexs-MacBook-Pro files % als
Max Depth: 1 
Excluding Files/Folders: []
 
Cargo.toml
.DS_Store
target/
Cargo.lock
.git/
src/
```

```
alexdickens@Alexs-MacBook-Pro files % als --max 2
Max Depth: 2 
Excluding Files/Folders: []
 
Cargo.toml
.DS_Store
target/
  .DS_Store
  .rustc_info.json
  CACHEDIR.TAG
  release/
  debug/
Cargo.lock
.git/
  config
  objects/
  HEAD
  info/
  logs/
  description
  hooks/
  refs/
  index
  COMMIT_EDITMSG
src/
  main.rs
```

```
alexdickens@Alexs-MacBook-Pro files % als --max 2 --exclude src target
Max Depth: 2 
Excluding Files/Folders: ["src", "target"]
 
Cargo.toml
.DS_Store
Cargo.lock
.git/
  config
  objects/
  HEAD
  info/
  logs/
  description
  hooks/
  refs/
  index
  COMMIT_EDITMSG
```
