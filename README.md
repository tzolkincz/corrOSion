# corrOSion

## Run
Make sure you have installed all dependencies. Then you can simply run
```
make run
```



## Dependencies

You need to have `nasm`, `grub-mkrescue`, `xorriso`, `qemu`, and a nightly Rust compiler installed.

On Linux, Debian like distributions use `apt`:
```
apt install nasm grub-common xorriso qemu-system-x86
```

Install Rust nightly. __Warning: `curl ... | sh` is insecure__, use it in some virtual machine or chroot.

```
curl -f -L https://static.rust-lang.org/rustup.sh -O
sh rustup.sh --channel=nightly
```

#### Ubuntu specific dependencies
```apt-get install grub-pc-bin```
