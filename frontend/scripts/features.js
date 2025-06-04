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

  connectButton.disabled = true;
  emit("connect-elm", {
    serialPort: serialPort,
    baudRate: parseInt(baudRate),
    protocol: parseInt(protocol),
  });
}

export async function disconnectElm() {
  const status = document.getElementById("connection-details");

  connectButton.disabled = true;
  emit("disconnect-elm");
}

export function freezeFrameDisclaimer(show) {
  freezeFrameBar.style.display = show ? "flex" : "none";
}

export function clearObdView() {
  obdGrid.innerHTML = "";
}

export function showGraph() {}
