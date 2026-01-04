# Changelog

All notable changes to the Laravel extension for Zed will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.1] - 2025-01-04

### Fixed
- Replace placeholder URLs with correct GitHub username in documentation

## [0.2.0] - 2025-11-17

### 🎉 Major Update: Complete Laravel Extension

Transformed from Blade-only extension to complete Laravel development suite.

### Added
- **PHP Language Support** - Full tree-sitter-php grammar integration
- **120+ Code Snippets** - Blade (40+), PHP Laravel (50+), Livewire 3 (30+)
- **Laravel Pint Integration** - Auto-formatting for PHP files
- **Artisan Tasks Template** - 25+ pre-configured tasks (optional)
- **Multi-Language Support** - Blade + PHP with proper LSP coordination

### Changed
- **Extension renamed** from `blade-enhanced` to `laravel`
- **Repository renamed** from `zed-blade-enhanced` to `zed-laravel`
- Expanded description to reflect comprehensive Laravel support
- Improved documentation with usage examples

### Technical
- Updated Cargo package name to `zed-laravel`
- Reorganized snippets into language-specific folders
- Added PHP grammar (tree-sitter-php v0.23.9)
- Enhanced Intelephense configuration for Laravel
- Created templates directory for optional configurations

## [0.1.0] - 2025-11-17

### Initial Release - Blade Enhanced

Initial release focused on Blade template support.

### Added
- **Go to Definition** for `@include` directives
- **Autocomplete** for Blade view paths
- **Laravel Language Server** integration (auto-downloaded)
- **Intelephense** support for PHP code in Blade
- **Syntax Highlighting** via tree-sitter-blade v0.14.0
- **Diagnostics** for missing views
- Cross-platform support (macOS, Linux, Windows)

### Technical
- Rust-based WASM extension (160KB compiled)
- Automatic laravel-ls binary download
- Multi-LSP coordination (laravel-ls + Intelephense)
- Tree-sitter-blade grammar v0.14.0

---

## Migration from 0.1.0 to 0.2.0

If you installed version 0.1.0 (`blade-enhanced`):

1. Uninstall old extension in Zed
2. Install new `laravel` extension
3. All previous Blade features still work
4. New PHP, snippets, and formatting features available automatically

No configuration changes needed!
