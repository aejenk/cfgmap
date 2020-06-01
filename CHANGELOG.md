# Changelog

All notable changes to this project are documented in this file.

## Overview

- [Changelog](#changelog)
  - [Overview](#overview)
  - [[0.4.0]](#040)
  - [[0.3.0]](#030)
  - [[0.2.1]](#021)

## [0.4.0]

_2020.06.01_

- **Added support for YAML**

## [0.3.0]

_2020.03.03_

- **Added `value` and `array` macros**, to facilitate the creation of new `CfgValue`s and `CfgValue::List`s.
- **Added `preamble` submodule**, to make importing all common features of this crate less verbose.

## [0.2.1]

_2020.02.03_

- **Added optional `generator` feature**, which includes an ability to generate values using a `CfgValue`.
- **Added `.get()` and `.get_mut()` shortcut functions for `CfgValue`.** This is useful to skip the _as + unwrap_ step, which can become verbose.