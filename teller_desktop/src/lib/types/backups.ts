import type { WorldItem, WorldLevelData } from './worlds';

export interface RemoteBackup {
	remote_url: string;
	api_key: string;
}

export interface BackupSettings {
	schedule: string;
	auto_backup: boolean;
	enable_remote_backup: boolean;
	default_vaults: string[] | null;
	vaults: Record<string, string>;
	remote_vaults: Record<string, RemoteBackup>;
}

export interface BackupMetadata {
	entry: WorldItem;
	data: WorldLevelData;
}

export interface SnapshotInfo {
	created: number;
	size: number;
	path: string;
}

export interface Vault {
	[key: string]: {
		path: string;
		default: boolean;
	};
}
