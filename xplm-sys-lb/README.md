# xplm-sys-lb: Lee Baker's Rust bindings for the X-Plane SDK

Rust bindings to the X-Plane plugin SDK.

This crate contains the XPLM SDK from Laminar, with minor modifications that produce a more Rust-like interface.

## SDK versioning

In order to support previous versions of X-Plane, crate features are used to select which API to compile against:

* `xplm4`: Support the latest version of X-Plane SDK v4 (4.2.0), supporting X-Plane 12 features (includes `xplm3` and `xplm2`).
* `xplm3`: Support the latest version of X-Plane SDK v3 (3.0.3), supporting X-Plane 11 features (includes `xplm2`).
* `xplm2`: Support the latest version of X-Plane SDK v2 (2.1.0), supporting X-Plane 10 features.
* `deprecated`: Include SDK features deprecated by Laminar (corresponds to defining `XPLM_DEPRECATED`).

For more details on what's included in each feature, the actual compiler flags are in `build.rs` along with some information about which X-Plane versions they correspond to.

As new versions of the SDK come out, I'll update this crate with support.

## Changes to the XPLM SDK

There are a number of changes to the XPLM SDK to fix bugs, or better support Rust interfaces:

* Enums have generally been converted from using C `int` types to using C `enum` so that `bindgen`'s `rustified_enum()` can be used to generate compatible Rust enums.
* Some SDK functions don't have the `#ifdef`s to compile for older versions of X-Plane. An incomplete list:
  * `XPLMCreateInstance()` has no XPLM `#ifdef`, but does use a `XPLMObjectRef` parameter which needs SDK 2.0. I've added some.
  * `XPLMInstanceSetPosition()` has no XPLM `#ifdef`, but does use a `XPLMDrawInfo_t` parameter which needs SDK 2.0. I've added some.

I expect to continue making similar trivial changes to the header files that don't break the C interface.

## Plugins built on this SDK

This SDK is used to produce my X-Plane plugins, including [DataRefTool v2](https://datareftool.com).

## Links

* Laminar SDK [documentation](https://developer.x-plane.com/sdk/plugin-sdk-documents/) and [downloads](https://developer.x-plane.com/sdk/plugin-sdk-downloads/)

## Similar projects

I've intentionally put my initials in the name of this crate (`xplm-sys-lb`) to distinguish it from several other very similar crates which are unfortunately out of date:

* Sam Crow's original SDK bindings crate [xplm-sys](https://crates.io/crates/xplm-sys), which inspired this crate. At time of writing, the crate was last updated in Nov 2024 for SDK 4.1.0.
* `mkreu`/`judemille`'s fork [xplm-sys](https://github.com/judemille/xplane-sys). At time of writing, the crate was last updated in Feb 2024 for SDK 4.0.1.

## License

Code for the bindings (ie. everything not in SDK/) is dual licensed under the same license as Rust itself, specifically:

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

The X-Plane SDK is licensed its own license. See [SDK/license.txt](SDK/license.txt) for details.
