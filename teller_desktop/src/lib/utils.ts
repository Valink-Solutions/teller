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

export interface WorldLevelData {
	name: string;
	folder: string | null;
	icon: string | null;
	difficulty: string;
	game_engine: string;
	game_type: string;
	last_played: string | null;
	size_on_disk: number;
	players: any[];
	game_rules: GameRules | null;
}

export interface GameRules {
	do_fire_tick: boolean;
	mob_loot: boolean;
	keep_inventory: boolean;
	do_mob_spawning: boolean;
	do_tile_drops: boolean;
	command_block_output: boolean;
	natural_regeneration: boolean;
	do_daylight_cycle: boolean;
	do_weather_cycle: boolean;
	do_immediate_respawn: boolean;
	drowning_damage: boolean;
	fall_damage: boolean;
	fire_damage: boolean;
	do_insomnia: boolean;
	invulnerable: boolean;
	max_command_chain_length: number;
	random_tick_speed: number;
	reduced_debug_info: boolean;
	send_command_feedback: boolean;
	show_death_messages: boolean;
	spawn_radius: number;
	spectators_generate_chunks: boolean;
}

export interface DirectorySettings {
	categories: Record<string, VaultEntries>;
}

export interface VaultEntries {
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
