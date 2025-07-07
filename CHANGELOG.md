# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Search functionality to filter accounts by Mint address
- Safety confirmation dialog before processing operations
- Multi-platform release builds (Windows, macOS, Linux)
- Installation scripts for easy setup
- Comprehensive documentation in English and Chinese

### Changed
- Processing hotkey changed from `P` to `Ctrl+P` for safety
- Improved user interface with better status information
- Enhanced error handling and user feedback

### Security
- Added confirmation dialogs to prevent accidental operations
- Improved input validation

## [1.0.0] - 2024-XX-XX

### Added
- Initial release
- TUI interface for managing Solana token accounts
- Token burning functionality
- ATA account closure with SOL recovery
- Real-time account information display
- Keyboard navigation and selection
- Batch operations support

### Features
- 🔥 Burn SPL tokens
- 💰 Close ATA accounts and recover SOL rent
- 🖥️ Modern terminal interface based on ratatui
- ⚡ Real-time data via Solana RPC
- 🎯 Precise control over account selection

### Supported Platforms
- Linux x86_64
- Linux x86_64 (musl)
- Windows x86_64
- macOS x86_64 (Intel)
- macOS aarch64 (Apple Silicon)

---

## Release Notes Template

When creating a new release, use this template:

```markdown
## [X.Y.Z] - YYYY-MM-DD

### Added
- New features

### Changed
- Changes in existing functionality

### Deprecated
- Soon-to-be removed features

### Removed
- Now removed features

### Fixed
- Bug fixes

### Security
- Security improvements
```
