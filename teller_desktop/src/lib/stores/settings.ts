import { writable } from 'svelte/store';
import type { DirectorySettings } from '../types/config';
import type { BackupSettings, Vault } from '../types/backups';

export let directorySettings = writable<DirectorySettings>({ categories: {} });

export let backupSettings = writable<BackupSettings>({
	schedule: '0 0 * * * * *',
	auto_backup: false,
	enable_remote_backup: false,
	default_vaults: [],
	vaults: {},
	remote_vaults: {}
});

export let localVaults = writable<Vault>({});
