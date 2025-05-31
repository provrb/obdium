const { listen, emit } = window.__TAURI__.event;

import {
  clearDtcs,
  exportDtcs,
  connectElm,
  disconnectElm,
  clearObdView,
} from "./features.js";

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

// UI Components
const dropdowns = document.querySelectorAll(".dropdown");
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

  console.log("frontend loaded");
});

clearObdButton.addEventListener("click", clearObdView);
pauseObdButton.addEventListener("click", () => {
  window.obdViewPaused = !window.obdViewPaused;
  if (obdViewPaused) {
    pauseObdButton.textContent = "RESUME";
  } else {
    pauseObdButton.textContent = "PAUSE";
  }
})

dropdowns.forEach((dropdown) => {
  const toggle = dropdown.querySelector(".dropdown-toggle");
  const menu = dropdown.querySelector(".dropdown-menu");

  toggle.addEventListener("click", (e) => {
    e.stopPropagation();
    if (menu.style.display === "block") {
      menu.style.display = "none";
    } else {
      document.querySelectorAll(".dropdown-menu").forEach((m) => {
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
    }
  });
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
    baudRate.textContent,
    serialPort.textContent,
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
dtcLogButton.addEventListener("click", exportDtcs);
