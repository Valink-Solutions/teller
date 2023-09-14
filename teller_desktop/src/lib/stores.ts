import type { WorldLevelData } from '$lib/utils';
import { writable, type Writable } from 'svelte/store';

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

export interface WorldCacheItem {
	name: string;
	data: WorldLevelData | null;
}

export let worldCache = writable<WorldCacheItem[]>([]);

export function addToCache(item: WorldCacheItem) {
	worldCache.update((cache) => {
		cache.push(item);

		if (cache.length > 5) {
			cache.shift();
		}

		return cache;
	});
}

export function removeFromCache() {
	worldCache.update((cache) => {
		if (cache.length > 0) {
			cache.shift();
		}
		return cache;
	});
}
