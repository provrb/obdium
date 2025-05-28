const { listen, emit } = window.__TAURI__.event;

// ELM connection
// Changes when connection-status is fired
let connected = false;

// When frontend gets loaded
// alert the backend with an event.
window.addEventListener('DOMContentLoaded', () => {
    emit('frontend-loaded');
    // Load pid list
    emit('get-pids')

    // load serial ports
    emit('get-serial-ports');
});

listen('update-card', (event) => {
    const cards = document.querySelectorAll('.card');
    const exists = Array.from(cards).some(card => {
        return card.textContent.includes(event.payload.name);
    });

    // create card if it doesnt exist
    if (!exists) {
        const container = document.querySelector('.grid');
        if (!container)
            return;

        // create the card
        // has a header, value, and then unit
        const card = document.createElement('div');
        const h3 = document.createElement('h3');
        const valueDiv = document.createElement('div');
        const unitSpan = document.createElement('span');

        card.className = 'card';
        valueDiv.className = 'value';
        unitSpan.className = 'unit';

        h3.textContent = event.payload.name;

        if (event.payload.unit.toLowerCase() !== "no data")
           unitSpan.textContent = event.payload.unit;

        // if the unit is no data meaning that scalar has 'no data'
        // (see backend docs), then display 'N/A' for value
        const valueText = event.payload.unit.toLowerCase() === "no data" ? "N/A" : event.payload.value.toString();

        // add value and unit to one div to align horizontally
        valueDiv.appendChild(document.createTextNode(valueText + " "));
        valueDiv.appendChild(unitSpan);

        // append elements to card
        card.appendChild(h3);
        card.appendChild(valueDiv);

        container.appendChild(card);

        return;
    }

    cards.forEach(card => {
        // Don't update card data.
        if (card.classList.contains('dimmed')) {
            return;
        }

        const h3 = card.querySelector('h3');
        if (!h3) return;

        const title = h3.textContent?.toLowerCase();
        if (title == event.payload.name.toLowerCase()) {
            const valueElem = card.querySelector('.value');
            const unitElem = card.querySelector('.unit');

            if (valueElem) {
                const textNode = Array.from(valueElem.childNodes).find(
                    node => node.nodeType === Node.TEXT_NODE
                );

                if (textNode) {
                    textNode.textContent = event.payload.unit.toLowerCase() === "no data"
                        ? "N/A "
                        : event.payload.value.toString() + " ";
                }
            }
        }
    });
});

listen('vehicle-details', (event) => {
    const vin = document.querySelector(".vin")
    const makeModel = document.querySelector(".car-model");

    vin.textContent = "VIN: " + event.payload.vin.toUpperCase();
    makeModel.textContent = (event.payload.make + " " + event.payload.model).toUpperCase();
});

listen('connection-status', (event) => {
    const connection_label = document.getElementById("connection-label");
    const connection_icon = document.getElementById("connection-icon");

    if (event.payload.connected) {
        connection_label.textContent = "ELM327 CONNECTED VIA " + event.payload.serial_port.toUpperCase();
        connection_icon.src = "/assets/icons/connected.png";
        connected = true;
    } else {
        connection_label.textContent = "ELM327 NOT CONNECTED";
        connection_icon.src = "/assets/icons/not-connected.png";
        connected = false;
    }

    console.log(event.payload.message);
});

listen('update-pids', (event) => {
    const pids = event.payload;
    const pidList = document.getElementById('pid-list');

    for (const pidInfo of pids) {
        const pidGroup = document.createElement('div');
        pidGroup.className = 'pid-group';

        pidGroup.innerHTML = `
        <div class="pid-container">
            <div class="pid-row">
            <button class="arrow-icon"><img src="/assets/icons/arrow-icon.png"></button>
            <span class="name">${pidInfo.pid_name.toUpperCase()}</span>
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
            <button class="pid-button">DETAILS</button>
            </div>
        </div>
      `;

        pidList.appendChild(pidGroup);

        // Increment results counter
        const header = document.getElementById('pid-list-header');
        header.textContent = "VIEW PIDS (" + pidList.children.length + ")";

        // Add expand/collapse event listener
        const row = pidGroup.querySelector('.pid-row');
        const details = pidGroup.querySelector('.pid-details');

        row.addEventListener('click', () => {
            const expanded = row.classList.contains('expanded');

            if (expanded) {
                details.style.height = details.scrollHeight + 'px';
                requestAnimationFrame(() => {
                    details.style.height = '0px';
                });
                row.classList.remove('expanded');
            } else {
                details.style.display = 'block';
                const height = details.scrollHeight + 'px';
                details.style.height = '0px';
                requestAnimationFrame(() => {
                    details.style.height = height;
                });
                row.classList.add('expanded');
            }

            details.addEventListener('transitionend', function handler(e) {
                if (e.propertyName === 'height') {
                    if (!row.classList.contains('expanded')) {
                        details.style.display = 'none';
                    } else {
                        details.style.height = 'auto';
                    }
                    details.removeEventListener('transitionend', handler);
                }
            });
        });
    }
});

const menu = document.getElementById('serial-port-dropdown-menu');
const serialPortSelected = document.getElementById('serial-port-selected');

listen('update-serial-ports', (event) => {
    serialPortSelected.textContent = "NO PORTS SELECTED";
    menu.innerHTML = '';
    
    if (event.payload === "") {
        return;
    }
    
    const portOption = document.createElement('li');
    portOption.textContent = event.payload;
    portOption.dataset.value = event.payload;

    menu.appendChild(portOption);
})

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
    const baudRate = document.getElementById('baud-rate-selected').textContent;
    const serialPort = document.getElementById('serial-port-selected').textContent;
    
    const protocol = document.getElementById('protocol-selected').dataset.value;
    const status = document.getElementById('connection-details');
    
    let dots = 0;
    const interval = setInterval(() => {
        if (dots == 4) {
            dots = 0;
        }
        
        status.textContent = "CONNECTING" + '.'.repeat(dots);
        dots+=1;
    }, 500);

    connectButton.disabled = true;
    emit('connect-elm', { serialPort: serialPort, baudRate: parseInt(baudRate), protocol: parseInt(protocol) });
    await new Promise(r => setTimeout(r, 2000));
    clearInterval(interval);
    
    if (connected) {
        status.textContent = "CONNECTED THROUGH SERIAL PORT " + serialPort.toUpperCase();
        connectButton.disabled = true;
        disconnectButton.disabled = false;
    } else {
        status.textContent = "FAILED TO CONNECT THROUGH SERIAL PORT";
        connectButton.disabled = false;
        disconnectButton.disabled = true;
    }
})

disconnectButton.addEventListener("click", async () => {
    const status = document.getElementById('connection-details');
    
    let dots = 0;
    const interval = setInterval(() => {
        if (dots == 4) {
            dots = 0;
        }
        
        status.textContent = "DISCONNECTING" + '.'.repeat(dots);
        dots+=1;
    }, 500);

    connectButton.disabled = true;
    emit('disconnect-elm');
    await new Promise(r => setTimeout(r, 1000));
    clearInterval(interval);
    
    status.textContent = "NO CONNECTION ESTABLISHED";
    connectButton.disabled = false;
    disconnectButton.disabled = true;
})
