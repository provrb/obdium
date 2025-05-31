const { emit } = window.__TAURI__.event;
const { save } = window.__TAURI__.dialog;
const { writeFile } = window.__TAURI__.fs;
import { saveConnectionConfig } from "./settings.js";

const connectButton = document.getElementById("btn-connect");
const disconnectButton = document.getElementById("btn-disconnect");
const dtcList = document.getElementById("dtc-list");
const dtcHeader = document.getElementById("dtc-header");
const freezeFrameBar = document.getElementById("freeze-frame-status");
const obdGrid = document.getElementById("dashboard-cards");

export function clearDtcs() {
  console.log("test", dtcList.innerHTML);
  if (dtcList.innerHTML.trim() == "") {
    return;
  }

  // clear codes
  emit("clear-dtcs");
  dtcHeader.textContent = "DIAGNOSTIC TROUBLE CODES (0)";
  dtcList.innerHTML = ``;

  // get permanant codes
  // (codes that remain even after being cleared)
  emit("get-dtcs");
}

export async function exportDtcs(autoSave) {
  console.log("exporting autoSave?", autoSave);

  let totalJSON = [];
  let path = "./dtc_log.json";
  dtcList.childNodes.forEach((dtcRow) => {
    if (dtcRow.nodeType == 1) {
      const dtcJSON = {
        category: dtcRow.querySelector("#dtc-category").textContent.trim(),
        name: dtcRow.querySelector("#dtc-name").textContent.trim(),
        location: dtcRow.querySelector("#dtc-location").textContent.trim(),
        description: dtcRow
          .querySelector("#dtc-description")
          .textContent.trim(),
      };

      totalJSON.push(dtcJSON);
    }
  });

  if (!autoSave) {
    path = await save({
      title: "Save as JSON",
      defaultPath: "dtc_log.json",
      filters: [{ name: "JSON", extensions: ["json"] }],
    });

    if (!path) {
      return;
    }
  }

  console.log("path:", path);
  await writeFile({ path, contents: JSON.stringify(totalJSON, null, 2) });
}

export async function connectElm(baudRate, serialPort, protocol) {
  if (window.connected) {
    return;
  }

  const status = document.getElementById("connection-details");
  const recordResponses = document.getElementById("record-responses");
  const replayResponses = document.getElementById("replay-responses");

  let dots = 0;
  const interval = setInterval(() => {
    if (dots == 4) {
      dots = 0;
    }

    status.textContent = "CONNECTING" + ".".repeat(dots);
    dots += 1;
  }, 500);

  connectButton.disabled = true;
  emit("connect-elm", {
    serialPort: serialPort,
    baudRate: parseInt(baudRate),
    protocol: parseInt(protocol),
  });
  await new Promise((r) => setTimeout(r, 1000));
  clearInterval(interval);

  if (window.connected) {
    status.textContent =
      "CONNECTED THROUGH SERIAL PORT " + serialPort.toUpperCase();
    connectButton.disabled = true;
    disconnectButton.disabled = false;

    window.connectionConfig = {
      serialPort: serialPort,
      baudRate: baudRate,
      protocol: parseInt(protocol),
    };

    saveConnectionConfig();

    // enable buttons for logging
    recordResponses.disabled = false;
    replayResponses.disabled = false;

    if (window.autoCheckCodes) {
      emit("get-dtcs");
    }
  } else {
    status.textContent = "FAILED TO CONNECT THROUGH SERIAL PORT";
    connectButton.disabled = false;
    disconnectButton.disabled = true;
  }

  document.getElementById("baud-rate-selected").textContent = baudRate;
  document.getElementById("serial-port-selected").textContent = serialPort;
  document.getElementById("protocol-selected").dataset.value = protocol;
}

export async function disconnectElm() {
  const recordResponses = document.getElementById("record-responses");
  const replayResponses = document.getElementById("replay-responses");
  const status = document.getElementById("connection-details");

  let dots = 0;
  const interval = setInterval(() => {
    if (dots == 4) {
      dots = 0;
    }

    status.textContent = "DISCONNECTING" + ".".repeat(dots);
    dots += 1;
  }, 500);

  connectButton.disabled = true;
  emit("disconnect-elm");
  await new Promise((r) => setTimeout(r, 1000));
  clearInterval(interval);

  status.textContent = "NO CONNECTION ESTABLISHED";
  connectButton.disabled = false;
  disconnectButton.disabled = true;
  recordResponses.disabled = true;
  replayResponses.disabled = true;
}

export function freezeFrameDisclaimer(show) {
  freezeFrameBar.style.display = show ? "flex" : "none";
}

export function clearObdView() {
  obdGrid.innerHTML = '';
}