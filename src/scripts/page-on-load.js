(function () {
	let slims = (obj0) => {
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
	};
	window.__page_ = {};

	let onResponse = (r0, handler) => {
		let { invoke } = window.__TAURI__.tauri;
		let props = (function ({ config, headers, response, status }) {
			let { method, url, body, req } = (function (config) {
				let { method, url, headers, body, ...oth } = config;
				//let body = headers["body"];delete headers["body"];
				return { method, url, body, req: { ...headers, ...oth } };
			})(config);
			let { ctype, length, rsp } = (function (headers) {
				let ctype = headers["content-type"];
				let length = headers["content-length"];
				delete headers["content-type"];
				delete headers["content-length"];
				return { ctype, length, rsp: headers };
			})(headers);
			let origin = document.location.href;
			return { origin, method, url, body, status, content: response, ctype, length, headers: [req, rsp] };
		})(slims(r0));
		console.log("___slimed_", document.location.href, r0); //,r0
		invoke("sample", props).then(([jsf, bytes]) => {
			if (jsf && jsf.length > 0) {
				// if (uuid && uuid.length > 0) { window.__page_[uuid] = {}; }
				console.log("sample,collect ___", jsf, bytes.length, eval(jsf)(bytes));
			}
		});
		handler.next(r0);
	};
	// window.__mods_ = {
	// 	require: async function (path) {
	// 		let mod = window.__mods_[path];
	// 		if (mod) return mod;
	// 		let script = await invoke("get_script", { path });
	// 		const blob = new Blob([script], { type: "text/javascript" });
	// 		const url = URL.createObjectURL(blob);
	// 		return (window.__mods_[path] = await import(url));
	// 	},
	// };
	ah.proxy({ onResponse });
	window.__TAURI__.event.listen("jedi", (json) => {
		let { payload } = json;
		console.log("___ jedi ___", json);
		if (typeof payload === "string" || payload instanceof String) {
			let rv = window.eval(payload);
			// console.log("___ jedi eval:", rv);
		} else if (payload.action == "eval") {
			window.eval(payload.script);
		}
	});
	console.log("___ init ___");
})();
