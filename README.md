<div align="center">
  <h1>OBDium</h1>
  <p><em>A Rust-based vehicle diagnostics tool designed to connect with ELM327 adapters, offering live OBD-II data, fault code analysis, and offline VIN decoding.</p></em>
  
  <img alt="tests" src="https://img.shields.io/github/actions/workflow/status/provrb/obdium/rust.yml?label=tests&style=flat&logo=github&color=0080ff">
  <img alt="build" src="https://img.shields.io/github/actions/workflow/status/provrb/obdium/rust.yml?label=build&style=flat&logo=github&color=0080ff)">
  <img alt="last-commit" src="https://img.shields.io/github/last-commit/provrb/obdium?style=flat&logo=git&logoColor=white&color=0080ff">
  <img alt="repo-top-language" src="https://img.shields.io/github/languages/top/provrb/obdium?style=flat&color=0080ff">
  <img alt="repo-language-count" src="https://img.shields.io/github/languages/count/provrb/obdium?style=flat&color=0080ff">
</div>

## Table of Contents
- [Overview](#overview)
- [Implementation](#implementation-and-logic)
- [Features](#features)
- [Usage](#example-usage)
- [License](#license)
- [Project Structure](#project-structure)
- [Roadmap](#roadmap)

## Overview

**OBDium** ( pronounced Oh-Bid-ium ), is a fast, extensible Rust-based diagnostic tool for interfacing with OBD-II systems via ELM327 serial adapters. It provides live access to vehicle sensor data, in-depth diagnostics, and accurate VIN decoding without relying on external crates for critical parsing logic. 

Our goal with OBDium is to fill a gap in the ecosystem, providing the best free, open-source, and easy-to-use vehicle diagnostics tool.

## Implementation and Logic
This project required extensive research into concepts like ELM327, the OBD-II protocol, and response decoding. Below is a brief explanation of the implementation and logic behind OBDium.

1. **All real-time vehicle communication** is done through the [OBD](src/obd.rs) struct. To initiate communication, OBDium establishes a serial port connection to the ELM327 adapter. Vehicle data is requested using PIDs (Parameter IDs) and service numbers. An example request to get the Engine Coolant Temperature would look as such: `0105`. A full list of standard OBD-II PIDs can be found [here](https://en.wikipedia.org/wiki/OBD-II_PIDs). 

2. **Responses** from the vehicle are returned as hexadecimal-encoded strings. In general, responses contain particular bytes: 'A' refers to the byte at index 0, which spans to 'E' at byte index 4. Finally, a specific equation is used alongside these special bytes (A, B, C, D, E) to calculate the expected result. See the implementation of the response logic at ['src/response.rs'](src/response.rs). For Engine Coolant Temperature it would be `A - 40`.

3. For the **VIN parsing implementation**, I spent several hours reverse engineering the [National Highway Traffic Safety Administration’s Product Information Catalog Vehicle Listing (vPIC) MSSQL implementation](https://vpic.nhtsa.dot.gov/api/) to work with SQLite and Rust. Now, from just a simple VIN, the exact, make, model and year of that vehicle can be decoded, providing specific details like airbag locations, the number of engine cylinders, or if the vehicle comes equipped with traction control.

For any questions about the implementation or logic behind OBDium, feel free to create a Discussion or open an Issue!

## Features

- **🔌 Serial Communication:** Connects to ELM327 OBD-II adapters via serial port
- **🧠 Live Vehicle Metrics:** Reads and decodes various OBD-II PIDs (engine, fuel, air, exhaust, diagnostics, etc.) with plans for manufacturer specific PIDs soon
- **🔎 Advanced VIN Decoding:** In-depth VIN decoding using a custom parser and SQLite-backed lookups based off of the NHTSA's VPIC MSSQL database
- **💾 SQLite-Backed Caching & Queries:** Persistent VIN metadata and decoded lookup results are stored locally for fast and offline operation
- **🧪 Unit Tests:** Comprehensive unit tests for VIN decoding and database access ([`tests/vin.rs`](tests/vin.rs))
- **⚙️ Error Handling and Resilience:** Gracefully handles common ELM327 quirks and serial errors with clear messages and fallbacks

## Example Usage

Connect your ELM327 adapter to your computer and note the serial port (e.g., `COM4` on Windows or `/dev/ttyUSB0` on Linux).

```sh
cargo run --release
```

Sample output:
```
Model year: 2018
WMI: "KL4"
WMI ID: 2069
Key: "CJASB|JB660929"

======================== VEHICLE MODEL INFO ========================
Manufacturer Country: ...
Manufacturer Name: ...
Manufacturer Region: ...
...

======================== DIAGNOSTICS ========================
Supported pids for ECUs
ECU 7E8:
    01 03 04 05 06 07 0A 0B 0C 0D ...
Check engine light: false
Number of trouble codes: 0
OBD standard: JOBD and OBD-II
Auxiliary input status: Inactive
...
```

## Installation

1. **Install Rust**  
   Download and install from [rust-lang.org](https://www.rust-lang.org/tools/install).

2. **Clone the repository**  
   ```sh
   git clone https://github.com/provrb/obdium.git
   cd obdium
   ```

   Ensure you correctly install the vpic.sqlite file from `/data/`.

3. **Build the project**  
   ```sh
   cargo build --release
   ```

4. **(Optional) Run tests**  
   ```sh
   cargo test
   ```

5. **Prepare SQLite databases**  
   Ensure the required SQLite databases are present in the `data/` directory (see [`src/vin/parser.rs`](src/vin/parser.rs) for expected paths). This will change to be more convenient in the future.

## Contributing

Contributions are welcome! Please open issues or submit pull requests for bug fixes, new features, or improvements.  
To contribute:

- Fork the repository
- Create a new branch for your feature or fix
- Make your changes and add tests as appropriate
- Submit a pull request describing your changes

## Project Structure

- [`src/main.rs`](src/main.rs): Entry point and CLI logic
- [`src/obd.rs`](src/obd.rs): OBD-II communication and protocol handling
- [`src/pid/`](src/pid/mod.rs): PID modules for various OBD-II data groups
- [`src/vin/`](src/vin/mod.rs): VIN decoding and database integration
- [`tests/vin.rs`](tests/vin.rs): Unit tests for VIN decoding and database access

## License

This code has minimal restrictions, such that any distributions are made free-of-cost. See [`LICENSE`](LICENSE) for details. 

## Roadmap
1. Finish VIN parsing functionality.
2. Fully functional user-interface with live graph view and the ability to send requests manually to the vehicle.

---

**Note:**  
- Ensure your user account has permission to access the serial port.
- The tool expects the SQLite databases to be present in the `data/` directory.
- For more details on extending PID or VIN decoding, see the module files in [`src/pid/`](src/pid/mod.rs) and [`src/vin/`](src/vin/mod.rs).