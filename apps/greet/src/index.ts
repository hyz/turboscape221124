// import { fs, tauri } from "tauri-apps-api-dist";
import { fs, tauri } from "tauri-apps-api";
import init, { greet } from "../pkg";

console.log(1, 2, 3, fs.exists, tauri.invoke, init, greet);

(async function () {
	let bytes = await fs.readBinaryFile("../pkg/index_bg.wasm");
	let blob = new Blob([bytes], { type: "application/wasm" });
	let url = URL.createObjectURL(blob);
	await init(url);
})().then(console.log);

// init().then(console.log);
// invoke("greet", { name: "foo" }).then(console.log);
