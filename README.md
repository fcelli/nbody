# N-Body Simulation
This project is a simple N-body gravitational simulation written in Rust.  
The simulation can be visualized in the browser using WebAssembly (WASM).

## How to Set Up
Make sure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://drager.github.io/wasm-pack/)
- [npm](https://nodejs.org/) or [python](https://www.python.org/)

> **Note:**  
> You don’t *need* Node.js or Python for the simulation itself - they are only used to run a simple web server for testing the simulation locally.

Clone the repository with:
```shell
git clone git@github.com:fcelli/nbody.git
```

From the `web/` directory:
```shell
cd web
wasm-pack build --target web
```
This compiles your Rust code to WebAssembly and outputs it in the `web/pkg/` directory.

## How to Run Locally
You can serve the `web/` directory with any simple static server.
For example, using npm’s http-server:
```shell
npm install -g http-server
http-server .
```

Alternatively, with Python 3:
```shell
python3 -m http.server
```
