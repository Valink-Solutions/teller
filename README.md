![ChunkVault Banner](assets/chunkvault-banner.jpg)

# Teller

Teller is a comprehensive Minecraft backup solution. It is designed to facilitate both local backups and interfacing with the ChunkVault Backend. This dual functionality allows users to maintain local copies of their Minecraft worlds while also leveraging the secure storage capabilities of the ChunkVault Backend.

## Table of Contents
- [Teller Library](#the-teller-library)
- [Teller_Desktop](#teller_desktop)
- [The Vault Backend](#the-vault-backend)
- [CommandBlock Library](#commandblock-library)
- [License](#license)
- [Contributing](#contributing)
- [Shoutout](#shoutout)

## The Teller Library

Teller is a comprehensive library designed for processing and backing up Minecraft worlds. It provides robust functionality to handle various aspects of world backup, including local backups, private server backups, and public ChunkVault backups. 

## Teller_Desktop

[![ChunkVault Desktop for MacOS](assets/chunkvault-desktop-marketing-screenshot.png)](https://docs.chunkvault.com)

Teller_Desktop is a Tauri/SvelteKit application that serves as the ChunkVault App - a user-friendly interface for backing up Minecraft worlds. It leverages the power of the Teller library to provide an all-in-one tool for Minecraft world backup. 

## The Vault Backend

While Teller and Teller_Desktop handle the backup process, [The Vault Backend](https://github.com/Valink-Solutions/vault) ensures secure storage of these world backups in the cloud.

## CommandBlock Library

The [CommandBlock](https://github.com/Valink-Solutions/commandblock) library is a rust library that provides functionality for parsing and executing Minecraft data such as NBT, LevelDB and more. It is used by Teller to parse Minecraft world data and extract the necessary information for backups. It is in development but its functionality is already being used by Teller.

## License

This project is licensed under the [GNU General Public License v3.0](LICENSE.txt).

## Contributing

Follow the [contributing guidelines](https://docs.chunkvault.com/teller/contributing/) to contribute to this project.

## Shoutout

A special shoutout to the development team behind [Modrinth's Theseus Minecraft Launcher](https://github.com/modrinth/theseus). We've modeled our application setup/structure after theirs, as it is both easy to follow and use. Their work has greatly influenced the development of Teller and we appreciate their contribution to the Minecraft community, as well as their support of open-source software.
