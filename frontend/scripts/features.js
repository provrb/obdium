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
const terminalOutput = document.getElementById("terminal-output");

export function clearDtcs() {
  addNotification(
    "TROUBLE CODES",
    "Cleared all pending diagnostic trouble codes.",
  );

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

const activeTrackers = new Map(); // graphId -> cleanupFn

export async function trackGraph(graphId, name, unit) {
  if (activeTrackers.has(graphId)) {
    const cleanup = activeTrackers.get(graphId);
    cleanup();
  }

  const graph = Chart.getChart(graphId);
  if (!graph) return;

  // Reset graph data
  graph.data.labels = [];
  graph.data.datasets.forEach((dataset) => {
    dataset.data = [];
  });

  graph.options.scales.x.title.text = "TIME";
  graph.startTime = Date.now();
  graph.update();

  let lastValue = null;
  let lastSeenValue = null;
  let lastElapsed = null;
  const myName = name.trim().toUpperCase();

  const listener = (event) => {
    const incomingName = event.payload.name?.trim().toUpperCase();
    if (incomingName === myName) {
      lastValue = event.payload.value;

      if (
        graph.options.scales.y.title.text.toUpperCase() !==
        event.payload.unit.toUpperCase()
      ) {
        graph.options.scales.y.title.text = event.payload.unit.toUpperCase();
      }
    }
  };

  const unlisten = await listen("update-graphs", listener);
  const intervalId = setInterval(() => {
    if (!graph.config?.options) return;

    const now = Date.now();
    const elapsedMs = now - graph.startTime;
    const minutes = Math.floor(elapsedMs / 60000);
    const seconds = Math.floor((elapsedMs % 60000) / 1000);
    const elapsed = `${minutes}:${seconds.toString().padStart(2, "0")}`;

    if (lastElapsed === elapsed) return;
    lastElapsed = elapsed;

    if (graph.data.labels.length > 8) {
      graph.data.labels.shift();
      graph.data.datasets[0].data.shift();
    }

    if (lastValue !== null && lastValue !== undefined) {
      lastSeenValue = lastValue;
    }

    graph.data.datasets[0].data.push(lastSeenValue);
    graph.data.labels.push(elapsed);

    graph.update();
  }, 1000);

  const cleanup = () => {
    clearInterval(intervalId);
    unlisten();
    activeTrackers.delete(graphId);
  };

  activeTrackers.set(graphId, cleanup);
}

export function appendTerminalOutput(msg) {
  const now = new Date();
  const timeStr =
    now.toLocaleTimeString("en-US", { hour12: false }) +
    ":" +
    now.getMilliseconds().toString().padStart(3, "0");
  terminalOutput.innerText += `${timeStr} ${msg}\n`;
  terminalOutput.scrollTop = terminalOutput.scrollHeight;
}

export function removeNotification(el) {
  el.style.transition = "opacity 0.2ss";
  el.style.opacity = "0";
  setTimeout(() => el.remove(), 200);
}

export function addNotification(title, desc) {
  const container = document.getElementById("notification-container");

  const notification = document.createElement("div");
  notification.className = "notification";
  notification.innerHTML = `
    <div class="notification-hint">NOTIFICATION</div>
    <button class="notification-close">
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
        stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
        <path d="M18 6 6 18" />
        <path d="m6 6 12 12" />
      </svg>
    </button>
    <div class="notification-title">${title}</div>
    <div class="notification-desc">${desc}</div>
    <div class="notification-progress"></div>
  `;

  notification
    .querySelector(".notification-close")
    .addEventListener("click", function () {
      removeNotification(notification);
    });

  container.appendChild(notification);

  // if there are more than 3 notifications present
  // remove the bottom one
  if (container.children.length > 3) {
    removeNotification(container.children[0]);
  }

  setTimeout(() => removeNotification(notification), 6000);
}

export function listenExpandPID(pidGroup) {
  // Add expand/collapse event listener
  const row = pidGroup.querySelector(".info-row");
  const details = pidGroup.querySelector(".pid-details");

  row.addEventListener("click", () => {
    const expanded = row.classList.contains("expanded");

    if (expanded) {
      details.style.height = details.scrollHeight + "px";
      requestAnimationFrame(() => {
        details.style.height = "0px";
      });
      row.classList.remove("expanded");
    } else {
      details.style.display = "block";
      const height = details.scrollHeight + "px";
      details.style.height = "0px";
      requestAnimationFrame(() => {
        details.style.height = height;
      });
      row.classList.add("expanded");
    }

    details.addEventListener("transitionend", function handler(e) {
      if (e.propertyName === "height") {
        if (!row.classList.contains("expanded")) {
          details.style.display = "none";
        } else {
          details.style.height = "auto";
        }
        details.removeEventListener("transitionend", handler);
      }
    });
  });
}

export function btnHoldToActivate(
  button,
  fillElement,
  onActivate,
  options = {},
) {
  const { holdTime = 1000, fillTransition = "width 0.3s" } = options;

  let interval;
  let progress = 0;
  let isActive = false;

  function resetButtonFill() {
    clearInterval(interval);
    fillElement.style.transition = fillTransition;
    fillElement.style.width = "0%";
    progress = 0;
    isActive = false;
  }

  button.addEventListener("mousedown", () => {
    progress = 0;
    isActive = true;
    fillElement.style.transition = "none";
    fillElement.style.width = "0%";
    const step = 100 / (holdTime / 10);

    interval = setInterval(() => {
      if (!isActive) return;
      progress += step;
      fillElement.style.width = Math.min(progress, 100) + "%";
      if (progress >= 100) {
        clearInterval(interval);
        resetButtonFill();
        onActivate();
      }
    }, 10);
  });

  button.addEventListener("mouseup", resetButtonFill);
  button.addEventListener("mouseleave", resetButtonFill);
}

export function addCustomPIDRow() {
  const pidGroup = document.createElement("div");
  const pidList = document.getElementById("pid-list");

  pidGroup.className = "pid-group";
  pidGroup.innerHTML = `
      <div class="pid-container">
        <div class="info-row">
          <button class="arrow-icon"><img src="/assets/icons/arrow-icon.png"></button>
          <input class="name" type="text" placeholder="ENTER PID NAME" maxlength="55"></input>
        </div>
        <div class="pid-details" style="display: none; height: 0;">
          <div class="pid-data-columns">
            <div class="pid-column">
              <div class="pid-label">MODE</div>
              <input class="pid-value" type="text" placeholder="??" maxlength="2"></input>
            </div>
            <div class="pid-column">
              <div class="pid-label">PID</div>
              <input class="pid-value" type="text" placeholder="??" maxlength="4"></input>
            </div>
            <div class="pid-column">
              <div class="pid-label">COMMAND</div>
              <div class="pid-value"></div>
            </div>
            <div class="pid-column">
              <div class="pid-label">EQUATION</div>
              <input class="pid-value" type="text" placeholder="??"></input>
            </div>
            <div class="pid-column">
              <div class="pid-label">UNIT</div>
              <input class="pid-value" type="text" placeholder="??"></input>
            </div>
          </div>
          <div class="button-row">
            <div class="pid-button" id="remove-pid">
              <span style="z-index: 5">REMOVE</span>
              <div class="btn-hold-fill" id="remove-pid-fill" style="transition: width 0.3s; width: 0%;"></div>
            </div>
            <div class="pid-button" id="track-pid">
              <span style="z-index: 5">TRACK</span>
              <div class="btn-hold-fill" id="track-pid-fill" style="transition: width 0.3s; width: 0%;"></div>
            </div>
          </div>
        </div>
      </div>
      `;

  const nameInput = pidGroup.querySelector(".info-row .name");
  const modeInput = pidGroup.querySelector(
    ".pid-column:nth-child(1) .pid-value",
  );
  const pidInput = pidGroup.querySelector(
    ".pid-column:nth-child(2) .pid-value",
  );
  const commandDiv = pidGroup.querySelector(
    ".pid-column:nth-child(3) .pid-value",
  );
  const equationInput = pidGroup.querySelector(
    ".pid-column:nth-child(4) .pid-value",
  );
  const unitInput = pidGroup.querySelector(
    ".pid-column:nth-child(5) .pid-value",
  );

  function isValidPID() {
    // mode, pid, equation, unit, pid name must all be filled out
    const mode = modeInput.value.trim();
    const pid = pidInput.value.trim();
    const equation = equationInput.value.trim().toUpperCase();
    const unit = unitInput.value.trim();
    const name = nameInput.value.trim();

    return (
      mode.startsWith("$") &&
      mode.length - 1 == 2 &&
      pid &&
      pid.length >= 2 &&
      equation &&
      (equation.includes("A") ||
        equation.includes("B") ||
        equation.includes("C") ||
        equation.includes("D") ||
        equation.includes("E") ||
        equation.includes("F")) &&
      unit &&
      name
    );
  }

  function updateCommand() {
    const mode = modeInput.value.trim().replace("$", "");
    const pid = pidInput.value.trim();
    commandDiv.textContent = mode && pid ? `${mode}${pid}` : "";
  }

  function trackCustomPID() {
    const modeValue = modeInput.value.trim();
    const pidValue = pidInput.value.trim();
    const equationValue = equationInput.value.trim().toUpperCase();
    const unitValue = unitInput.value.trim();
    const commandValue = commandDiv.textContent.trim();
    const nameValue = nameInput.value.trim();

    console.log("track");

    // tell the backend to track the new custom pid
    let customPid = {
      mode: modeValue,
      pid: pidValue,
      unit: unitValue,
      command: commandValue,
      equation: equationValue,
      name: nameValue,
    };

    emit("track-custom-pid", customPid);
    trackBtn.disabled = true;
    trackBtn.style += "cursor: not-allowed;";
  }

  modeInput.addEventListener("keydown", (e) => {
    if (e.key === "Enter") {
      const mode = modeInput.value.trim();
      if (!mode.startsWith("$")) {
        modeInput.value = "$" + mode;
      }

      const pid = pidInput.value.trim();
      commandDiv.textContent = mode && pid ? `${mode}${pid}` : "";
    }
  });

  modeInput.addEventListener("blur", () => {
    const mode = modeInput.value.trim();
    if (!mode.startsWith("$") && mode.length > 0) {
      modeInput.value = "$" + mode;
    }

    const pid = pidInput.value.trim();
    commandDiv.textContent = mode && pid ? `${mode}${pid}` : "";
  });

  modeInput.addEventListener("click", (e) => {
    const mode = modeInput.value.trim();
    if (mode.startsWith("$")) {
      modeInput.value = mode.slice(1);
    }
  });

  pidInput.addEventListener("keydown", (e) => {
    if (e.key === "Enter") {
      updateCommand();
    }
  });

  pidInput.addEventListener("blur", () => {
    updateCommand();
  });

  const removeBtn = pidGroup.querySelector("#remove-pid");
  const fill = pidGroup.querySelector("#remove-pid-fill");
  btnHoldToActivate(
    removeBtn,
    fill,
    () => {
      if (pidGroup) {
        pidGroup.remove();
      }
    },
    { holdTime: 500 },
  );

  const trackBtn = pidGroup.querySelector("#track-pid");
  const trackFill = pidGroup.querySelector("#track-pid-fill");

  btnHoldToActivate(
    trackBtn,
    trackFill,
    () => {
      if (isValidPID()) {
        trackCustomPID();
      }
    },
    { holdTime: 200 },
  );

  pidList.appendChild(pidGroup);
  pidList.scrollTop = pidList.scrollHeight;
  listenExpandPID(pidGroup);
}
