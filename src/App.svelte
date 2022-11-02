<script lang="ts">
	import { onMount } from 'svelte';
	import {
		isPermissionGranted,
		requestPermission,
		sendNotification,
		type Permission
	} from '@tauri-apps/api/notification';
	import './app.css';

	let permission: Permission;

	onMount(async () => {
		const permissionGranted = await isPermissionGranted();
		if (!permissionGranted) {
			permission = await requestPermission();
		}
	});

	const buttons = [
		{
			value: 10,
			display: '10 seconds'
		},
		{
			value: 900,
			display: '15 minutes',
			default: true
		},
		{
			value: 1800,
			display: '30 minutes'
		},
		{
			value: 3600,
			display: '60 minutes'
		}
	];

	let selectedButton = buttons.find((b) => b.default);
	let time = selectedButton.value;
	let timerStart = false;

	$: effect = {
		subscribe: () => {
			const interval = setInterval(async () => {
				if (timerStart) {
					if (time > 0) {
						time--;
					} else if (time === 0) {
						if (permission === 'granted') {
							sendNotification({
								title: 'Time’s up!',
								body: '🎉 Congrats on completing a session!'
							});
						}
						clearInterval(interval);
					}
				}
			}, 1000);
			return () => clearInterval(interval);
		}
	};
	$: $effect;

	const toggleTimer = () => (timerStart = !timerStart);

	const triggerResetDialog = async () => {
		time = selectedButton.value;
		timerStart = false;
	};
</script>

<main>
	<h1>Tomate</h1>
	<h2>
		{`${Math.floor(time / 60) < 10 ? `0${Math.floor(time / 60)}` : `${Math.floor(time / 60)}`}:${
			time % 60 < 10 ? `0${time % 60}` : time % 60
		}`}
	</h2>
	<p>
		<button on:click={toggleTimer}>{!timerStart ? 'Start' : 'Pause'}</button>
		<button on:click={triggerResetDialog}>Reset</button>
	</p>
	{#each buttons as button}
		<button
			on:click={() => {
				selectedButton = button;
				time = selectedButton.value;
				timerStart = true;
			}}>{button.display}</button
		>
	{/each}
</main>
