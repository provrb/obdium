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
    
    vin.textContent = event.payload.vin.toUpperCase();
    makeModel.textContent = (event.payload.make + " " + event.payload.model).toUpperCase();
});

listen('connection-status', (event) => {
    const connection_label = document.getElementById("connection-label");
    const connection_icon = document.getElementById("connection-icon");

    if (event.payload.connected) {
        connection_label.textContent = "ELM327 CONNECTED VIA " + event.payload.serial_port;
        connection_icon.src = "/assets/connected.png";
    } else {
        connection_label.textContent = "ELM327 NOT CONNECTED";
    }

    console.log(event.payload.message);
});