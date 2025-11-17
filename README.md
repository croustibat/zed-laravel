# Laravel Blade Enhanced for Zed

Enhanced Laravel Blade support for [Zed Editor](https://zed.dev) with Go to Definition, autocomplete, and improved syntax highlighting.

## Features

✨ **Go to Definition** - Ctrl+Click on `@include('view.name')` to jump to the file
🔍 **Autocomplete** - Smart suggestions for view paths as you type
🎨 **Enhanced Syntax Highlighting** - Improved highlighting for all Blade directives
⚠️ **Diagnostics** - Warnings for non-existent views and syntax errors
🚀 **Laravel LSP Integration** - Powered by [laravel-ls](https://github.com/laravel-ls/laravel-ls)
🧠 **PHP IntelliSense** - Full PHP support via Intelephense for embedded PHP code

## Installation

### Prerequisites

- [Zed Editor](https://zed.dev) installed
- A Laravel project

### Automatic Installation (Recommended)

1. Open Zed Editor
2. Press `Cmd+Shift+P` (Mac) or `Ctrl+Shift+P` (Linux/Windows)
3. Type "extensions" and select "zed: extensions"
4. Search for "Laravel Blade Enhanced"
5. Click "Install"

### Manual Installation (Development)

1. Clone this repository:
```bash
git clone https://github.com/croustibat/zed-blade-enhanced.git
cd zed-blade-enhanced
```

2. Build the extension:
```bash
cargo build --target wasm32-wasip1 --release
```

3. Install as dev extension in Zed:
   - Open Zed
   - Press `Cmd+Shift+P` / `Ctrl+Shift+P`
   - Type "install dev extension"
   - Select the `zed-blade-enhanced` directory

## Usage

### Go to Definition

Simply **Ctrl+Click** (or **Cmd+Click** on Mac) on any Blade include directive:

```blade
@include('components.header')  {{-- Ctrl+Click to jump to header.blade.php --}}
@include('emails.welcome', ['user' => $user])
```

### Autocomplete

Start typing a view path and get intelligent suggestions:

```blade
@include('  {{-- Start typing, get autocomplete suggestions --}}
```

### Supported Directives

The extension provides enhanced support for:

- `@include` - Include partials
- `@extends` - Template inheritance
- `@component` - Blade components
- `@each` - Iterate over arrays
- All standard Blade directives with proper syntax highlighting

## Configuration

### Laravel Language Server

The extension automatically downloads and configures [laravel-ls](https://github.com/laravel-ls/laravel-ls). No additional configuration needed!

### Intelephense

For PHP code within Blade templates, the extension uses Intelephense. Make sure you have it installed:

```bash
npm install -g intelephense
```

### Customizing Settings

Add to your Zed `settings.json`:

```json
{
  "languages": {
    "Blade": {
      "tab_size": 4,
      "format_on_save": false,
      "preferred_line_length": 120
    }
  }
}
```

## How It Works

This extension combines multiple technologies:

1. **Tree-sitter-blade** - Fast, accurate syntax parsing
2. **Laravel Language Server** - Provides Go to Definition, autocomplete, and diagnostics
3. **Intelephense** - Handles PHP code embedded in Blade templates
4. **Zed Extension API** - Seamless integration with Zed Editor

## Troubleshooting

### Extension not working?

1. Check that you have a Laravel project open
2. Verify that `laravel-ls` is running:
   - Open Zed's language server output panel
   - Look for "Laravel Language Server" status

### Go to Definition not working?

Make sure your view files are in standard Laravel locations:
- `resources/views/`
- `vendor/{package}/resources/views/` (for packages)

### Autocomplete not showing?

Ensure your Laravel project structure is correct and the extension can find your views directory.

## Development

### Building from Source

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-wasip1

# Build the extension
cargo build --target wasm32-wasip1 --release
```

### Testing

```bash
# Run with debug output
zed --foreground
```

### Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## Related Projects

- [zed-for-laravel](https://github.com/croustibat/zed-for-laravel) - Complete Zed configuration kit for Laravel
- [tree-sitter-blade](https://github.com/EmranMR/tree-sitter-blade) - Blade grammar for tree-sitter
- [laravel-ls](https://github.com/laravel-ls/laravel-ls) - Language server for Laravel

## License

MIT License - see [LICENSE](LICENSE) file for details

## Credits

- Created by [croustibat](https://github.com/croustibat)
- Powered by [laravel-ls](https://github.com/laravel-ls/laravel-ls)
- Uses [tree-sitter-blade](https://github.com/EmranMR/tree-sitter-blade) grammar
- Built for [Zed Editor](https://zed.dev)

## Support

- 🐛 [Report Issues](https://github.com/croustibat/zed-blade-enhanced/issues)
- 💬 [Discussions](https://github.com/croustibat/zed-blade-enhanced/discussions)
- ⭐ Star this repo if you find it useful!
