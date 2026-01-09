import { connectElm, freezeFrameDisclaimer } from "./features.js";
const { emit } = window.__TAURI__.event;

const defaultUserSettings = {
      saveDtcs: false,
      autoCheckCodes: false,
      autoConnect: false,
      hideNotifications: false,
      connectionConfig: {
        protocol: 0,
        baudRate: "0",
        serialPort: "0",
      },
      showPartialVin: false,
      deleteLogsOnExit: false,
      unitPreferences: {
        speed: "KilometersPerHour",
        distance: "Kilometers",
        temperature: "Celsius",
        torque: "NewtonMeters",
        pressure: "KiloPascal",
        flowRate: "LitresPerHour",
      },
    };

function importSettings() {
  let settings = JSON.parse(localStorage.getItem("userSettings"));
  if (!settings)
    settings = defaultUserSettings 

  if (!settings.connectionConfig) {
    settings.connectionConfig = window.connectionConfig;
  }

  // read from connectionConfig
  // if empty, we cannot connect
  if (settings.autoConnect && !window.connected && settings.connectionConfig) {
    connectElm(
      settings.connectionConfig.baudRate,
      settings.connectionConfig.serialPort,
      0,
    );
  }

  window.hideVin = settings.showPartialVin;
  window.deleteLogsOnExit = settings.deleteLogsOnExit;
  window.autoCheckCodes = settings.autoCheckCodes;
  window.autoSaveCodes = settings.saveDtcs;
  window.unitPreferences = settings.unitPreferences;
  window.hideNotifications = settings.hideNotifications;

  // show buttons depending on which are toggled
  document.getElementById("save-dtcs").checked = settings.saveDtcs;
  document.getElementById("auto-check-codes").checked = settings.autoCheckCodes;
  document.getElementById("hide-notifications").checked = settings.hideNotifications;
  document.getElementById("auto-connect").checked = settings.autoConnect;
  document.getElementById("hide-vin").checked = settings.showPartialVin;
  document.getElementById("del-logs").checked = settings.deleteLogsOnExit;

  updateUnitConversionDropdowns();
}

export function saveConnectionConfig() {
  let settings = JSON.parse(localStorage.getItem("userSettings"));
  if (!settings) {
    return;
  }

  if (window.connectionConfig) {
    settings.connectionConfig = window.connectionConfig;
    localStorage.setItem("userSettings", JSON.stringify(settings));
  }
}

export async function saveUnitPreference(unitType, unit) {
  if (!unitType || !window.unitPreferences[unitType]) return;
  window.unitPreferences[unitType] = unit;
  emit("set-unit-preferences", window.unitPreferences);

  // save setting
  let settings = JSON.parse(localStorage.getItem("userSettings"));
  if (!settings) settings = defaultUserSettings;

  settings.unitPreferences = window.unitPreferences;
  localStorage.setItem("userSettings", JSON.stringify(settings));

  updateUnitConversionDropdowns();
}

export async function updateUnitConversionDropdowns() {
  // update unit conversion dropdowns
  for (const [unitTypeKey, unitValue] of Object.entries(
    window.unitPreferences,
  )) {
    document.querySelectorAll("#unit-preference").forEach((dropdown) => {
      if (dropdown.getAttribute("data-target") == unitTypeKey) {
        const ul =
          dropdown.nextElementSibling &&
          dropdown.nextElementSibling.classList.contains("dropdown-menu")
            ? dropdown.nextElementSibling
            : null;
        if (ul) {
          ul.querySelectorAll("li").forEach((li) => {
            if (li.getAttribute("data-value") === unitValue) {
              dropdown.textContent = li.textContent;
            }
          });
        }
      }
    });
  }
}

async function settingChange(event) {
  // get existing settings
  let settings = JSON.parse(localStorage.getItem("userSettings"));
  if (!settings) settings = defaultUserSettings;

  // settings to save to localStorage
  // save dtcs
  // auto check for codes
  // auto connect startup
  // show partial vin
  // delete logs on exit
  const tId = event.target.id;
  const checked = event.target.checked;

  switch (tId) {
    case "save-dtcs":
      settings.saveDtcs = checked;
      break;
    case "auto-check-codes":
      settings.autoCheckCodes = checked;
      break;
    case "auto-connect":
      // save connection config
      settings.autoConnect = checked;
      if (window.connectionConfig) {
        settings.connectionConfig = window.connectionConfig;
      }
      break;
    case "hide-vin":
      settings.showPartialVin = checked;
      break;
    case "del-logs":
      settings.deleteLogsOnExit = checked;
      break;
    // the rest will have the frontend send an event
    // record response
    // replay response
    // use freeze frame
    case "record-responses":
      // uncheck replay requests
      document.getElementById("replay-responses").checked = false;
      await new Promise((r) => setTimeout(r, 2000));
      emit("settings-changed", { tId: "replay-responses", checked: false });
      emit("settings-changed", { tId, checked, data: window.logFilePath });
      break;
    case "replay-responses":
      // uncheck record requests
      document.getElementById("record-responses").checked = false;
      await new Promise((r) => setTimeout(r, 2000));
      emit("settings-changed", { tId: "record-responses", checked: false });
      emit("settings-changed", { tId, checked });
      break;
    case "hide-notifications":
      settings.hideNotifications = checked;
      window.hideNotifications = settings.hideNotifications;
      break;
    case "use-freeze-frame":
      emit("settings-changed", { tId, checked });
      freezeFrameDisclaimer(checked);
      break;
  }

  localStorage.setItem("userSettings", JSON.stringify(settings));
}

window.addEventListener("DOMContentLoaded", async () => {
  // sleep
  await new Promise((r) => setTimeout(r, 500));

  importSettings();

  document
    .getElementById("record-responses")
    .addEventListener("change", settingChange);
  document
    .getElementById("replay-responses")
    .addEventListener("change", settingChange);
  document
    .getElementById("hide-notifications")
    .addEventListener("change", settingChange);
  document
    .getElementById("save-dtcs")
    .addEventListener("change", settingChange);
  document
    .getElementById("auto-check-codes")
    .addEventListener("change", settingChange);
  document
    .getElementById("use-freeze-frame")
    .addEventListener("change", settingChange);
  document
    .getElementById("auto-connect")
    .addEventListener("change", settingChange);
  document.getElementById("hide-vin").addEventListener("change", settingChange);
  document.getElementById("del-logs").addEventListener("change", settingChange);
});
