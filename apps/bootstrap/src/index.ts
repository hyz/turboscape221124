// import { fs } from "tauri-apps-api";

// (async function (index_js) {
// 	eval(await window.__TAURI__.fs.readTextFile(index_js));
// })("apps/greet/dist/index.js")
// 	.then(console.log)
// 	.catch(console.error);
window.__TAURI__.fs.readTextFile("apps/greet/dist/index.js").then(eval);
