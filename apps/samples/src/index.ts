import { fs, tauri } from "tauri-apps-api"; //"tauri-apps-api-dist";
import init, { greet, create_dir, exists } from "../pkg";

// import "../pkg/index_bg.wasm";
// console.log(1, 2, 3, fs.exists, tauri.invoke, init, greet);

const _name__ = "samples";

fs.readBinaryFile(`apps/${_name__}/dist/index_bg.wasm`)
	.then(async (bytes) => {
		let blob = new Blob([bytes], { type: "application/wasm" });
		let url = URL.createObjectURL(blob);
		await init(url);
		let rsp = await greet("lucky day");
		console.log("___", rsp, await exists("foo/bar/wasm/rust"));
		await create_dir("dist/foo/bar/wasm/rust");
	})
	.catch(console.error);
