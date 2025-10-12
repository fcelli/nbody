import init, { WasmSim } from "./pkg/web.js";

async function run() {
    // Load and initialize the WebAssembly binary (web_bg.wasm)
    await init();

    // Create simulation instance
    const sim = new WasmSim();

    // Get reference to canvas and 2D context
    const canvas = document.getElementById("sim");
    const ctx = canvas.getContext("2d");

    // Set the canvas size to match the current window size
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;

    function draw() {
        sim.step(0.001);
        const positions = sim.get_positions();

        // Clear the screen
        ctx.fillStyle = "black";
        ctx.fillRect(0, 0, canvas.width, canvas.height);

        // Set the color for drawing the bodies (white)
        ctx.fillStyle = "white";

        // Loop over all bodies and draw them
        for (let i = 0; i < positions.length; i++) {
            const [x, y] = positions[i];

            // Convert simulation coordinates into screen pixels
            const cx = canvas.width / 2 + x * 100;
            const cy = canvas.height / 2 + y * 100;

            // Draw a circle for each body
            ctx.beginPath();
            ctx.arc(cx, cy, 3, 0, 2 * Math.PI);
            ctx.fill();
        }

        requestAnimationFrame(draw);
    }

    draw();
}

run();