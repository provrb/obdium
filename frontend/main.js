const { listen, emit } = window.__TAURI__.event;

// When frontend gets loaded
// alert the backend with an event.
window.addEventListener('DOMContentLoaded', () => {
    window.__TAURI__.event.emit('frontend-loaded');
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
    } else {
        connection_label.textContent = "ELM327 NOT CONNECTED";
        connection_icon.src = "/assets/icons/not-connected.png";
    }

    console.log(event.payload.message);
});

listen('add-pid-info', (event) => {
    const pidList = document.getElementById('pid-list');
    const pidGroup = document.createElement('div');
    pidGroup.className = 'pid-group';

    pidGroup.innerHTML = `
    <div class="pid-row">
      <button class="arrow-icon"><img src="/assets/icons/arrow-icon.png"></button>
      <span class="name">${event.payload.pid_name.toUpperCase()}</span>
    </div>
    <div class="pid-details" style="display: none; height: 0;">
      <div class="pid-data-columns">
        <div class="pid-column">
          <div class="pid-label">MODE</div>
          <div class="pid-value">$${event.payload.mode}</div>
        </div>
        <div class="pid-column">
          <div class="pid-label">PID</div>
          <div class="pid-value">${event.payload.pid}</div>
        </div>
        <div class="pid-column">
          <div class="pid-label">COMMAND</div>
          <div class="pid-value">${event.payload.mode + event.payload.pid}</div>
        </div>
        <div class="pid-column">
          <div class="pid-label">EQUATION</div>
          <div class="pid-value">${event.payload.formula == "" ? "??" : event.payload.formula}</div>
        </div>
        <div class="pid-column">
          <div class="pid-label">UNIT</div>
          <div class="pid-value">${event.payload.unit == "" ? "??" : event.payload.unit.toUpperCase()}</div>
        </div>
      </div>
      <button class="pid-button">DETAILS</button>
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
});