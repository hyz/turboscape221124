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
	target: ["es2017"],
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
