//@ts-ignore
import init, { greet } from "../pkg";
import { invoke } from "./tauri-apps/tauri";

// Don't worry if vscode told you can't find my-crate
// It's because you're using a local crate
// after yarn dev, wasm-pack plugin will install my-crate for you

// init().then(() => {
// 	console.log("init wasm-pack");
// 	greet("from vite!");
// });

// export * from "/crates/greet";

export function sayHi() {
	console.log("hi");
}

export async function greet(name: String) {
	return invoke("greet", { name });
}

// import * as x from "@tauri-apps/api";

// eslint-disable-next-line turbo/no-undeclared-env-vars
// const port = process.env.PORT || 5001;
// const server = createServer();

// server.listen(port, () => {
// 	sayHi();
// });

init().then(console.log);
// invoke("greet", { name: "foo" }).then(console.log);

// "scripts": {
// 	"build": "tsup index.ts --format esm --keep-names --dts",
// 	"clean": "rm -rf dist",
// 	"dev": "tsup index.ts --format cjs --watch --dts --external react",
// 	"lint": "TIMING=1 eslint \"src/**/*.ts*\""
// },
