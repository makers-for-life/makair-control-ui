# MakAir Control UI

<p align="center">
  <img alt="Normal Respiratory Cycle" src="./doc/screens/normal-cycle.gif">
</p>

## Versions

| Version | Last Changelog | Ready? |
| ------- | -------------- | ------ |
| V1.1.x | Operational initial release | ✅
| V1.2.x | Crashes fixed + smoother graph + implemented medical staff feedbacks | ✅
| V1.3.x | Improve text readability, by increasing font sizes | ✅
| V1.4.x | Added internationalization capabilities | ✅
| V1.5.x | Improve UI readability + allow to change MakAir's settings | ✅

## How To Build?

In order to setup your environment and build the code, please follow the following commands (for MacOS):

1. Install [Rustup](https://rustup.rs/)
2. Ènsure you are using the Rust stable toolchain: `rustup default stable`
3. On Linux, make sure you have cmake installed and those libraries (debian):
  `libxcb-shape0 libxcb-shape0-dev libxcb-xfixes0 libxcb-xfixes0-dev libfontconfig libfontconfig1-dev`
4. Build the project: `cargo build`

Wayland support may be available but it hasn't been tested. You need a working X11 server.

_Tested at Rust version: `rustc 1.43.0 (4fb7144ed 2020-04-20)`_

## How To Run?

To run the Control UI, please ensure that your device first has an open serial connection with the motherboard.

_Note that the Control UI relies on the MakAir Telemetry library, which gets pulled from the [makair-telemetry](https://github.com/makers-for-life/makair-telemetry) repository._

Take note of the serial port used as an input on your Control UI board, and call:

1. Run: `./makair-control --port=0` (where `--port` is your serial port ID, maybe /dev/ttyUSB0)

You may also play a pre-recorded file, by passing it as an input (this is handy while developing; example records are available in the telemetry library repository):

1. Pull the telemetry library (in the parent directory): `git pull https://github.com/makers-for-life/makair-telemetry.git`
2. Run an example (from the Control UI directory): `./makair-control --input=../makair-telemetry/records/few_cycles` (where `--input` is an UNIX file path)

## Supported Translations

As MakAir was designed to be used internationally, we have made sure that the Control UI was fully translated in major languages.

You can pass the desired locale code when running the `makair-control` binary, using the `--translation` argument. _Make sure to pass the locale ISO code, eg. for French: `--translation=fr`._

**➡️ You can find the list of supported languages, with their ISO language codes below:**

* 🇬🇧 **[English](./res/locales/en.ftl)**: `en`
* 🇫🇷 **[French](./res/locales/fr.ftl)**: `fr`
* 🇩🇪 **[German](./res/locales/de.ftl)**: `de`
* 🇮🇹 **[Italian](./res/locales/it.ftl)**: `it`
* 🇪🇸 **[Spanish](./res/locales/es.ftl)**: `es`
* 🇵🇹 **[Portuguese](./res/locales/pt.ftl)**: `pt`
* 🇵🇱 **[Polish](./res/locales/pl.ftl)**: `pl` — (WIP)
* 🇨🇿 **[Czech](./res/locales/cs.ftl)**: `cs` — (WIP)
* 🇱🇻 **[Latvian](./res/locales/lv.ftl)**: `lv`
* 🇷🇺 **[Russian](./res/locales/ru.ftl)**: `ru`
* 🇺🇦 **[Ukrainian](./res/locales/uk.ftl)**: `uk`
* 🇹🇷 **[Turkish](./res/locales/tr.ftl)**: `tr` — (WIP)
* 🇦🇿 **[Azerbaijani](./res/locales/az.ftl)**: `az` — (WIP)
* 🇨🇳 **[Chinese (Simplified)](./res/locales/zh.ftl)**: `zh`
* 🇯🇵 **[Japanese](./res/locales/ja.ftl)**: `ja` — (WIP)
* 🇰🇷 **[Korean](./res/locales/ko.ftl)**: `ko` — (WIP)

ℹ️ If your language does not appear in the list above, you may translate the [base English file](./res/locales/en.ftl), then [open a Pull Request](https://github.com/makers-for-life/makair-control-ui/pulls).

## How To Build A Release? (Cross-Compile For ARM)

To cross-compile a new release for an ARM target (using MUSL; statically-linked libraries), you can call the `release_binaries.sh` script:

1. Run: `./scripts/release_binaries.sh --version=1.0.0`

_Make sure to replace the `version` script argument with the current release version. This is used for file naming purposes only._

**⚠️ Important note: this is still a work in progress. We are near cross-compiling capabilities, though some work still needs to be done due to non-MUSL C-based dependencies.**

## Optional Features

The Control UI behavior can be tuned at compile time, by enabling some optional features while building the binary:

* **Automatic graph scaler:**
  * Feature name: `graph-scaler`
  * Build command: `cargo build --features=graph-scaler`
  * Purpose: enables automatic graph maximum calculation, and re-adjustments as the target peak pressure is adjusted by an operator, or the largest point pressure value on screen overflows the graph. This overrides the default static maximum pressure value, that is based on the theoretical saturation maximum in an human subject. While the dynamic graph scaler system was previously the default, based on medical staff feedback it has been made static in order to improve machine usability in real-world scenarios. The auto-scaler feature was confusing for them, as traditional respirator systems use an hardcoded maximum.

## Troubleshooting

In case of an issue, please review the following points and check if any could help:

* If you are on Linux, you need to be a member of the `dialout` group for your user to be able to use the device created by the serial port;
