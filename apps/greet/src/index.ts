import { fs, tauri } from "tauri-apps-api"; //"tauri-apps-api-dist";
import init, { greet } from "../pkg";

// import "../pkg/index_bg.wasm";
// console.log(1, 2, 3, fs.exists, tauri.invoke, init, greet);

const _name__ = "greet";

fs.readBinaryFile(`apps/${_name__}/dist/index_bg.wasm`)
	.then(async (bytes) => {
		let blob = new Blob([bytes], { type: "application/wasm" });
		let url = URL.createObjectURL(blob);
		await init(url);
		greet("lucky day");
	})
	.catch(console.error);
