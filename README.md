# inside-chroot

Detect if code is running inside a [chroot environment](https://en.wikipedia.org/wiki/Chroot).

## How does it work

Retrieve inode for `/` if it is not `2` assume code runs inside chroot.

## API

```rust
use inside_chroot::inside_chroot;
let inside = inside_chroot().unwrap();
assert!(!inside) // may fail if tests are run inside a chroot env
```

## Example

### Outside chroot

```
cargo run --example inside-root
```

```
currently not inside chroot
```

### Inside a docker container (chroot)

```
# compiles inside-chroot example binary
cargo build --example inside-chroot
# start a docker container
docker run -it -v ${PWD}/target/debug/examples/:/examples archlinux:latest /examples/inside-chroot 
currently inside chroot 
```

## Credits

* https://stackoverflow.com/questions/75182/detecting-a-chroot-jail-from-within
* https://unix.stackexchange.com/questions/14345/how-do-i-tell-im-running-in-a-chroot

## License

Dual-licensed under MIT or the Apache License V2.0.
