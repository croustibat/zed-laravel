# Laravel Extension for Zed Editor

**Complete Laravel development experience for Zed Editor** - Blade templates, PHP, Pint formatting, IntelliSense, snippets, and more.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## ✨ Features

### 🎨 Blade Template Support
- **Go to Definition** - Ctrl/Cmd+Click on `@include('view.name')` to jump to files
- **Autocomplete** - Smart suggestions for view paths, routes, and config
- **Syntax Highlighting** - Enhanced highlighting via tree-sitter-blade v0.14.0
- **Diagnostics** - Warnings for non-existent views and syntax errors

### 🐘 PHP Development
- **IntelliSense** - Full PHP autocompletion via Intelephense
- **Laravel Pint** - Automatic code formatting (format on save or Cmd/Ctrl+Shift+I)
- **Syntax Highlighting** - Via tree-sitter-php
- **Laravel-specific** - Facades, helpers, and Laravel patterns support

### 📝 Code Snippets (120+ snippets included)

**Blade Templates (40+ snippets)**:
- Control structures: `@if`, `@foreach`, `@for`, `@while`
- Templates: `@extends`, `@section`, `@yield`, `@include`
- Components: `component`, `slot`, `@props`
- Auth: `@auth`, `@guest`, `@can`
- Helpers: `@csrf`, `@method`, `@error`

**PHP Laravel (50+ snippets)**:
- Routes: `route`, `routepost`, `routeres`
- Eloquent: `hasMany`, `belongsTo`, `belongsToMany`
- Artisan: Controllers, Models, Migrations
- Jobs, Events, Listeners, Middleware, Policies

**Livewire 3 (30+ snippets)**:
- Components, Properties, Lifecycle
- `wire:model`, `wire:click`, `wire:loading`
- Forms, file uploads, pagination

### 🚀 Language Servers

- **Laravel LS** - Auto-downloaded, handles Laravel-specific features
- **Intelephense** - PHP language server for code intelligence

## 📦 Installation

### Via Zed Extensions (Recommended - Coming Soon)

1. Open Zed Editor
2. Press `Cmd+Shift+P` / `Ctrl+Shift+P`
3. Type "extensions" → "zed: extensions"
4. Search for "Laravel"
5. Click "Install"

### Manual Installation (For Now)

```bash
git clone https://github.com/croustibat/zed-laravel.git
cd zed-laravel
cargo build --target wasm32-wasip1 --release
```

Then in Zed:
- `Cmd+Shift+P` → "install dev extension"
- Select the `zed-laravel` directory

## 🎯 Usage

### Go to Definition

Ctrl/Cmd+Click on Blade directives:

```blade
@include('components.header')  {{-- Jump to file --}}
@extends('layouts.app')
```

### Autocomplete

Start typing for suggestions:

```blade
@include('  {{-- Shows all views --}}
```

### Laravel Pint Formatting

- **Auto**: Enable format on save
- **Manual**: `Cmd+Shift+I` / `Ctrl+Shift+I`

### Snippets

Type prefix + Tab:

```blade
@if<Tab>      →  if/endif block
@foreach<Tab>  →  foreach loop
```

```php
route<Tab>   →  Route definition
hasMany<Tab>  →  Eloquent relationship
```

## 📋 Optional: Artisan Tasks

Copy tasks template to your project:

```bash
mkdir -p .zed
cp path/to/zed-laravel/templates/tasks.json .zed/tasks.json
```

Access with `Cmd+Shift+T` / `Ctrl+Shift+T`:
- Artisan Serve, Migrate, Tinker
- Pint Format, PHPUnit/Pest
- NPM dev/build, Composer install

See `templates/README.md` for details.

## ⚙️ Configuration

### Recommended Zed Settings

```json
{
  "languages": {
    "Blade": {
      "tab_size": 4,
      "format_on_save": false
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
# Install Intelephense globally (recommended)
npm install -g intelephense

# Laravel Pint (in your project)
composer require laravel/pint --dev
```

## 🛠️ Development

```bash
# Build extension
./build.sh

# Or manually
cargo build --target wasm32-wasip1 --release
```

See `TESTING.md` for testing procedures.

## 🐛 Troubleshooting

**Laravel LS not starting?**
- Check logs: `View` → `Language Server Logs`
- Install manually: `go install github.com/laravel-ls/laravel-ls/cmd/laravel-ls@latest`

**Pint not working?**
- Ensure `./vendor/bin/pint` exists
- Check `composer require laravel/pint --dev`

**Go to Definition fails?**
- Verify Laravel project structure
- Restart language server: `Cmd+Shift+P` → "restart language server"

## 📚 Resources

- [Zed Editor](https://zed.dev)
- [Laravel Docs](https://laravel.com/docs)
- [Laravel Pint](https://laravel.com/docs/pint)
- [laravel-ls](https://github.com/laravel-ls/laravel-ls)

## 🤝 Contributing

Contributions welcome! Fork, branch, commit, push, PR.

## 📄 License

MIT - see [LICENSE](LICENSE)

## 🙏 Credits

- **EmranMR** - tree-sitter-blade
- **laravel-ls team** - Laravel Language Server
- **Zed team** - Zed Editor
- **croustibat** - Extension integration

## 🌟 Related

- [zed-for-laravel](https://github.com/croustibat/zed-for-laravel) - Themes & settings kit

*Use together for complete Laravel experience!*

## 📞 Support

- 🐛 [Issues](https://github.com/croustibat/zed-laravel/issues)
- 💬 [Discussions](https://github.com/croustibat/zed-laravel/discussions)
- ⭐ Star if useful!

---

**Made with ❤️ for the Laravel community**
*Generated with [Claude Code](https://claude.com/claude-code)*
