# Integration

The UI is started fullscreen, using the cage compositor for wayland.

These packages are required for the minimal operation: `cage`, `fontconfig`, `ttf-opensans` and `libxcb`.

**Note:** `ttf-opensans` is required as a default _ttf_ font. The control ui does not support a non-ttf default
sans-serif font.

The main systemd service is `cage@.service`, which starts the control UI. It is enabled for `tty1`.

**Note:** On the integrated front panel that connects to the raspberry header, a udev rule symlinks the makair
serial port to `/dev/makair`. If you do not use these boards, you have to change the cage unit to use another port (most probably /dev/ttyAMA0).

## Recording telemetry on a USB stick

There are additional support scripts for recording the telemetry data onto a USB stick.

The script `makair-restart`, triggered by _udevil_ on USB detection handles reconfiguring and restarting the UI to
log data onto the USB if necessary.

These packages are required for on-the-fly recording the telemetry the USB telemetry: `usdisks2` and `devmon`
