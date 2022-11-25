const __TEMPLATE_uuid__ = 1,
	__TEMPLATE_host__ = 1;
await (async function (uuid, host) {
	const workerSrcFunc = async function () {
		// const code = await window.__TAURI__.fs.readTextFile("src-wasm/console-log.js");
		// const blob = new Blob([code], { type: "text/javascript" });
		// const bloburl = URL.createObjectURL(blob);
		// const wasm = await import(bloburl);
		// const bg_wasm = await window.__TAURI__.fs.readBinaryFile("pkg/console_log_bg.wasm");

		self.onmessage = async ({ data, ...msg }) => {
			const blob = new Blob([data], { type: "text/javascript" });
			let wasm = await import(URL.createObjectURL(blob));
			let _x: Module = wasm;
			// console.log("import URL.createObjectURL", wasm, typeof wasm);
			// self.postMessage(wasm, [wasm]);
			self.postMessage(wasm, [wasm]);
			self.close();
		};
		// self.postMessage({ kind: "WASM", wasm }, [wasm]);
	};
	const workerSrc = "(" + workerSrcFunc.toString() + ")()";
	const workerBlob = new Blob([workerSrc], { type: "text/javascript" });
	const workerUrl = URL.createObjectURL(workerBlob);
	const worker = new Worker(workerUrl, { type: "module" });
	worker.onmessage = async ({ data, ...msg }) => {
		console.log("main onmessage rcpt:", data, msg);
		// await data.greet();
		// switch (data.kind) {
		// 	case "WASM": {
		// 		await data.wasm.greet();
		// 		break;
		// 	}
		// 	default: {
		// 		break;
		// 	}
		// }
	};
	let data = await window.__TAURI__.fs.readBinaryFile("src-wasm/console-log.js");
	worker.postMessage(data, [data.buffer]);
	// const blob = new Blob([data], { type: "text/javascript" });
	// console.log("typeof data", typeof blob, blob);
	// worker.postMessage(blob, [new Date(), blob]);
	// console.log("main postMessage, first");

	// const src = "console.log(123,456,99999);";
	// const blob = new Blob([src], { type: "text/javascript" });
	// const url = URL.createObjectURL(blob);
	// await import(url); //.then(console.log).catch(console.error);
})(__TEMPLATE_uuid__, __TEMPLATE_host__)
	.then(console.log)
	.catch(console.error);

// (async function (uuid, host) {
// 	const code = await window.__TAURI__.fs.readTextFile(`src-wasm/pkg/${host}}.js`);
// 	const bg_wasm = await window.__TAURI__.fs.readBinaryFile("pkg/console_log_bg.wasm");
// 	const blob = new Blob([code], { type: "text/javascript" });
// 	const bloburl = URL.createObjectURL(blob);
// 	const mod = await import(bloburl);

// 	// js.initSync(bg_wasm.buffer);
// })(__TEMPLATE_uuid__, __TEMPLATE_host__);
