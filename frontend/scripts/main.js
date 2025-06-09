const { listen, emit } = window.__TAURI__.event;
const { save } = window.__TAURI__.dialog;
const { writeFile, removeFile } = window.__TAURI__.fs;
const { appWindow } = window.__TAURI__.window;

import {
  clearDtcs,
  exportDtcs,
  connectElm,
  disconnectElm,
  clearObdView,
  appendTerminalOutput,
} from "./features.js";

import { saveUnitPreference } from "./settings.js";

// ELM connection
// Changes when connection-status is fired
let connected = false;
window.connectionConfig = {
  serialPort: "0",
  baudRate: "0",
  protocol: 0,
};

// Personal settings
let hideVin = false;
let deleteLogsOnExit = false;
let autoCheckCodes = false;
let autoSaveCodes = false;
let logFilePath = "";

// UI Components
const dropdowns = document.querySelectorAll(".dropdown");
const graphDropdowns = document.querySelectorAll(".graph-dropdown");
const connectButton = document.getElementById("btn-connect");
const disconnectButton = document.getElementById("btn-disconnect");
const clearObdButton = document.getElementById("obd-clear");
const pauseObdButton = document.getElementById("obd-pause");
const dtcClearButton = document.getElementById("dtc-clear-button");

// When frontend gets loaded
// alert the backend with an event.
window.addEventListener("DOMContentLoaded", () => {
  emit("frontend-loaded");

  // load serial ports
  emit("get-serial-ports");

  emit("get-connection-status");
});

function handleDropdown(dropdown, toggleName, menuName) {
  const toggle = dropdown.querySelector(toggleName);
  const menu = dropdown.querySelector(menuName);

  toggle.addEventListener("click", (e) => {
    e.stopPropagation();
    if (menu.style.display === "block") {
      menu.style.display = "none";
    } else {
      document.querySelectorAll(menuName).forEach((m) => {
        m.style.display = "none";
      });
      menu.style.display = "block";
    }
  });

  menu.addEventListener("click", (e) => {
    if (e.target.tagName === "LI") {
      toggle.textContent = e.target.textContent;
      toggle.dataset.value = e.target.dataset.value;
      menu.style.display = "none";

      if (toggle.id == "unit-preference" && window.unitPreferences) {
        // change unit
        const unitType = toggle.getAttribute("data-target");
        const unit = toggle.dataset.value;

        saveUnitPreference(unitType, unit);
      }
    }
  });
}

clearObdButton.addEventListener("click", clearObdView);
pauseObdButton.addEventListener("click", () => {
  window.obdViewPaused = !window.obdViewPaused;
  if (obdViewPaused) {
    pauseObdButton.textContent = "RESUME";
  } else {
    pauseObdButton.textContent = "PAUSE";
  }
});

dropdowns.forEach((dropdown) => {
  handleDropdown(dropdown, ".dropdown-toggle", ".dropdown-menu");
});

graphDropdowns.forEach((dropdown) => {
  handleDropdown(dropdown, ".graph-dropdown-toggle", ".graph-dropdown-menu");
});

document.addEventListener("click", () => {
  document.querySelectorAll(".dropdown-menu").forEach((menu) => {
    menu.style.display = "none";
  });
});

connectButton.addEventListener("click", async () => {
  const baudRate = document.getElementById("baud-rate-selected");
  const serialPort = document.getElementById("serial-port-selected");
  const protocol = document.getElementById("protocol-selected");

  connectElm(
    baudRate.textContent.trim(),
    serialPort.textContent.trim(),
    parseInt(protocol.dataset.value),
  );
});

disconnectButton.addEventListener("click", disconnectElm);

const dtcScanButton = document.getElementById("dtc-scan-button");
dtcScanButton.addEventListener("click", async () => {
  await new Promise((r) => setTimeout(r, 500));

  emit("get-dtcs");
});

const fill = document.getElementById("btn-hold-fill");

function resetButtonFill() {
  clearInterval(interval);
  fill.style.transition = "width 0.3s";
  fill.style.width = "0%";
}

let holdTimeout;
let progress = 0;
let interval;

