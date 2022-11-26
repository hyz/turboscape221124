// import { fs, tauri } from "tauri-apps-api-dist";
import { fs, tauri } from "tauri-apps-api";
import init, { greet } from "../pkg";
// import "../pkg/index_bg.wasm";

// console.log(1, 2, 3, fs.exists, tauri.invoke, init, greet);

(async function (index_bg_wasm) {
	let bytes = await fs.readBinaryFile(index_bg_wasm);
	let blob = new Blob([bytes], { type: "application/wasm" });
	let url = URL.createObjectURL(blob);
	await init(url);
	greet("lucky day");
})("apps/greet/dist/index_bg.wasm")
	.then(console.log)
	.catch(console.error);

// init().then(console.log);
// invoke("greet", { name: "foo" }).then(console.log);
