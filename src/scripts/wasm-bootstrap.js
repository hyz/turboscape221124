// const TEMPLATE_uuid__ = 1, TEMPLATE_host__ = "greet";
(async function (host, uuid) {
	// console.log(`wasm-bootstrap ___ ${host} ${uuid} ...`);
	const { exists, readBinaryFile } = window.__TAURI__.fs;
	// host = host.trim();if (host.startWith('www.')) { host = host.substring(4)}
	let stem = host;
	stem = `dist/${stem}/index`;
	if (!(await exists(`${stem}.js`))) {
		if (!host.includes(".")) {
			console.error(`${stem}.js not found`);
			return;
		}
		stem = host.substr(host.indexOf(".") + 1);
		stem = `dist/${stem}/index`;
		if (!(await exists(`${stem}.js`))) {
			console.error(`${stem}.js not found`);
			return;
		}
		// console.log(`wasm-bootstrap ___ ${host} ${stem} ...`);
	}
	//let stats = window.__page_[host];
	let srcjs = await readBinaryFile(`${stem}.js`);
	let srcwasm = await readBinaryFile(`${stem}_bg.wasm`);

	const blobjs = new Blob([srcjs], { type: "text/javascript" });
	const urljs = URL.createObjectURL(blobjs);

	const blobwasm = new Blob([srcwasm], { type: "application/wasm" });
	const urlwasm = URL.createObjectURL(blobwasm);

	const { default: init, play } = await import(urljs); //(urlwasm);
	await init(urlwasm);
	await play();

	console.log(`___ ${host} ${stem} ...`);
})(__TEMPLATE_host__, __TEMPLATE_uuid__)
	.then(console.log)
	.catch(console.error);
