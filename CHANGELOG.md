# Changelog

All notable changes to this project are documented in this file.

## Overview

- [Changelog](#changelog)
  - [Overview](#overview)
  - [[0.2.1]](#021)

## [0.2.1]

- **Added optional `generator` feature**, which includes an ability to generate values using a `CfgValue`.
- **Added `.get()` and `.get_mut()` shortcut functions for `CfgValue`.** This is useful to skip the _as + unwrap_ step, which can become verbose.