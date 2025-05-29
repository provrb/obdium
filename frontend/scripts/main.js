const { listen, emit } = window.__TAURI__.event;
import { saveConnectionConfig } from './settings.js'

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

// When frontend gets loaded
// alert the backend with an event.
window.addEventListener('DOMContentLoaded', () => {
    emit('frontend-loaded');
    // Load pid list
    emit('get-pids')

    // load serial ports
    emit('get-serial-ports');
});

export async function connect_elm(baudRate, serialPort, protocol) {
    const status = document.getElementById('connection-details');
    const recordResponses = document.getElementById('record-responses');
    const replayResponses = document.getElementById('replay-responses');
    
    let dots = 0;
    const interval = setInterval(() => {
        if (dots == 4) {
            dots = 0;
        }

        status.textContent = "CONNECTING" + '.'.repeat(dots);
        dots += 1;
    }, 500);

    connectButton.disabled = true;
    emit('connect-elm', { serialPort: serialPort, baudRate: parseInt(baudRate), protocol: parseInt(protocol) });
    await new Promise(r => setTimeout(r, 2000));
    clearInterval(interval);

    if (window.connected) {
        status.textContent = "CONNECTED THROUGH SERIAL PORT " + serialPort.toUpperCase();
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
    } else {
        status.textContent = "FAILED TO CONNECT THROUGH SERIAL PORT";
        connectButton.disabled = false;
        disconnectButton.disabled = true;
    }

    document.getElementById('baud-rate-selected').textContent = baudRate;
    document.getElementById('serial-port-selected').textContent = serialPort;
    document.getElementById('protocol-selected').dataset.value = protocol ;
}

async function disconnect_elm() {
    const recordResponses = document.getElementById('record-responses');
    const replayResponses = document.getElementById('replay-responses');
    const status = document.getElementById('connection-details');

    let dots = 0;
    const interval = setInterval(() => {
        if (dots == 4) {
            dots = 0;
        }

        status.textContent = "DISCONNECTING" + '.'.repeat(dots);
        dots += 1;
    }, 500);

    connectButton.disabled = true;
    emit('disconnect-elm');
    await new Promise(r => setTimeout(r, 1000));
    clearInterval(interval);

    status.textContent = "NO CONNECTION ESTABLISHED";
    connectButton.disabled = false;
    disconnectButton.disabled = true;
    recordResponses.disabled = true;
    replayResponses.disabled = true;
}

const dropdowns = document.querySelectorAll(".dropdown");

dropdowns.forEach(dropdown => {
    const toggle = dropdown.querySelector(".dropdown-toggle");
    const menu = dropdown.querySelector(".dropdown-menu");

    toggle.addEventListener("click", (e) => {
        e.stopPropagation();
        if (menu.style.display === "block") {
            menu.style.display = "none";
        } else {
            document.querySelectorAll(".dropdown-menu").forEach(m => {
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
    document.querySelectorAll(".dropdown-menu").forEach(menu => {
        menu.style.display = "none";
    });
});

const connectButton = document.getElementById('btn-connect');
const disconnectButton = document.getElementById('btn-disconnect');

connectButton.addEventListener("click", async () => {
    const baudRate = document.getElementById('baud-rate-selected');
    const serialPort = document.getElementById('serial-port-selected');
    const protocol = document.getElementById('protocol-selected');

    connect_elm(baudRate.textContent, serialPort.textContent, parseInt(protocol.dataset.value));
})

disconnectButton.addEventListener("click", async () => {
    disconnect_elm();
})