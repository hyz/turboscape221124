const WebviewWindow = window.__TAURI__.window.WebviewWindow;

const routeSelect = document.querySelector("#route");
const link = document.querySelector("#link");

routeSelect.addEventListener("change", (event) => {
	link.href = event.target.value;
});

document.querySelector("#go").addEventListener("click", () => {
	window.location.href = window.location.origin + "/" + routeSelect.value;
});

document.querySelector("#open-window").addEventListener("click", () => {
	new WebviewWindow(Math.random().toString().replace(".", ""), {
		url: routeSelect.value,
	});
});

document.querySelector("#baidu").addEventListener("click", () => {
	document.location.href = "https://baidu.com";
});

// (function play() {
// 	const { invoke, convertFileSrc } = window.__TAURI__.tauri;
// 	invoke("video_uri").then(([scheme, path]) => {
// 		const div = document.createElement("div");
// 		const source = document.createElement("source");
// 		source.type = "video/mp4";
// 		source.src = convertFileSrc(path, scheme);
// 		console.log(`${scheme} ${path}`, source.src);
// 		const video = document.createElement("video"); // document.getElementById("video_source");
// 		video.autoplay = true;
// 		video.controls = true;
// 		video.name = "media";
// 		video.appendChild(source);
// 		div.append(video);
// 		document.body.append(div);
// 		video.load();
// 	});
// 	//console.log("log X");
// 	//setTimeout(play, 9000);
// })();
