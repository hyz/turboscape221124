let codestr = 'console.log("hello jsc")';

async function imports(path) {
	let mod = window.__mods_[path];
	if (mod) return mod;
	let script = await window.__TAURI__.fs.readTextFile(path);
	const blob = new Blob([script], { type: "text/javascript" });
	const url = URL.createObjectURL(blob);
	return (window.__mods_[path] = await import(url));
}

async function requirejs(path) {
	let script = await window.__TAURI__.fs.readTextFile(path);
	const blob = new Blob([script], { type: "text/javascript" });
	const url = URL.createObjectURL(blob);
	return await import(url);
}
async function imports(buf, props) {
	const blob = new Blob([buf], props || { type: "text/javascript" });
	const url = URL.createObjectURL(blob);
	return await import(url);
}

let bg_wasm = await window.__TAURI__.fs.readBinaryFile("pkg/console_log_bg.wasm");
bg_wasm.buffer; // ArrayBuffer

let js = await requirejs("pkg/console_log.js");
js.initSync(bg_wasm);
// Uncaught RangeError: WebAssembly.Compile is disallowed on the main thread, if the buffer size is larger than 4KB. Use WebAssembly.compile, or compile on a worker thread.
//     at Module.initSync ...

// // Worker.js
import * as wasm from "./pkg/synchronous_instantiation.js";
self.onmessage = ({ data: bytes }) => {
	wasm.initSync(bytes);
	wasm.greet("greet from js to wasm");
};
self.postMessage({ type: "FETCH_WASM" });

(function () {
	const workerSrc =
		"self.onmessage = (msg) => { \
	 console.log(msg); \
	 const blob = new Blob([`console.log('blob ${msg}')`], { type: 'text/javascript' }); \
	 const url = URL.createObjectURL(blob); \
	 await import(url); \
	 self.postMessage({ type: 'FETCH_WASM' }); \
	 self.close(); \
};";
	const workerBlob = new Blob([workerSrc], { type: "text/javascript" });
	const workerUrl = URL.createObjectURL(workerBlob);
	const worker = new Worker(workerUrl, { type: "module" });
	worker.onmessage = ({ data, ...msg }) => {
		console.log(data);
		switch (data.type) {
			case "FETCH_WASM": {
				break;
			}
			default: {
				break;
			}
		}
	};
	worker.postMessage({ type: "hi", data: "from main to worker" });
	console.log("hi");
})();

// https://www.douyin.com/follow

// /******/ 	(() => {
// /******/ 		__webpack_require__.v = (exports, wasmModuleId, wasmModuleHash, importsObj) => {
// /******/ 			var req = fetch(__webpack_require__.p + "" + wasmModuleHash + ".module.wasm");
// /******/ 			if (typeof WebAssembly.instantiateStreaming === 'function') {
// /******/ 				return WebAssembly.instantiateStreaming(req, importsObj)
// /******/ 					.then((res) => (Object.assign(exports, res.instance.exports)));
// /******/ 			}
// /******/ 			return req
// /******/ 				.then((x) => (x.arrayBuffer()))
// /******/ 				.then((bytes) => (WebAssembly.instantiate(bytes, importsObj)))
// /******/ 				.then((res) => (Object.assign(exports, res.instance.exports)));
// /******/ 		};
// /******/ 	})();

// qcc, body > div > div.app-search > div.container.m-t > div.adsearch-list > nav > ul > li.active > a
// baidu, #page > div > strong > span

https: function baiduClick(elem, timeout) {
	// console.log(pageNumbs, elem);
	elem.click();
	//var f = ()=>{elem.click()}
	//setTimeout(f, timeout || (1500 + Math.random()*1000))
}
window.pageNumbs = [];

let baidu = { selector: "#page > div > strong" };
let qcc = { selector: "div.adsearch-list > nav > ul > li.active" };
//paging_hint(origin: &str, url: &str, index: i32, paging: tauri::State<'_, Database>)
// let props = { origin: document.location.href, url: window._lasturl_search, index: };
//window.__TAURI__.tauri.invoke('paging_exists', props).then(console.log);

