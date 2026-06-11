![AppImage](/docs/banner2.jpg)


<div align="center">
  <h1>OBDium</h1>
  <b>Your open-source app for everything car diagnostics.</b>
  <p><em>A Rust-based vehicle diagnostics tool designed to connect with ELM327 adapters, offering live OBD-II data, fault code analysis, and offline VIN decoding.</p></em>
  
  <img src="https://img.shields.io/badge/version-1.5.1-blue" alt="Version">
  <img src="https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-blue" alt="Platform">
  <img src="https://img.shields.io/badge/license-GPL--3.0-purple" alt="License">
  <img src="https://img.shields.io/github/downloads/provrb/obdium/total?color=brightgreen" alt="Total Downloads">

   <a href="#about">About</a>
   ·
   <a href="https://github.com/provrb/obdium/releases">Download</a>
   ·
   <a href="#quick-start">Quick Start</a>
   ·
   <a href="https://provrb.github.io/obdium/">User Manual</a>
   ·
   <a href="https://ko-fi.com/provrb">Sponsor</a>

</div>

<details>
   <summary><b>Click to view app screenshots!</b></summary>

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
  
</details>

## Table of Contents

- [About](#about)
- [Features](#features)
- [Download](#download)
- [Quick Start](#quick-start) 
- [Build from Source](#build-from-source)
- [Implementation & Logic](#implementation-and-logic)
- [Contributing](#contributing)
- [License](#license)

## About

**OBDium** is a fast, modern and extensible Rust-based diagnostic tool for interfacing with OBD-II systems via ELM327 serial adapters fully offline. It provides live access to vehicle sensor data, in-depth diagnostics, and accurate VIN decoding without relying on external crates for critical parsing logic.

The goal with OBDium is to fill a gap in the ecosystem, providing the best free, open-source, and easy-to-use vehicle diagnostics tool.

## Features

- **⚠️ View Trouble Codes:** Read diagnostic trouble codes, including **Powertrain, Body, Chassis,** and **Network** alongside a description
- **🧠 Live Vehicle Metrics:** Reads and decodes various OBD-II PIDs (engine, fuel, air, exhaust, diagnostics, etc.) with plans for manufacturer specific PIDs soon
- **🔎 Advanced VIN Decoding:** In-depth VIN decoding using a custom parser and SQLite-backed lookups based off of the NHTSA's VPIC MSSQL database
- **🧪 I/M Readiness Tests:** Verify that your car's emissions systems are functioning properly for both compression and spark ignition engines
- **📱 Modern GUI:** No more ugly and outdated native applications. Developed with modern web development technologies using Tauri with JS/HTML/CSS
- **🖥️ Cross-platform:** Available on macOS, Windows 10 and 11, and Linux

## Download
[You can download the latest release here.](https://github.com/provrb/obdium/releases)

## Quick Start
For a full user manual, view the docs [here.](https://provrb.github.io/obdium/)

### Connecting to a vehicle

1. Run the application
2. Connect your ELM327 adapter to your vehicles OBD-II port and device
3. Turn on your ignition or start your vehicle
4. Navigate to the **Connection** panel:
   - Select your serial port. If no serial ports appear, you can click the refresh button to reload and rediscover serial ports
   - Optionally change or leave the `OBD-II Protocol` and `Baud Rate` settings as is
5. Click **Connect** and wait for the notification telling you the connection was established

### Understanding each feature

- OBD data will be recorded in the **OBD Dashboard**
- View diagnostic trouble codes in the **DTC** panel.
- View graphs for live data in the **Graphs** panel.
- Decode a VIN to receive model-specific information in the **VIN Decoding** panel.
- To stop tracking a metric, click on the card it's being displayed in, in the **OBD Dashboard**
  - Resume tracking by clicking on it again at the very bottom of the dashboard.
- Modify preferences like units, privacy, or startup settings in the **Settings** panel.
- View an index of all PIDs in the **PID List** panel.
- Add a custom PID to track in the **PID List** panel.

### Demo mode
Demo mode is a unique feature that replays requests and simulates receiving a vehicle's live OBD-II data completely locally by reading an OBDium `requests.json` file. 

You can record your own requests when connected to an actual vehicle by navigating to the **Settings** panel and toggling the **Record OBD Responses** option. By default, the file containing your recorded responses will be in a subfolder called `data` in the directory the app is ran in. You can change this by clicking the **Choose a File** button below.

To replay already recorded requests simply:
1. Download an existing recorded requests file. You can use another OBDium users requests file or use the one [here](https://github.com/provrb/obdium/blob/main/backend/data/requests.json)
   - You can also share your vehicles recorded requests for people from the community to use!
2. Drag and drop the file into your data directory
   - This allows you to use your own recorded requests from your vehicle at a later time without having to be hooked up
3. Navigate to the **Connection** panel
4. Select "DEMO MODE" under the serial port dropdown
5. Click 'Connect'

Data will start popping up and simulate a real vehicle.

## Build from Source
Instructions on how to download the source code and build the application manually. 

<details>
   <summary><b>Command-Line Installation</b></summary>


   1. **Install Rust**  
      Download and install from [rust-lang.org](https://www.rust-lang.org/tools/install).

   2. **Install Tauri**

      ```sh
      cargo install tauri-cli --version "^2.0.0"
      ```

   3. **Clone the repository**

      ```sh
      git clone https://github.com/provrb/obdium.git
      cd obdium
      ```

   With this you can:

   1. **Build the project**

      ```sh
      cargo tauri build
      ```

   2. **Find the application**:

      - The application binary will be located in: `backend/target/release`
      - Windows' MSI and NSIS installers will be located in: `backend/target/release/bundle`
      - Linux bundles will be located in:
         - `backend/target/release/bundle/deb`
         - `backend/target/release/bundle/rpm`
         - `backend/target/release/bundle/appimage`

   OR

   1. **Run the project in developer mode**

      ```sh
      cargo tauri dev --no-watch
      ```

</details>

## Implementation and Logic

This project required extensive research into concepts like ELM327, the OBD-II protocol, and response decoding. Below is a brief explanation of the implementation and logic behind OBDium.


<details>
   <summary><b>Implementation Logic</b></summary>

   1. **All real-time vehicle communication** is done through the [OBD](backend/src/obd.rs) struct. To initiate communication, OBDium establishes a serial port connection to the ELM327 adapter. Vehicle data is requested using PIDs (Parameter IDs) and service numbers. An example request to get the Engine Coolant Temperature would look as such: `0105`. A full list of standard OBD-II PIDs can be found [here](https://en.wikipedia.org/wiki/OBD-II_PIDs).

   2. **Responses** from the vehicle are returned as hexadecimal-encoded strings. In general, responses contain particular bytes: 'A' refers to the byte at index 0, which spans to 'E' at byte index 4. Finally, a specific equation is used alongside these special bytes (A, B, C, D, E) to calculate the expected result. See the implementation of the response logic at ['src/response.rs'](backend/src/response.rs). For Engine Coolant Temperature it would be `A - 40`.

   3. For the **VIN parsing implementation**, I spent several hours reverse engineering the [National Highway Traffic Safety Administration’s Product Information Catalog Vehicle Listing (vPIC) MSSQL implementation](https://vpic.nhtsa.dot.gov/api/) to work with SQLite and Rust. Now, from just a simple VIN, the exact, make, model and year of that vehicle can be decoded, providing specific details like airbag locations, the number of engine cylinders, or if the vehicle comes equipped with traction control.

   For any questions about the implementation or logic behind OBDium, feel free to create a Discussion or open an Issue!
</details>


## Contributing
Please view the [CONTRIBUTING](CONTRIBUTING.md) file for more information.

## License
This project is licensed under GPL-3.0. See [`LICENSE`](LICENSE) for details.
