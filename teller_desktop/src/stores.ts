import { writable, type Writable } from 'svelte/store';

interface Directory {
	[name: string]: string | string[] | null;
}

export let directories: Writable<Directory> = writable({});

export let localDirs = writable<Directory>({});

export let currentDir = writable<string>('default');
