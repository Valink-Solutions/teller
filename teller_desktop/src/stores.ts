import { writable, type Writable } from 'svelte/store';

interface Directory {
	[name: string]: string | string[] | null;
}

export let directories: Writable<Directory> = writable({});

export let localDirs = writable<Directory>({});

export interface CurrentDir {
	category: string | null;
	path: string;
}

export let currentDir = writable<CurrentDir>({ category: 'default', path: 'default' });
