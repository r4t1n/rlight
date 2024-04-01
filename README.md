# rlight

## Installation
Make sure you have cargo installed to build from source, replace \<user> with the output of `whoami`. After adding youself to the `video` group you need to reboot.

```
sudo gpasswd -a <user> video
```

```
cargo build --release
```

```
sudo ./install.sh
```