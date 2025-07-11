const { listen, emit } = window.__TAURI__.event;
import {
  exportDtcs,
  addGraphDropdownOption,
  appendTerminalOutput,
  addNotification,
  listenExpandPID,
} from "./features.js";
import {
  saveConnectionConfig,
  updateUnitConversionDropdowns,
} from "./settings.js";

const connectionLabel = document.getElementById("connection-label");
const connectionIcon = document.getElementById("connection-icon");
const replayResponses = document.getElementById("replay-responses");
const recordResponses = document.getElementById("record-responses");
const connectButton = document.getElementById("btn-connect");
const disconnectButton = document.getElementById("btn-disconnect");
const demoStatus = document.getElementById("demo-status");

listen("update-card", (event) => {
  const cards = document.querySelectorAll(".card");
  const exists = Array.from(cards).some((card) => {
    return card.textContent.includes(event.payload.name);
  });

  // create card if it doesnt exist
  if (!exists) {
    const container = document.querySelector(".grid");
    if (!container) return;

    // create the card
    // has a header, value, and then unit
    const card = document.createElement("div");
    const h3 = document.createElement("h3");
    const valueDiv = document.createElement("div");
    const unitSpan = document.createElement("span");

    card.className = "card";
    valueDiv.className = "value";
    unitSpan.className = "unit";

    h3.textContent = event.payload.name;
    unitSpan.textContent =
      event.payload.unit.toLowerCase() === "no data" ? "" : event.payload.unit;

    // if the unit is no data meaning that scalar has 'no data'
    // (see backend docs), then display 'N/A' for value
    const valueText =
      event.payload.unit.toLowerCase() === "no data"
        ? "N/A"
        : event.payload.value.toString();

    // add value and unit to one div to align horizontally
    valueDiv.appendChild(document.createTextNode(valueText + " "));
    valueDiv.appendChild(unitSpan);

    // append elements to card
    card.appendChild(h3);
    card.appendChild(valueDiv);

    container.appendChild(card);

    return;
  }

  cards.forEach((card) => {
    // Don't update card data.
    if (card.classList.contains("dimmed")) {
      return;
    }

    const h3 = card.querySelector("h3");
    if (!h3) return;

    const title = h3.textContent?.toLowerCase();
    if (title == event.payload.name.toLowerCase()) {
      const valueElem = card.querySelector(".value");

      if (valueElem) {
        const textNode = Array.from(valueElem.childNodes).find(
          (node) => node.nodeType === Node.TEXT_NODE,
        );

        if (textNode) {
          textNode.textContent =
            event.payload.unit.toLowerCase() === "no data"
              ? "N/A "
              : event.payload.value.toString() + " ";

          const unitElem = valueElem.querySelector(".unit");
          unitElem.textContent =
            event.payload.unit.toLowerCase() === "no data"
              ? ""
              : event.payload.unit;
        }
      }
    }
  });

  // update any graphs that might be using this
  emit("update-graphs", {
    name: event.payload.name,
    value: event.payload.value,
    unit: event.payload.unit,
  });
});

listen("vehicle-details", (event) => {
  const vin = document.querySelector(".vin");
  const makeModel = document.querySelector(".car-model");

  if (!hideVin) {
    vin.textContent = "VIN: " + event.payload.vin.toUpperCase();
  } else {
    vin.textContent =
      "VIN: " + event.payload.vin.toUpperCase().slice(0, 6) + "*".repeat(12);
  }

  makeModel.textContent = (
    event.payload.make +
    " " +
    event.payload.model
  ).toUpperCase();
});

