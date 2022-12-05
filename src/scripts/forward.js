(bytes) => {
	console.log(`forward ___ ${__TEMPLATE_host__} ...`);
	(async function (bytes) {
		const { readBinaryFile } = window.__TAURI__.fs;
		const host = __TEMPLATE_host__;
		const uuid = __TEMPLATE_uuid__;
		const index_js = __TEMPLATE_index__;
		const index_bg = __TEMPLATE_index_bg__;

		const src_js = await readBinaryFile(index_js);
		const blob_js = new Blob([src_js], { type: "text/javascript" });
		const url_js = URL.createObjectURL(blob_js);

		const { default: init, collect } = await import(url_js);

		const src_wasm = await readBinaryFile(index_bg);
		const blob_wasm = new Blob([src_wasm], { type: "application/wasm" });
		const url_wasm = URL.createObjectURL(blob_wasm);

		window.__page_.uuid = uuid;
		await init(url_wasm);
		console.log(`___ ${host} ${uuid} ...`, index_js, bytes);
		await collect(bytes);
	})(bytes)
		.then(console.log)
		.catch(console.error);
};
