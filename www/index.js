import * as wasm from "orbits";

let solSys = wasm.SolSystem.new();

let zf = 30;

// time step of the simulation, in days
let step = 10  

// let positions = sol_sys.positions();

// console.log(positions);

const canvas = document.getElementById("sol-system");
canvas.height = 800;
canvas.width = 1200;

const ctx = canvas.getContext('2d');

const renderloop = () => {
    fps.render()
    ctx.fillStyle = "black"
    ctx.fillRect(0, 0, canvas.width, canvas.height)
    solSys.tick(step);
    drawSystem();
    requestAnimationFrame(renderloop)
}

const drawSystem = () => {
    let position = solSys.positions();
    let time = position.time;
    let date = julianToDate(time);
    let day = date.getUTCDate();
    let month = date.getUTCMonth();
    month = monthNumToText(month)
    let year = date.getUTCFullYear();

    let coords = position.coords

    let names = position.names

    ctx.fillStyle = "white";
    ctx.strokeStyle = "#FFFFFF50";
    ctx.font = "15px sans-serif"

    ctx.fillText(`Year: ${year}`, canvas.width - 95, 20)
    ctx.fillText(`Month: ${month}`, canvas.width - 95, 40)
    ctx.fillText(`Day: ${day}`, canvas.width - 95, 60)

    let i = 0
    for (let [c, name] of zip(coords, names)) {
        ctx.fillStyle = name === "Sol" ? "yellow" : "white";

        let x = c[0]*zf+(canvas.width / 2)
        let y = -c[1]*zf+(canvas.height / 2);
        ctx.fillText(name, 10, 20+20*i)
        ctx.moveTo(50, 20+20*i)
        ctx.lineTo(x,y)
        ctx.stroke();
        ctx.beginPath();
        
        let circle_rad = name === "Sol" ? 5 : 2;
        ctx.arc(x, y, circle_rad, 0, 2 * Math.PI);
        ctx.fill();
        i++
        ctx.fillStyle = "white"
    }
}

const zip = (a, b) => a.map((k, i) => [k, b[i]]);

const julianToDate = (j) => {
    let millis = (j - 2440587.5) * 86400000
    return new Date(millis)
}

const monthNumToText = (m) => {
    let months = ["Jan", "Feb", "Mar", "Apr", "May",
                    "Jun", "Jul", "Aug", "Sep", "Oct",
                    "Nov", "Dec"];
    return months[m]
}


/*
    fps counter from https://rustwasm.github.io/book/game-of-life/time-profiling.html
*/
const fps = new class {
  constructor() {
    this.fps = document.getElementById("fps");
    this.frames = [];
    this.lastFrameTimeStamp = performance.now();
  }

  render() {
    // Convert the delta time since the last frame render into a measure
    // of frames per second.
    const now = performance.now();
    const delta = now - this.lastFrameTimeStamp;
    this.lastFrameTimeStamp = now;
    const fps = 1 / delta * 1000;

    // Save only the latest 100 timings.
    this.frames.push(fps);
    if (this.frames.length > 100) {
      this.frames.shift();
    }

    // Find the max, min, and mean of our 100 latest timings.
    let min = Infinity;
    let max = -Infinity;
    let sum = 0;
    for (let i = 0; i < this.frames.length; i++) {
      sum += this.frames[i];
      min = Math.min(this.frames[i], min);
      max = Math.max(this.frames[i], max);
    }
    let mean = sum / this.frames.length;

    // Render the statistics.
    this.fps.textContent = `
Frames per Second:
         latest = ${Math.round(fps)}
avg of last 100 = ${Math.round(mean)}
min of last 100 = ${Math.round(min)}
max of last 100 = ${Math.round(max)}
`.trim();
  }
};

renderloop()