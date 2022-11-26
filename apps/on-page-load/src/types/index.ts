export {};

declare global {
	interface Window {
		__TAURI__: any; //async (fp:String) => String;
		__page_: any;
		ah: any;
	}
}
