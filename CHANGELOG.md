# Changelog

-----

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

***Added:***

- Strip symbols from release builds

## 0.4.0 - 2023-05-10

***Changed:***

- Rename `PYAPP_STARSHIP_PROMPT` option to `PYAPP_METADATA_TEMPLATE`

***Added:***

- Add `PYAPP_PIP_EXTRA_ARGS` option to provide extra `pip install` arguments
- Add `PYAPP_PIP_ALLOW_CONFIG` option to allow runtime configuration of `pip`
- Add configuration for correct cross compilation

## 0.3.1 - 2023-05-10

***Fixed:***

- Fix default distribution detection for Linux on architectures other than x86_64

## 0.3.0 - 2023-05-10

***Changed:***

- Rename `PYAPP_DISTRIBUTION_COMPRESSION` option to `PYAPP_DISTRIBUTION_FORMAT`

***Added:***

- Add `--pre` flag to the `self update` command to allow pre-release and development versions
- Add environment variable for detection
- Add `PYAPP_SELF_COMMAND` option to control the name of the management command
- Add `PYAPP_SKIP_INSTALL` option to skip project installation
- Remove dependence on OpenSSL

***Fixed:***

- Properly display error messages from `pip install` commands
- Fix project version reading for the metadata command on non-Windows systems

## 0.2.0 - 2023-05-07

This is the initial public release.
