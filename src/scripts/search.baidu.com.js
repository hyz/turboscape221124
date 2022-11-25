(async function () {
	let selector = "#page > div > strong";
	function pageNumb(lie) {
		let a = lie.querySelector("a") || lie;
		return parseInt(a.innerText);
	}
	function click(lie, uuid) {
		if (window.__page_.scraping && uuid === window.__page_.uuid) {
			let a = lie.querySelector("a") || lie;
			a.click();
		}
	}
	async function exists(numb) {
		let hashindex = __TEMPLATE_hash_index__;
		let { invoke } = window.__TAURI__.tauri;
		return await invoke("numb_exists", { numb, hashindex }); //.then(() => f());
	}

	let lie = document.querySelector(selector);
	if (lie && lie.nextElementSibling) {
		lie.scrollIntoView(false);
		let numb = pageNumb(lie);
		let found = await exists(numb);
		let timeout = 2000 + Math.random() * 6000;
		let uuid = __TEMPLATE_uuid__;
		window.__page_.uuid = uuid;
		setTimeout(() => click(lie.nextElementSibling, uuid), timeout);
		console.log("___ next-page", timeout, numb, found, lie);
	}
})().then(console.log);
