# Guide de test - Laravel Blade Enhanced

## Installation de l'extension en mode développement

### Prérequis

- Zed Editor installé
- Un projet Laravel pour les tests

### Étape 1 : Installer l'extension en mode dev

1. Ouvrez Zed Editor
2. Appuyez sur `Cmd+Shift+P` (Mac) ou `Ctrl+Shift+P` (Linux/Windows)
3. Tapez "install dev extension"
4. Sélectionnez "zed: install dev extension"
5. Naviguez vers le dossier `zed-blade-enhanced`
6. L'extension sera installée et activée automatiquement

### Étape 2 : Vérifier l'installation

1. Ouvrez un fichier `.blade.php` dans votre projet Laravel
2. Vérifiez que la syntaxe est bien colorée
3. Ouvrez le panneau "Language Server" pour voir si `laravel-ls` est actif :
   - `View` → `Language Server Logs` ou `Cmd+Shift+Y`

## Tests à effectuer

### Test 1 : Coloration syntaxique

Ouvrez un fichier Blade et vérifiez que les directives sont bien colorées :

```blade
@extends('layouts.app')

@section('content')
    <div class="container">
        @include('components.header')

        @foreach($items as $item)
            <div>{{ $item->name }}</div>
        @endforeach
    </div>
@endsection
```

**Résultat attendu :**
- Les directives Blade (`@extends`, `@include`, etc.) sont colorées
- Le HTML est coloré correctement
- Les variables PHP sont mises en évidence

### Test 2 : Go to Definition sur @include

1. Créez une structure de vues Laravel :
```
resources/views/
├── layouts/
│   └── app.blade.php
├── components/
│   └── header.blade.php
└── home.blade.php
```

2. Dans `home.blade.php`, ajoutez :
```blade
@include('components.header')
```

3. **Maintenez `Cmd` (Mac) ou `Ctrl` (Windows/Linux)** et **cliquez** sur `'components.header'`

**Résultat attendu :**
- Le fichier `resources/views/components/header.blade.php` s'ouvre
- Le curseur est positionné au début du fichier

### Test 3 : Autocomplete des chemins de vues

1. Dans un fichier Blade, commencez à taper :
```blade
@include('
```

2. Attendez que l'autocomplete se déclenche

**Résultat attendu :**
- Une liste de suggestions apparaît avec les vues disponibles
- Les vues sont affichées avec leur notation Laravel (ex: `components.header`)
- Vous pouvez naviguer avec les flèches et sélectionner

### Test 4 : Diagnostics pour vues inexistantes

1. Ajoutez une directive vers une vue qui n'existe pas :
```blade
@include('this.view.does.not.exist')
```

**Résultat attendu :**
- Un avertissement/erreur apparaît sous la ligne
- Le message indique que la vue n'existe pas

### Test 5 : Support PHP avec Intelephense

1. Dans un fichier Blade, tapez du code PHP :
```blade
@php
    $variable = App\Models\User::
@endphp
```

**Résultat attendu :**
- L'autocomplete PHP fonctionne après `::`
- Les méthodes de la classe User sont suggérées
- La coloration syntaxique PHP est active

## Débogage

### Laravel LS ne démarre pas

1. Ouvrez les logs du serveur de langage :
   - `View` → `Language Server Logs`
   - Cherchez "laravel-ls" dans les logs

2. Vérifiez si laravel-ls est installé :
```bash
which laravel-ls
# ou
laravel-ls --version
```

3. Si nécessaire, installez manuellement laravel-ls :
```bash
go install github.com/laravel-ls/laravel-ls/cmd/laravel-ls@latest
```

### L'extension ne se charge pas

1. Vérifiez que le fichier WASM a été généré :
```bash
ls -lh target/wasm32-wasip1/release/zed_blade_enhanced.wasm
```

2. Recompilez si nécessaire :
```bash
cargo build --target wasm32-wasip1 --release
```

3. Relancez Zed Editor complètement (Quit + Restart)

### Go to Definition ne fonctionne pas

1. Vérifiez que vous êtes dans un projet Laravel valide
2. Assurez-vous que le dossier `resources/views/` existe
3. Vérifiez que laravel-ls est bien démarré (voir logs)
4. Essayez de redémarrer le serveur de langage :
   - `Cmd+Shift+P` → "restart language server"

## Structure de test recommandée

Créez un projet Laravel minimal pour tester :

```bash
# Créer un nouveau projet Laravel (si nécessaire)
laravel new test-blade-extension
cd test-blade-extension

# Créer quelques vues de test
mkdir -p resources/views/{components,layouts,partials}

# layouts/app.blade.php
echo '@yield("content")' > resources/views/layouts/app.blade.php

# components/header.blade.php
echo '<header>Header Component</header>' > resources/views/components/header.blade.php

# home.blade.php
cat > resources/views/home.blade.php << 'EOF'
@extends('layouts.app')

@section('content')
    @include('components.header')

    <div class="content">
        @foreach($items as $item)
            <p>{{ $item }}</p>
        @endforeach
    </div>
@endsection
EOF
```

Ouvrez ensuite `resources/views/home.blade.php` dans Zed et testez toutes les fonctionnalités.

## Rapporter des problèmes

Si vous rencontrez des problèmes :

1. Collectez les informations suivantes :
   - Version de Zed (`zed --version`)
   - Logs du serveur de langage
   - Message d'erreur exact
   - Fichier Blade de test minimal

2. Créez une issue sur : https://github.com/croustibat/zed-blade-enhanced/issues

## Prochaines étapes

Une fois les tests validés :

1. Publier l'extension sur le dépôt officiel Zed
2. Créer une release sur GitHub
3. Annoncer sur Reddit r/laravel et r/zed
4. Mettre à jour zed-for-laravel pour recommander cette extension
