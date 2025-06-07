const { emit, listen } = window.__TAURI__.event;
const { save } = window.__TAURI__.dialog;
const { writeFile } = window.__TAURI__.fs;
const invoke = window.__TAURI__.invoke;
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
  connectButton.disabled = true;
  emit("disconnect-elm");
}

export function freezeFrameDisclaimer(show) {
  freezeFrameBar.style.display = show ? "flex" : "none";
}

export function clearObdView() {
  obdGrid.innerHTML = "";
}

export async function addGraphDropdownOption(pid, name, unit, equation) {
  if (!unit.trim() && !equation.trim()) {
    // likely a statement or cannot be represented as a draph (enum)
    // e,g: dtc's, fuel system status
    return;
  }

  const graphs = document.querySelectorAll("canvas");
  for (const graph of graphs) {
    const graphId = graph.id;
    const dropDownMenu = document.getElementById(graphId + "-menu");

    const existingOption = dropDownMenu.querySelector(
      `li[data-value="${pid}"]`,
    );
    if (existingOption) continue;

    const pidOption = document.createElement("li");
    pidOption.textContent = name.toUpperCase();
    pidOption.dataset.value = pid;

    dropDownMenu.appendChild(pidOption);

    // track click of dropdown menu
    pidOption.addEventListener("click", () => {
      // switch graph metrics
      trackGraph(graphId, name, unit);
    });
  }
}

export async function trackGraph(graphId, name, unit) {
  // reset all chart.js data
  // track graph

  let graph = Chart.getChart(graphId);
  if (!graph) return;

  // reset graph
  graph.data.labels = [];
  graph.data.datasets.forEach((dataset) => {
    dataset.data = [];
  });

  // x axis label will always be time
  graph.options.scales.x.title.text = "TIME";
  graph.startTime = Date.now();
  graph.update();

  // track data
  let lastValue = null;
  let lastElapsed = null;

  // listen for updates and store the latest value
  listen("update-graphs", (event) => {
    if (event.payload.name.toUpperCase() === name.toUpperCase()) {
      console.log(" -> Update!");
      lastValue = event.payload.value;
      if (graph.options.scales.y.title.text.toUpperCase() != event.payload.unit.toUpperCase()) {
        graph.options.scales.y.title.text = event.payload.unit.toUpperCase();
      }
    }
  });

  // update graph every second, even if no new event
  setInterval(() => {
    if (
      graph.config &&
      graph.config.options
    ) {
      const now = Date.now();
      const elapsedMs = now - graph.startTime;
      const minutes = Math.floor(elapsedMs / 60000);
      const seconds = Math.floor((elapsedMs % 60000) / 1000);
      const elapsed = `${minutes}:${seconds.toString().padStart(2, "0")}`;

      if (lastElapsed === elapsed) return;
      lastElapsed = elapsed;

      const maxPoints = 8;
      if (graph.data.labels.length > maxPoints) {
        graph.data.labels.shift();
        graph.data.datasets[0].data.shift();
      }

      // Use lastValue if available, otherwise push null
      graph.data.datasets[0].data.push(lastValue);
      graph.data.labels.push(elapsed);

      graph.update();
    }
  }, 1000);
}
