# corrOSion

## Run
Make sure you have installed all dependencies. Then you can simply run
```
make run
```



#### Dependencies

You need to have `nasm`, `grub-mkrescue`, `xorriso`, `qemu`, and a nightly Rust compiler installed. Then you can run it using `make run`.

On Linux, Debian like distributions use `apt`:
```
apt install nasm grub-common xorriso qemu-system-x86
```

Install Rust nightly. __Warning: `curl ... | sh` is insecure__, use it in some virtual machine or chroot.

```
curl https://sh.rustup.rs -sSf | sh
rustup override add nightly
```
