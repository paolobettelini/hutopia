import init, { run } from 'widget';
async function main() {
    let urlString = document.currentScript.src;
    let address = new URL(urlString).host;
    window.CHAT_WS_ADDRESS = address; // Set this property so that the widgets can access it

    await init();
    run();
}
main();