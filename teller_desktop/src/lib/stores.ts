import type { BackupSettings, DirectorySettings, WorldItem, WorldLevelData } from '$lib/utils';
import { writable } from 'svelte/store';

interface Directory {
	[name: string]: string | string[] | null;
}

export interface CurrentDir {
	type?: string;
	category: string | null;
	path: string;
}
export let directories = writable<Directory>({});

export let localDirs = writable<Directory>({});

export let currentDir = writable<CurrentDir>({
	type: 'world',
	category: 'default',
	path: 'default'
});

export let directorySettings = writable<DirectorySettings>({ categories: {} });

export let backupSettings = writable<BackupSettings>({
	schedule: '0 0 * * * * *',
	auto_backup: false,
	enable_remote_backup: false,
	default_vaults: [],
	vaults: {},
	remote_vaults: {}
});

export interface Vault {
	[key: string]: {
		path: string;
		default: boolean;
	};
}

export let localVaults = writable<Vault>({});

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
	category: string | null;
	instance: string;
	path: string;
	data: WorldItem[];
}

export let worldListCache = writable<WorldListCacheItem>({
	category: '',
	instance: '',
	path: '',
	data: []
});

export const worldSortOption = writable({ option: 'size', direction: 'desc' });
