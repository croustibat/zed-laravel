# Laravel Extension for Zed Editor

**Complete Laravel development experience for Zed Editor** - Blade templates, PHP, Pint formatting, IntelliSense, snippets, and more.

[![Zed](https://img.shields.io/badge/Zed-Extension-blue)](https://zed.dev)
[![Laravel](https://img.shields.io/badge/Laravel-11.x-FF2D20?logo=laravel)](https://laravel.com)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

### Blade Template Support
- **Go to Definition** - Ctrl/Cmd+Click on `@include('view.name')` to jump to files
- **Autocomplete** - Smart suggestions for view paths, routes, and config
- **Syntax Highlighting** - Enhanced highlighting via tree-sitter-blade
- **Diagnostics** - Warnings for non-existent views and syntax errors

### PHP Development
- **IntelliSense** - Full PHP autocompletion via Intelephense
- **Laravel Pint** - Automatic code formatting on save
- **Syntax Highlighting** - Via tree-sitter-php
- **Laravel-specific** - Facades, helpers, and Laravel patterns support

### Code Snippets (120+ included)

| Category | Examples |
|----------|----------|
| **Blade (40+)** | `@if`, `@foreach`, `@extends`, `@section`, `@auth`, `@csrf` |
| **PHP Laravel (50+)** | `route`, `controller`, `model`, `migration`, `hasMany`, `belongsTo` |
| **Livewire 3 (30+)** | `livewire-component`, `wire:model`, `wire:click`, `livewire-form` |

### Language Servers

- **Laravel LS** - Auto-downloaded, handles Laravel-specific features
- **Intelephense** - PHP language server for code intelligence

## Installation

### Via Zed Extensions (Recommended)

1. Open Zed Editor
2. Press `Cmd+Shift+P` / `Ctrl+Shift+P`
3. Type "extensions" and select "zed: extensions"
4. Search for **"Laravel"**
5. Click **Install**

### Manual Installation

```bash
git clone https://github.com/croustibat/zed-laravel.git
cd zed-laravel
./build.sh
```

Then in Zed: `Cmd+Shift+P` → "zed: install dev extension" → Select the directory

## Usage

### Go to Definition

Ctrl/Cmd+Click on Blade directives:

```blade
@include('components.header')  {{-- Jump to file --}}
@extends('layouts.app')
```

### Snippets

Type prefix + Tab:

```blade
@if<Tab>       →  @if/$endif block
@foreach<Tab>  →  @foreach loop
```

```php
route<Tab>     →  Route::get('uri', [Controller::class, 'method']);
hasMany<Tab>   →  hasMany relationship method
```

### Laravel Pint Formatting

- **Auto**: Enable format on save in settings
- **Manual**: `Cmd+Shift+I` / `Ctrl+Shift+I`

## Configuration

### Recommended Settings

Add to your Zed `settings.json`:

```json
{
  "languages": {
    "Blade": {
      "tab_size": 4
    },
    "PHP": {
      "tab_size": 4,
      "format_on_save": "on"
    }
  }
}
```

### Prerequisites

```bash
npm install -g intelephense
composer require laravel/pint --dev
```

## Optional: Artisan Tasks

Copy the tasks template to your project for quick Artisan command access:

```bash
mkdir -p .zed
curl -o .zed/tasks.json https://raw.githubusercontent.com/croustibat/zed-laravel/main/templates/tasks.json
```

Access with `Cmd+Shift+T` / `Ctrl+Shift+T`:
- Artisan Serve, Migrate, Tinker
- Pint Format, PHPUnit/Pest
- NPM dev/build, Composer install

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Laravel LS not starting | Check `View` → `Language Server Logs`. Install manually: `go install github.com/laravel-ls/laravel-ls/cmd/laravel-ls@latest` |
| Pint not working | Ensure `./vendor/bin/pint` exists. Run `composer require laravel/pint --dev` |
| Go to Definition fails | Verify Laravel project structure. Restart language server via Command Palette |

## Related Projects

**[zed-for-laravel](https://github.com/croustibat/zed-for-laravel)** - Complete Zed configuration kit with 200+ snippets including Pest, Filament, Inertia.js and Volt.

*Use both together for the ultimate Laravel development experience!*

## Contributing

Contributions welcome! Please feel free to submit a Pull Request.

## Credits

- [EmranMR](https://github.com/EmranMR) - tree-sitter-blade
- [laravel-ls](https://github.com/laravel-ls/laravel-ls) - Laravel Language Server
- [Zed](https://zed.dev) - The high-performance code editor

## License

MIT - see [LICENSE](LICENSE)

---

**Made with ❤️ for the Laravel community**
