import { appWindow } from '@tauri-apps/api/window'
import App from './App.svelte'

document
  .getElementById('titlebar-minimize')
  .addEventListener('click', () => appWindow.minimize())
document
  .getElementById('titlebar-maximize')
  .addEventListener('click', () => appWindow.toggleMaximize())
document
  .getElementById('titlebar-close')
  .addEventListener('click', () => appWindow.close())

const app = new App({
  target: document.getElementById('app')
})

export default app
