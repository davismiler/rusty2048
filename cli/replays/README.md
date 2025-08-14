# Replays Folder

This folder contains saved game replay files for the Rusty2048 CLI version.

## File Format

Replay files are saved as JSON files with the naming convention:
- `replay_[timestamp].json`

## File Structure

Each replay file contains:
- Game configuration
- Initial board state
- Complete move history
- Final game statistics
- Metadata (creation time, player info, etc.)

## Usage

Replay files can be:
1. **Loaded and played back** through the CLI replay system
2. **Shared** with other players
3. **Analyzed** for game strategy improvement

## Notes

- This folder is automatically created when the replay system is first used
- Replay files are excluded from version control (see .gitignore)
- Files can be safely deleted if no longer needed
