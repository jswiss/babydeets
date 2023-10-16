export const ssr = false;
export const prerender = false;

import { invoke } from '@tauri-apps/api';

async function createBabyPhoto() {
	const pathname = window.location.pathname;
	const name = pathname.slice(pathname.lastIndexOf('/') + 1);
	console.log({ name });

	// const photo = new Uint8Array(await file.arrayBuffer());

	// console.log('photo', photo);
	// await invoke('create_baby_photo', { name, photo });
}
