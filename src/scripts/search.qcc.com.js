(async function () {
	let selector = "div.adsearch-list > nav > ul > li.active";
	function pageNumb(lie) {
		let a = lie.querySelector("a") || lie;
		return parseInt(a.innerText);
	}
	function click(lie, uuid) {
		if (lie && window.__page_.scraping && uuid === window.__page_.uuid) {
			let a = lie.querySelector("a") || lie;
			a.click();
		}
	}
	async function exists(numb) {
		let { invoke } = window.__TAURI__.tauri;
		let hashindex = __TEMPLATE_hash_index__;
		// let keyword = __TEMPLATE_keyword__;
		return await invoke("numb_exists", { numb, hashindex });
		// invoke("numb_exists", { index, keyword, url, origin: document.location.href });
	}

	let lie = document.querySelector(selector);
	// lie && !lie.classList.contains("vip-page")
	if (lie) {
		lie.scrollIntoView(false);
		// let numb = pageNumb(lie);
		// if (numb == 2 && !(await exists(1))) {
		// 	click(lie.prevElementSibling);
		// } else {
		let sib = lie.nextElementSibling;
		let numb = pageNumb(sib);
		while (numb > 1 && numb <= 9999) {
			if (!(await exists(numb))) {
				let timeout = 5000 + Math.random() * 15000;
				let uuid = __TEMPLATE_uuid__;
				window.__page_.uuid = uuid;
				setTimeout(() => click(sib, uuid), timeout);
				console.log("___ next-page", timeout, numb, sib);
				break;
			}
			sib = sib.nextElementSibling;
			numb = pageNumb(sib);
		}
		// }
	}
})().then(console.log);
