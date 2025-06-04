<div align="center">
  <h1>OBDium</h1>
  <p><em>A Rust-based vehicle diagnostics tool designed to connect with ELM327 adapters, offering live OBD-II data, fault code analysis, and offline VIN decoding.</p></em>

  <img alt="last-commit" src="https://img.shields.io/github/last-commit/provrb/obdium?style=flat&logo=git&logoColor=white&color=0080ff">
  <img alt="repo-top-language" src="https://img.shields.io/github/languages/top/provrb/obdium?style=flat&color=0080ff">
  <img alt="repo-language-count" src="https://img.shields.io/github/languages/count/provrb/obdium?style=flat&color=0080ff">
</div>

![AppImage](/docs/banner2.jpg)

## Table of Contents

- [Overview](#overview)
- [Implementation](#implementation-and-logic)
- [Features](#features)
- [Usage](#usage)
- [Showcase](#showcase)
- [License](#license)
- [Contributing](#contributing)
- [Project Structure](#project-structure)

## Overview

**OBDium** ( pronounced Oh-Bid-ium ), is a fast, modern and extensible Rust-based diagnostic tool for interfacing with OBD-II systems via ELM327 serial adapters fully offline. It provides live access to vehicle sensor data, in-depth diagnostics, and accurate VIN decoding without relying on external crates for critical parsing logic.

Our goal with OBDium is to fill a gap in the ecosystem, providing the best free, open-source, and easy-to-use vehicle diagnostics tool.

## Features

- **⚠️ View Troube Codes:** Read diagnostic trouble codes, including **Powertrain, Body, Chassis,** and **Network** alongside a description
- **🧠 Live Vehicle Metrics:** Reads and decodes various OBD-II PIDs (engine, fuel, air, exhaust, diagnostics, etc.) with plans for manufacturer specific PIDs soon
- **🔎 Advanced VIN Decoding:** In-depth VIN decoding using a custom parser and SQLite-backed lookups based off of the NHTSA's VPIC MSSQL database
- **🔌 Serial Communication:** Connects to ELM327 OBD-II adapters via serial port
- **📱 Modern GUI:** No more ugly and outdated native applications. Developed with modern web development technologies using Tauri with JS/HTML/CSS
- **🖥️ Cross-platform:** Available on any operating system include both Linux and Windows

## Implementation and Logic

This project required extensive research into concepts like ELM327, the OBD-II protocol, and response decoding. Below is a brief explanation of the implementation and logic behind OBDium.

1. **All real-time vehicle communication** is done through the [OBD](src/obd.rs) struct. To initiate communication, OBDium establishes a serial port connection to the ELM327 adapter. Vehicle data is requested using PIDs (Parameter IDs) and service numbers. An example request to get the Engine Coolant Temperature would look as such: `0105`. A full list of standard OBD-II PIDs can be found [here](https://en.wikipedia.org/wiki/OBD-II_PIDs).

2. **Responses** from the vehicle are returned as hexadecimal-encoded strings. In general, responses contain particular bytes: 'A' refers to the byte at index 0, which spans to 'E' at byte index 4. Finally, a specific equation is used alongside these special bytes (A, B, C, D, E) to calculate the expected result. See the implementation of the response logic at ['src/response.rs'](src/response.rs). For Engine Coolant Temperature it would be `A - 40`.

3. For the **VIN parsing implementation**, I spent several hours reverse engineering the [National Highway Traffic Safety Administration’s Product Information Catalog Vehicle Listing (vPIC) MSSQL implementation](https://vpic.nhtsa.dot.gov/api/) to work with SQLite and Rust. Now, from just a simple VIN, the exact, make, model and year of that vehicle can be decoded, providing specific details like airbag locations, the number of engine cylinders, or if the vehicle comes equipped with traction control.

For any questions about the implementation or logic behind OBDium, feel free to create a Discussion or open an Issue!

## Installation

### Installer

1. Head to the GitHub releases page [here](https://github.com/provrb/obdium/releases)
2. Download the latest release for your operating system
3. Extract and run the installer.

### Command-Line

1. **Install Rust**  
   Download and install from [rust-lang.org](https://www.rust-lang.org/tools/install).

2. **Install Tauri**

   ```sh
   cargo install tauri-cli --version 1.6.5
   ```

3. **Clone the repository**

   ```sh
   git clone https://github.com/provrb/obdium.git
   cd obdium
   ```

4. Download SQLite Databases with Git LFS
  
   ```sh
   git lfs fetch
   ```

5. **Build the project**

   ```sh
   cargo tauri build
   ```

6. **Find the application**:

   - The file built will be located in: `backend/target/release`
   - MSI and NSIS installers will be located in: `backend/target/release/bundle`

## Usage

### Demo mode
To replay already recorded requests simply:
1. Navigate to the **Connection** panel
2. Select "DEMO MODE" under the serial port dropdown
3. Click 'Connect'

Data will start popping up and simulate a real vehicle.

### Connecting

1. Run the application
2. Connect your ELM327 adapter to your vehicles OBD-II port and device.
3. Navigate to the **Connection** panel:
   - Select the OBD-II protocol, serial port, and baud rate to use.
   - If no serial ports appear, you can click the refresh button to reload serial ports.
4. Click 'Connect'

### Features

- OBD data will be recorded in the **OBD Dashboard**
- View diagnostic trouble codes in the **DTC** panel.
- View graphs for live data in the **Graphs** panel.
- Decode a VIN to receive model-specific information in the **VIN Decoding** panel.
- To stop tracking a metric, click on the card it's being displayed in, in the **OBD Dashboard**
  - Resume tracking by clicking on it again at the very bottom of the dashboard.
- Modify preferences like units, privacy, or startup settings in the **Settings** panel.
- View an index of all PIDs in the **PID List** panel.

## Showcase

![connect-screen](/examples/connect-screen.png)
<p align="center"><em>Connection Screen - Not Connected</em></p>

![connected-screen](/examples/connected-screen.png)
<p align="center"><em>Connection Screen - Connected</em></p>

![obd-overview](/examples/obd-overview-screen.png)
<p align="center"><em>OBD Overview Screen</em></p>

![readiness-tests](/examples/readiness-tests-screen.png)
<p align="center"><em>I/M Readiness Test Status</em></p>

![supported-pids](/examples/supported-pids-screen.png)
<p align="center"><em>A List of Supported Pids</em></p>

![vin-decoding](/examples/vin-decoding-screen.png)
<p align="center"><em>OBDium VIN Decoder</em></p>

![settings](/examples/settings-screen.png)
<p align="center"><em>Preferences Screen - With Freeze Frame Active</em></p>

## Contributing

Contributions are welcome! Please open issues or submit pull requests for bug fixes, new features, or improvements.  
To contribute:

- Fork the repository
- Create a new branch for your feature or fix
- Make your changes and add tests as appropriate
- Submit a pull request describing your changes

## Project Structure

### Backend

- [`backend/`](/backend/): Backend logic include calculations and core functionality of the app
- [`backend/src/vin/`](/backend/src/vin/mod.rs): VIN decoding and database integration
- [`backend/src/obd.rs`](/backend/src/obd.rs): Main OBD interface
- [`backend/src/bridge`](/backend/src/bridge/): Backend interface for the frontend

### Frontend
- [`frontend/`](/frontend/): Main GUI logic

## License
This code has minimal restrictions, such that any distributions are made free-of-cost. See [`LICENSE`](LICENSE) for details.