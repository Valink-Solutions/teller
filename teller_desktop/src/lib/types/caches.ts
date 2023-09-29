import type { WorldItem, WorldLevelData } from './worlds';

export interface WorldCacheItem {
	name: string;
	data: WorldLevelData | null;
}

export interface WorldListCacheItem {
	category: string | null;
	instance: string;
	path: string;
	data: WorldItem[];
}