listen("connection-status", async (event) => {
  const protocolDropdown = document.getElementById("protocol-menu");
  const protocolSelected = document.getElementById("protocol-selected");
  const status = document.getElementById("connection-details");
  console.log(event);

  if (event.payload.connected) {
    if (window.connected) return;

    const serialPort = event.payload.serialPort;
    const baudRate = event.payload.baudRate;
    const protocol = event.payload.protocol;

    connectionLabel.textContent =
      "ELM327 CONNECTED VIA " + serialPort.toUpperCase();
    status.textContent =
      "CONNECTED THROUGH SERIAL PORT " + serialPort.toUpperCase();
    connectionIcon.src = "/assets/icons/connected.png";
    window.connected = true;

    // Features only accessible in real mode
    if (serialPort !== "DEMO MODE") {
      recordResponses.disabled = false;
      replayResponses.disabled = false;
    } else
      // show demo mode status
      demoStatus.style.display = "flex";

    connectButton.disabled = true;
    disconnectButton.disabled = false;

    document.getElementById("baud-rate-selected").textContent = baudRate;
    document.getElementById("serial-port-selected").textContent = serialPort;
    protocolSelected.dataset.value = protocol;
    protocolSelected.textContent = protocolDropdown.querySelector(
      `li[data-value="${protocol}"]`,
    ).textContent;

    window.connectionConfig = {
      serialPort: serialPort,
      baudRate: baudRate,
      protocol: parseInt(protocol),
    };

    addNotification("CONNECTED VIA ELM", event.payload.message);

    saveConnectionConfig();

    if (window.autoCheckCodes) {
      emit("get-dtcs");
    }

    if (window.unitPreferences) {
      emit("set-unit-preferences", window.unitPreferences);
      updateUnitConversionDropdowns();
    }

    if (serialPort === "DEMO MODE") {
      await new Promise((r) => setTimeout(r, 15000));
      emit("get-pids");
      await new Promise((r) => setTimeout(r, 2000));
      emit("get-readiness-tests");
    } else {
      await new Promise((r) => setTimeout(r, 7000));
      emit("get-pids");
      await new Promise((r) => setTimeout(r, 2000));
      emit("get-readiness-tests");
    }
  } else {
    if (!window.connected) return;

    connectionLabel.textContent = "ELM327 NOT CONNECTED";
    status.textContent = "NO CONNECTION ESTABLISHED";
    connectionIcon.src = "/assets/icons/not-connected.png";
    window.connected = false;
    recordResponses.disabled = true;
    replayResponses.disabled = true;
    connectButton.disabled = false;
    disconnectButton.disabled = true;

    addNotification("DISCONNECTED FROM ELM", event.payload.message);

    if (demoStatus.style.display !== "none") demoStatus.style.display = "none";
  }
});

listen("update-pids", (event) => {
  const pids = event.payload;
  const pidList = document.getElementById("pid-list");
  pidList.innerHTML = ``;

  for (const pidInfo of pids) {
    const pidGroup = document.createElement("div");
    pidGroup.className = "pid-group";
    pidGroup.innerHTML = `
        <div class="pid-container" ${!pidInfo.supported ? 'style="opacity: 0.15"' : ""} >
            <div class="info-row">
            <button class="arrow-icon"><img src="/assets/icons/arrow-icon.png"></button>
            <span class="name">${pidInfo.supported ? "[SUPPORTED]" : "[UNSUPPORTED]"}  ${pidInfo.pidName.toUpperCase()}</span>
            </div>
            <div class="pid-details" style="display: none; height: 0;">
            <div class="pid-data-columns">
                <div class="pid-column">
                <div class="pid-label">MODE</div>
                <div class="pid-value">$${pidInfo.mode}</div>
                </div>
                <div class="pid-column">
                <div class="pid-label">PID</div>
                <div class="pid-value">${pidInfo.pid}</div>
                </div>
                <div class="pid-column">
                <div class="pid-label">COMMAND</div>
                <div class="pid-value">${pidInfo.mode + pidInfo.pid}</div>
                </div>
                <div class="pid-column">
                <div class="pid-label">EQUATION</div>
                <div class="pid-value">${pidInfo.formula == "" ? "??" : pidInfo.formula}</div>
                </div>
                <div class="pid-column">
                <div class="pid-label">UNIT</div>
                <div class="pid-value">${pidInfo.unit == "" ? "??" : pidInfo.unit.toUpperCase()}</div>
                </div>
            </div>
            <button class="pid-button" id="remove-pid">REMOVE</button>
            </div>
        </div>
        `;

    pidList.appendChild(pidGroup);

    listenExpandPID(pidGroup);

    addGraphDropdownOption(
      pidInfo.pid,
      pidInfo.pidName,
      pidInfo.unit,
      pidInfo.formula,
    );
  }

  // Increment results counter
  const header = document.getElementById("pid-list-header");
  if (header.textContent != "VIEW PIDS (" + pidList.children.length + ")") {
    addNotification("PID INDEX", "Updated list of supported parameter ID's.");
  }

  header.textContent = "VIEW PIDS (" + pidList.children.length + ")";
});

const dtcList = document.getElementById("dtc-list");
const dtcHeader = document.getElementById("dtc-header");

