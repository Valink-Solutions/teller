import { writable } from 'svelte/store';
import type { CurrentDir, Directory } from '../types/navigation';

export let directories = writable<Directory>({});

export let localDirs = writable<Directory>({});

export let currentDir = writable<CurrentDir>({
	type: 'world',
	category: 'default',
	path: 'default'
});

export let currentVault = writable<string | null>();
export let activeItem = writable<CurrentDir | null>(null);
