-- Add migration script here
CREATE TABLE worlds (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    name TEXT NOT NULL,
    seed INTEGER NOT NULL,
    current_version INTEGER NOT NULL,
    edition TEXT NOT NULL,
    created_at DATETIME,
    updated_at DATETIME
);

CREATE TABLE world_versions (
    id TEXT PRIMARY KEY,
    world_id TEXT NOT NULL,
    version INTEGER NOT NULL,
    backup_path TEXT NOT NULL,
    created_at DATETIME,
    difficulty TEXT NOT NULL,
    allow_cheats BOOLEAN NOT NULL,
    difficulty_locked BOOLEAN NOT NULL,
    spawn_x INTEGER NOT NULL,
    spawn_y INTEGER NOT NULL,
    spawn_z INTEGER NOT NULL,
    time INTEGER NOT NULL,
    weather TEXT NOT NULL,
    hardcore BOOLEAN NOT NULL,
    do_daylight_cycle BOOLEAN NOT NULL,
    do_mob_spawning BOOLEAN NOT NULL,
    do_weather_cycle BOOLEAN NOT NULL,
    keep_inventory BOOLEAN NOT NULL,
    size INTEGER NOT NULL,
    level_name TEXT NOT NULL,
    additional_data TEXT, -- JSON stored as text
    FOREIGN KEY(world_id) REFERENCES worlds(id)
);

CREATE TABLE game_rules (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    do_fire_tick BOOLEAN NOT NULL,
    mob_loot BOOLEAN NOT NULL,
    keep_inventory BOOLEAN NOT NULL,
    do_mob_spawning BOOLEAN NOT NULL,
    do_tile_drops BOOLEAN NOT NULL,
    command_block_output BOOLEAN NOT NULL,
    natural_regeneration BOOLEAN NOT NULL,
    do_daylight_cycle BOOLEAN NOT NULL,
    do_weather_cycle BOOLEAN NOT NULL,
    do_immediate_respawn BOOLEAN NOT NULL,
    drowning_damage BOOLEAN NOT NULL,
    fall_damage BOOLEAN NOT NULL,
    fire_damage BOOLEAN NOT NULL,
    do_insomnia BOOLEAN NOT NULL,
    invulnerable BOOLEAN NOT NULL,
    max_command_chain_length INTEGER NOT NULL,
    random_tick_speed INTEGER NOT NULL,
    reduced_debug_info BOOLEAN NOT NULL,
    send_command_feedback BOOLEAN NOT NULL,
    show_death_messages BOOLEAN NOT NULL,
    spawn_radius INTEGER NOT NULL,
    spectators_generate_chunks BOOLEAN NOT NULL
);

CREATE TABLE world_data (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    image TEXT NOT NULL,
    path TEXT NOT NULL,
    size INTEGER NOT NULL,
    last_played DATETIME,
    game_type TEXT
);

CREATE TABLE world_level_data (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    folder TEXT,
    icon TEXT,
    difficulty TEXT NOT NULL,
    game_engine TEXT NOT NULL,
    game_type TEXT NOT NULL,
    last_played DATETIME,
    size_on_disk INTEGER NOT NULL,
    players TEXT, -- JSON stored as text
    game_rules_id INTEGER,
    FOREIGN KEY(game_rules_id) REFERENCES game_rules(id)
);

CREATE TABLE config_categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    category TEXT NOT NULL
);

CREATE TABLE config_vault_entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    category_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    path TEXT NOT NULL,
    FOREIGN KEY(category_id) REFERENCES config_categories(id)
);

CREATE TABLE backup_vaults (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    backup_settings_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    path TEXT NOT NULL,
    FOREIGN KEY(backup_settings_id) REFERENCES backup_settings(id)
);

CREATE TABLE backup_remote_vaults (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    backup_settings_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    remote_url TEXT NOT NULL,
    api_key TEXT,
    FOREIGN KEY(backup_settings_id) REFERENCES backup_settings(id)
);

CREATE TABLE backup_settings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    schedule TEXT NOT NULL,
    auto_backup BOOLEAN NOT NULL,
    default_vaults TEXT -- JSON stored as text
);