dtcClearButton.addEventListener("mousedown", () => {
  progress = 0;
  fill.style.transition = "none";
  fill.style.width = "0%";

  interval = setInterval(() => {
    progress += 1;
    fill.style.width = progress + "%";
    if (progress >= 100) {
      clearInterval(interval);

      // held
      resetButtonFill();
      clearDtcs();
    }
  }, 10);
});

dtcClearButton.addEventListener("mouseup", resetButtonFill);
dtcClearButton.addEventListener("mouseleave", resetButtonFill);

const dtcLogButton = document.getElementById("dtc-log-file");
const dtcList = document.getElementById("dtc-list");
dtcLogButton.addEventListener("click", () => exportDtcs(false));

const logFileButton = document.getElementById("log-file-button");
logFileButton.addEventListener("click", async () => {
  window.logFilePath = await save({
    title: "Save as JSON",
    defaultPath: "requests.json",
    filters: [{ name: "JSON", extensions: ["json"] }],
  });

  // set default path
  if (!window.logFilePath) {
    window.logFilePath = "./requests.json";
    return;
  }

  document.getElementById("log-file-path").textContent = window.logFilePath;
});

const imTestRefreshButton = document.getElementById("readiness-test-refresh");
const imTestExportButton = document.getElementById("readiness-test-export");
const imTestList = document.getElementById("readiness-tests-list");

imTestRefreshButton.addEventListener("click", () => {
  emit("get-readiness-tests");
});

imTestExportButton.addEventListener("click", async () => {
  let totalJSON = [];
  imTestList.childNodes.forEach((testRow) => {
    if (testRow.nodeType == 1) {
      const available =
        testRow.querySelector("#test-availability").textContent.trim() ==
        "AVAILABLE"
          ? true
          : false;
      const complete =
        testRow.querySelector("#test-completeness").textContent.trim() ==
        "COMPLETE"
          ? true
          : false;

      const testJSON = {
        name: testRow
          .querySelector("#test-name")
          .textContent.replace("TEST: ", "")
          .trim(),
        available: available,
        complete: complete,
      };

      totalJSON.push(testJSON);
    }
  });

  const path = await save({
    title: "Save as JSON",
    defaultPath: "readiness_tests.json",
    filters: [{ name: "JSON", extensions: ["json"] }],
  });

  if (!path) {
    return;
  }

  await writeFile({ path, contents: JSON.stringify(totalJSON, null, 2) });
});

const vinExportButton = document.getElementById("vin-export");
const vinDetails = document.getElementById("vin-container");

vinExportButton.addEventListener("click", async () => {
  let vinObj = {};
  vinDetails.childNodes.forEach((card) => {
    if (card.nodeType == 1) {
      console.log(card);
      const key = card
        .querySelector("h3")
        .textContent.replaceAll(" ", "_")
        .toLowerCase();
      const value = card.querySelector(".value").textContent.trim();
      vinObj[key] = value;
    }
  });

  const path = await save({
    title: "Save as JSON",
    defaultPath: "vin_details.json",
    filters: [{ name: "JSON", extensions: ["json"] }],
  });

  if (!path) {
    return;
  }

  await writeFile({ path, contents: JSON.stringify(vinObj, null, 2) });
});

appWindow.listen("tauri://close-requested", async () => {
  // check if delete logs on exit setting is set
  if (window.deleteLogsOnExit && window.logFilePath) {
    try {
      removeFile(window.logFilePath);
    } catch (err) {
      console.error("Error when trying to delete log:", err);
    }
  }

  await appWindow.close();
});

const input = document.getElementById("terminal-input");

input.addEventListener("input", (e) => {
  if (!input.value.startsWith("> ")) {
    input.value = "> ";
  }
});

input.addEventListener("keydown", (e) => {
  if (input.selectionStart < 2) {
    e.preventDefault();
  }
});

input.addEventListener("keydown", (e) => {
  if (e.key === "Enter") {
    if (input.value.trim() === "") {
      return;
    }

    const command = input.value.slice(2).trim();
    console.log("command:", command);
    if (command.length > 0) {
      console.log("send");
      appendTerminalOutput(command);
      emit("terminal-command", command);
      input.value = "> ";
    }
    e.preventDefault();
  }
});
