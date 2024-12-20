# Getting started with Picoprobe

![stlink v2 photo](https://raw.githubusercontent.com/viktorchvatal/rpi-pico-rust-assets/master/picoprobe/rpi-picoprobe.jpg)

USB loader is quite inconvenient because for every firmware update, it is
needed to disconnect the USB, push the BOOTSEL putton, connect the USB and
copy the firmware.

Picoprobe software enables one Rpi Pico to be used as a flasher/debugger for
another Rpi Pico (so buy at least two pieces).

## Build `openocd` version that can handle pico

Prerequisities
```
sudo apt install automake autoconf build-essential texinfo libtool libftdi-dev libusb-1.0-0-dev
```

```
git clone https://github.com/raspberrypi/openocd.git --branch rp2040 --depth=1 --no-single-branch
cd openocd
./bootstrap
./configure --enable-picoprobe
make -j10
```

If you wish to install (not needed)
```
sudo make install
```

## Download and flash the Picoprobe firmware

Firmware can be downloaded from here: https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html#debugging-using-another-raspberry-pi-pico

After downloading, connect the Rpi PICO in USB mass storage mode (BOOTSEL button active)
and copy the downloaded `picoprobe.uf2` to the `RPI-RP2` device

## Enable libusb access to Picoprobe

Connect picoprobe device to the usb and check that it is connected

```
$ lsusb | grep 2e8a:0004
Bus 001 Device 112: ID 2e8a:0004
```

Edit USB acces rules

Open `sudo vi /etc/udev/rules.d/99-openocd.rules` and insert
```
# Raspberry Pi Picoprobe
ATTRS{idVendor}=="2e8a", ATTRS{idProduct}=="0004", MODE:="0666"
```

Apply the rules

```
$ sudo udevadm control --reload-rules
$ sudo udevadm trigger
```

## Run openocd

```
cd ~/openocd
src/openocd -f interface/picoprobe.cfg -f target/rp2040.cfg -s tcl
```

If everything went well, the output should be similar to

```
Info : Hardware thread awareness created
Info : Hardware thread awareness created
Info : RP2040 Flash Bank Command
Info : Listening on port 6666 for tcl connections
Info : Listening on port 4444 for telnet connections
Info : clock speed 5000 kHz
Info : SWD DPIDR 0x0bc12477
Info : SWD DLPIDR 0x00000001
Info : SWD DPIDR 0x0bc12477
Info : SWD DLPIDR 0x10000001
Info : rp2040.core0: hardware has 4 breakpoints, 2 watchpoints
Info : rp2040.core1: hardware has 4 breakpoints, 2 watchpoints
Info : starting gdb server for rp2040.core0 on 3333
Info : Listening on port 3333 for gdb connections
```

## Running a program using openocd

1. Install `gdb-multiarch` if not installed

```
```

2. Open `.cargo/config` and set GDB as runner

```
runner = 'gdb-multiarch'
```

3. Create `.gdbinit` in the application crate directory and insert

```
target remote :3333

monitor arm semihosting enable

load
step
```

4. Run the application

```
cargo run --release
```

Note: In case of `warning: File ".../.gdbinit" auto-loading has been declined,
follow instructions to add `set auto-load safe-path /path/to/gdbinit`
into `~/.config/gdb/gdbinit`

## Notes

Programming using Picoprobe is somewhat unsure compared to ST Link for STM
devices. Program usually flashes and runs successfully on second try.

More information can be found in the official documentation here: https://datasheets.raspberrypi.com/pico/getting-started-with-pico.pdf