listen("update-dtcs", (event) => {
  const dtcs = event.payload;
  if (!dtcs) {
    dtcHeader.textContent = "DIAGNOSTIC TROUBLE CODES (0)";
    return;
  }

  dtcHeader.textContent = "DIAGNOSTIC TROUBLE CODES (" + dtcs.length + ")";

  // clear dtc list
  dtcList.innerHTML = ``;

  for (const troubleCode of dtcs) {
    const description = troubleCode.permanant
      ? troubleCode.description + " [PERMANANT CODE]"
      : troubleCode.description + " [PENDING CODE]";

    let dtcRow = document.createElement("div");
    dtcRow.className = "info-row";
    dtcRow.style = "height: 60px; position: relative;";
    dtcRow.innerHTML = `
          <div class="category" id="dtc-category" style="font-size: 40px; font-weight: 900; margin-left: 6px; min-width: 70px; text-align: center; display: inline-block;">
              ${troubleCode.category}
          </div>
          <div class="name" style="display: inline-block; vertical-align: top;">
              <span class="name" id="dtc-name" style="font-size: 25px; font-weight: 700;">${troubleCode.name}</span>
              <div class="name" id="dtc-location" style="color: #BDBDBD; font-size: 15px; margin-top: -5px; font-weight: 600;">
              ${troubleCode.location}
              </div>
          </div>
          
          <svg xmlns="http://www.w3.org/2000/svg" width="40" height="30" viewBox="0 0 24 24" fill="none"
              stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"
              style="margin-left: 190px; position: absolute; top: 50%; transform: translateY(-50%);">
              <path d="M5 12h14" />
              <path d="m12 5 7 7-7 7" />
          </svg>
          
          <div class="name" id="dtc-description"
              style="position: absolute; color: #BDBDBD; left: 240px; top: 0; bottom: 0; max-width: 600px; overflow-wrap: break-word; display: flex; align-items: center; height: 100%;">
              ${description}
          </div>
          `;

    dtcList.appendChild(dtcRow);
  }

  if (window.autoSaveCodes) {
    exportDtcs(true);
  }
});

const menu = document.getElementById("serial-port-dropdown-menu");
const serialPortSelected = document.getElementById("serial-port-selected");

listen("update-serial-ports", (event) => {
  menu.innerHTML = "";

  const demoModePortOption = document.createElement("li");
  demoModePortOption.textContent = "DEMO MODE";
  demoModePortOption.dataset.value = "DEMO MODE";
  menu.appendChild(demoModePortOption);

  if (event.payload === "") {
    return;
  }

  const portOption = document.createElement("li");
  portOption.textContent = event.payload;
  portOption.dataset.value = event.payload;
  menu.appendChild(portOption);
});

const readinessTests = document.getElementById("readiness-tests-list");
listen("update-readiness-tests", (event) => {
  readinessTests.innerHTML = ``;

  addNotification("READINESS TESTS", "Updated I/M readiness test information.");

  const tests = event.payload;
  for (const test of tests) {
    const testRow = document.createElement("div");
    testRow.className = "info-row";
    testRow.style = "justify-content: space-between;";
    testRow.innerHTML = `
      <div class="name" id="test-name" style="flex: 2;">TEST: ${test.name.toUpperCase()}</div>
      <div class="name" id="test-availability" style="flex: 1;">${test.available ? "AVAILABLE" : "UNAVAILABLE"}</div>
      <div class="name" id="test-completeness" style="flex: 1;">${test.complete ? "COMPLETE" : "INCOMPLETE"}</div>
    `;

    readinessTests.appendChild(testRow);
  }
});

listen("update-command-output", (event) => {
  appendTerminalOutput(event.payload);
});

const exportButton = document.getElementById("vin-export");

// show vehicle info if no error
listen("decode-vin", (event) => {
  const data = event.payload;

  if (!data.error_msg) {
    return;
  }

  addNotification("VIN DECODING", data.error_msg);

  const container = document.getElementById("vin-container");
  if (!container) {
    return;
  }

  container.innerHTML = "";

  // show all fields
  for (const [key, value] of Object.entries(event.payload)) {
    if (key === "error_msg" || value == "N/A" || value == "-1") continue;

    const name = key.replaceAll("_", " ").toUpperCase();

    const card = document.createElement("div");
    const h3 = document.createElement("h3");
    const valueDiv = document.createElement("div");

    card.className = "card";
    valueDiv.className = "value";

    h3.textContent = name;
    valueDiv.textContent = value.toUpperCase();
    valueDiv.style.fontSize = "1.3rem";
    valueDiv.style.wordBreak = "break-word";
    valueDiv.style.whiteSpace = "normal";
    valueDiv.style.maxWidth = "320px";

    // append elements to card
    card.appendChild(h3);
    card.appendChild(valueDiv);

    container.appendChild(card);
  }

  exportButton.disabled = false;
});
