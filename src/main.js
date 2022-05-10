import { platform } from '@tauri-apps/api/os';
import { appWindow } from '@tauri-apps/api/window';
import App from './App.svelte';

platform().then((platform) => {
	if (platform !== 'darwin') {
		document.getElementById('titlebar-buttons').style.setProperty('display', 'flex');
	}
});

document.getElementById('titlebar-minimize').addEventListener('click', () => {
	appWindow.minimize();
});
document.getElementById('titlebar-maximize').addEventListener('click', () => {
	appWindow.toggleMaximize();
});
document.getElementById('titlebar-close').addEventListener('click', () => {
	appWindow.close();
});

const app = new App({
	target: document.getElementById('app')
});

export default app;
