export function formatBytes(bytes: number): string {
	if (bytes < 0) {
		return '0 bytes';
	} else if (bytes < 1024) {
		return bytes + ' bytes';
	} else if (bytes < 1024 * 1024) {
		return (bytes / 1024).toFixed(2) + ' KB';
	} else if (bytes < 1024 * 1024 * 1024) {
		return (bytes / (1024 * 1024)).toFixed(2) + ' MB';
	} else {
		return (bytes / (1024 * 1024 * 1024)).toFixed(2) + ' GB';
	}
}

export interface WorldItem {
	id: string;
	name: string;
	path: string;
	image: string;
	size: number;
}

export interface DirectorySettings {
	paths: Record<string, string>;
}

export function parseGameType(gameType: number) {
	switch (gameType) {
		case 0:
			return 'Survival';
		case 1:
			return 'Creative';
		case 2:
			return 'Adventure';
		case 3:
			return 'Spectator';
		default:
			return 'Unknown';
	}
}

export function parseDifficulty(difficulty: number) {
	switch (difficulty) {
		case 0:
			return 'Peaceful';
		case 1:
			return 'Easy';
		case 2:
			return 'Normal';
		case 3:
			return 'Hard';
		default:
			return 'Unknown';
	}
}
