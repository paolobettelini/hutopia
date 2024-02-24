class ConsoleElement extends HTMLElement {
    static observedAttributes = [];
  
    constructor() {
        super();
    }
  
    connectedCallback() {
        this.innerHTML = "console widget";
    }
}

customElements.define("widget-console", ConsoleElement);