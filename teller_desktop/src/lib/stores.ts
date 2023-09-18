import type { DirectorySettings, WorldItem, WorldLevelData } from '$lib/utils';
import { writable } from 'svelte/store';

interface Directory {
	[name: string]: string | string[] | null;
}

export interface CurrentDir {
	category: string | null;
	path: string;
}
export let directories = writable<Directory>({});

export let localDirs = writable<Directory>({});

export let currentDir = writable<CurrentDir>({ category: 'default', path: 'default' });

export let directorySettings = writable<DirectorySettings>({ categories: {} });

export interface WorldCacheItem {
	name: string;
	data: WorldLevelData | null;
}

export let worldCache = writable<WorldCacheItem[]>([]);

export function addToWorldCache(item: WorldCacheItem) {
	worldCache.update((cache) => {
		cache.push(item);

		if (cache.length > 5) {
			cache.shift();
		}

		return cache;
	});
}

export function removeFromWorldCache() {
	worldCache.update((cache) => {
		if (cache.length > 0) {
			cache.shift();
		}
		return cache;
	});
}

export interface WorldListCacheItem {
	category: string;
	instance: string;
	data: WorldItem[];
}

export let worldListCache = writable<WorldListCacheItem>({
	category: '',
	instance: '',
	data: []
});