function click(litem) {
	let a = litem.querySelector("a") || litem;
	a.click();
}
function pageNumb(litem) {
	let a = litem.querySelector("a") || litem;
	return parseInt(a.innerText);
}
function autoPaging(indexSelector, click) {
	let litem = document.querySelector(indexSelector);
	if (litem) {
		litem.scrollIntoView(false);
		let { invoke } = window.__TAURI__.tauri;
		let props = { origin: document.location.href, url: window._lasturl_search };
		let numb = pageNumb(litem);
		if (numb == 2 && !invoke("page_exists", { ...props, index: 1 })) {
			click(litem.prevElementSibling);
		} else {
			litem = litem.nextElementSibling;
			numb = pageNumb(litem);
			while (numb > 1 && numb <= 9999) {
				if (!invoke("page_exists", { ...props, index: numb })) {
					setTimeout(() => click(litem), 2000 + Math.random() * 5500);
					break;
				}
				litem = litem.nextElementSibling;
				numb = pageNumb(litem);
			}
			console.log("___ auto-paging", numb, litem);
		}
	}
}

// pageNumbs.push(parseInt(ahref.innerText));
// if (pageNumbs[pageNumbs.length - 1] == 1) {
// 	if (pageNumbs.length == 1) {
// 		baiduPaging(litem.nextElementSibling); //2
// 	} else {
// 		baiduPaging(litem.nextElementSibling.nextElementSibling); //3
// 	}
// } else if (pageNumbs[pageNumbs.length - 1] == 2) {
// 	baiduPaging(litem.previousElementSibling); //1
// } else {
// 	baiduPaging(litem.nextElementSibling);
// }
// // document.querySelector("#wrapper").scrollIntoView(false);
// document.body.scrollIntoView(false);
// let nextPage = () => {
// 	autoPaging(indexSelector, click);
// };
// setTimeout(nextPage, 1500 + Math.random() * 5000);
function simplify(obj0) {
	let found = [obj0];
	let traverse = (cpy, children, obj) => {
		const names = Object.keys(obj); //getOwnPropertyNames
		for (const name of names) {
			const val = obj[name];
			if (typeof val === "object" && val instanceof Object) {
				if (!found.find((x) => x === val)) {
					found.push(val);
					children.push([(cpy[name] = {}), val]); //traverse(val);
				}
			} else if (typeof val === "string" || typeof val === "number" || typeof val === "boolean") {
				cpy[name] = val;
			} else {
				//console.log(typeof val, name, val);
			}
			//if (val) {}
		}
	};
	let copy = {};
	let children = [];
	traverse(copy, children, obj0);
	while (children.length > 0) {
		const items = children;
		children = [];
		for (let [cpy, o] of items) {
			traverse(cpy, children, o);
		}
	}
	return copy; //traverse(o);
}

ah.hook({
	onloadend: (xhr, event) => {
		console.log("#on", event, xhr);
		return false;
	},
	onreadystatechange: (xhr, event) => {
		console.log("#on", event, xhr);
		return false;
	},
});

(function test(url) {
	var events = ["load", "loadend", "timeout", "error", "readystatechange", "abort"];

	var xhr = new XMLHttpRequest();
	events.forEach(function (e) {
		xhr["on" + e] = function (event) {
			console.log("on" + e, xhr.readyState, event);
		};
		xhr.addEventListener(e, function (event) {
			console.log(e, xhr.readyState, event);
		});
	});
	xhr.addEventListener("load", function (event) {
		console.log("response", xhr.response);
	});

	//setTimeout(()=>xhr.abort(),100)
	xhr.open("get", url, true);
	xhr.send();
})("http://127.0.0.1:1430/");

var xhr = new XMLHttpRequest();
xhr.open("get", "http://127.0.0.1:1430/index.html", true);
xhr.addEventListener("load", console.log);
xhr.send();

ah.proxy({
	onResponse: (response, handler) => {
		console.log(response);
		handler.next(response);
	},
});
