import { writable } from 'svelte/store';
import { WorldCacheItem, WorldListCacheItem } from '../types/caches';

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

export let worldListCache = writable<WorldListCacheItem>({
	category: '',
	instance: '',
	path: '',
	data: []
});
