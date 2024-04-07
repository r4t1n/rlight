# rlight

rlight is a CLI program to control backlight brightness. It uses the Linux backlight devices directly so no display server is needed.

## Features
- Percent based brightness controls
- Increment or decrement brightness with "+" and "_" operators
- List all backlight devices
- Set a default backlight device
- Set brightness per device

## Usage
```
$ rlight --help
```

```
Usage: rlight [OPTIONS] [BRIGHTNESS]

Arguments:
  [BRIGHTNESS]  [default: ]

Options:
  -d, --device <DEVICE>  [default: 0]
  -l, --list             
  -s, --set <SET>        [default: 0]
  -h, --help             Print help

```

### Read brightness
```
$ rlight
```

### Set brightness to 100%
```
$ rlight 100
```

### Increment brightness by 5%
```
$ rlight +5
```

### Decrement brightness by 5%
```
$ rlight _5
```

### Set default backlight device to 1
```
$ rlight --set 1
```

### Change brightness to 100% on device 2
```
$ rlight --device 2 100
```

## Installation
Make sure you have cargo installed to build from source, replace \<user> with the output of `whoami`. After adding youself to the `video` group you need to reboot.

```
sudo gpasswd -a <user> video
```

```
make
```

```
sudo make install
```
