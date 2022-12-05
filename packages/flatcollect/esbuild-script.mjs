import { existsSync, mkdirSync, copyFile, readFile } from "fs";
import { build } from "esbuild";
import path from "path";
// https://esbuild.github.io/plugins/#webassembly-plugin

existsSync("./dist/") || mkdirSync("./dist/");

build({
	plugins: [],
	loader: { ".wasm": "copy" },
	entryNames: "[name]",
	entryPoints: ["./pkg/index.js", "./pkg/index_bg.wasm"],
	outdir: "./dist",
	target: ["es2021"],
	format: "esm",
	minify: false,
	bundle: true,
	splitting: false,
	sourcemap: "external",
	treeShaking: true,
	logLevel: "info",
	mainFields: ["browser", "module", "main"], //
})
	.then(() => {
		// copyFile("./index.html", "./dist/index.html", (err) => {
		// 	if (err) throw err;
		// });
		// copyFile("./pkg/index_bg.wasm", "./dist/index_bg.wasm", (err) => {
		// 	if (err) throw err;
		// });
	})
	.catch((err) => {
		console.error(err);
		process.exit(1);
	});

//    "build": "tsup src/index.ts --format cjs --platform browser --out-dir ../../dist/greet ",

// fs.readBinaryFile(`apps/${_name__}/dist/index_bg.wasm`).then(async (bytes) => {
// console.log(`forward ___ ${JSON.parse('"www.baidu.com"')} ...`);
/*


(async function (bytes) {
	const { readBinaryFile } = window.__TAURI__.fs;
	const host = JSON.parse('"www.baidu.com"');
	const uuid = JSON.parse('"1dfc6eb0-3f58-4ba6-a9e1-48a10c98c9d9"');
	const index_js = JSON.parse('"packages/flatcollect/dist/index.js"');
	const index_bg = JSON.parse('"packages/flatcollect/dist/index_bg.wasm"');

	const src_js = await readBinaryFile(index_js);
	const blob_js = new Blob([src_js], { type: "text/javascript" });
	const url_js = URL.createObjectURL(blob_js);

	const { default: init, collect } = await import(url_js);

	const src_wasm = await readBinaryFile(index_bg);
	const blob_wasm = new Blob([src_wasm], { type: "application/wasm" });
	const url_wasm = URL.createObjectURL(blob_wasm);

	await init(url_wasm);
	console.log(`___ ${host} ...`, index_js, bytes);
	await collect(bytes);
})(input_bytes__)
	.then(console.log)
	.catch(console.error);
*/
// });
