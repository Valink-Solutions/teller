export interface DirectorySettings {
	categories: Record<string, VaultEntries>;
}

export interface VaultEntries {
	paths: Record<string, string>;
}
