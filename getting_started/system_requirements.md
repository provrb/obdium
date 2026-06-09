---
layout: page
title: System Requirements
parent: Getting Started
nav_order: 1
---

# System Requirements

OBDium is a lightweight desktop-only application built on top of [Tauri](https://v2.tauri.app/start/) and thus requires very minimal processing power. It supports Windows, macOS, and Linux, and can be compiled from source on all supported platforms.

The following requirements serve as a general rule of thumb for OBDium on all operating systems:

| Component                  | Minimum                                                                 | Recommended |
|:---------------------------|:------------------------------------------------------------------------|:-----------------------------|
| CPU                        | Dual-core x64 CPU                                                       | Quad-core x64 CPU or better  |
| Memory                     | 512 MB RAM                                                              | 4 GB RAM or more             |
| Storage                    | 1.5 GB available space (~15MB for OBDium, ~1.4GB for local databases)   | 2 GB available space         |

## Operating Systems

OBDium is currently supported on the following operating systems. Note that OBDium has been tested on real vehicles using Windows 10 and 11, and Arch Linux (KDE Plasma). Other environments are considered experimental and may vary in stability.

| Operating System           | Status                                                                 
|:---------------------------|:--------------------------------------------------------------------------|
| Windows 11 (64-bit)        | Supported - Tested                                                        |
| Windows 10 (64-bit)        | Supported - Tested                                                        |
| Linux                      | Supported - Tested - **May be experimental on some distributions**        |
| macOS                      | Supported - Experimental                                                  |

## Storage

You may notice that OBDium requires significantly more storage for offline databases then for the binary itself. 

OBDium ships with a compressed [xz](https://en.wikipedia.org/wiki/XZ_Utils) archive containing an [SQLite](https://en.wikipedia.org/wiki/SQLite) database file derived from the [National Highway Traffic Safety Administration's](https://www.nhtsa.gov/) Vehicle Product Information Catalog dataset. During the first launch of the application, this database is extracted and used locally to retrieve model-specific information when decoding Vehicle Identification Numbers in the VIN Decoder section of the application.

## Next Steps
- If you are ready to install OBDium onto your device, navigate to [Installing OBDium](installing.html).