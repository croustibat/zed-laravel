# Laravel Extension Templates

Ce dossier contient des fichiers de configuration optionnels que vous pouvez copier dans votre projet Laravel.

## tasks.json - Tâches Artisan

Copiez ce fichier pour avoir accès aux tâches Artisan directement dans Zed :

```bash
# À la racine de votre projet Laravel
mkdir -p .zed
cp path/to/zed-laravel/templates/tasks.json .zed/tasks.json
```

### Tâches disponibles

Une fois installé, appuyez sur `Cmd+Shift+T` (Mac) ou `Ctrl+Shift+T` (Linux/Windows) pour accéder aux tâches :

**Développement** :
- **Artisan Serve** - Démarre le serveur Laravel
- **Artisan Tinker** - Ouvre la console interactive
- **Artisan Route List** - Liste toutes les routes

**Base de données** :
- **Artisan Migrate** - Exécute les migrations
- **Artisan Migrate Fresh** - Reset et re-migrate
- **Artisan Migrate Rollback** - Annule la dernière migration

**Génération de code** :
- **Artisan Make Controller** - Crée un contrôleur
- **Artisan Make Model** - Crée un modèle
- **Artisan Make Livewire** - Crée un composant Livewire

**Formatting & Tests** :
- **Pint Format (All)** - Formate tout le code
- **Pint Format (Dirty)** - Formate uniquement les fichiers modifiés
- **PHPUnit (All)** - Lance tous les tests
- **PHPUnit (Current)** - Lance le test du fichier actuel
- **PHPUnit (Parallel)** - Tests en parallèle
- **PHPUnit (Coverage)** - Tests avec couverture
- **Pest** - Lance les tests Pest

**Frontend** :
- **NPM Dev** - Démarre Vite en mode dev
- **NPM Build** - Build pour production
- **NPM Install** - Installe les dépendances

**Composer** :
- **Composer Install** - Installe les dépendances PHP
- **Composer Update** - Met à jour les dépendances
- **Composer Dump Autoload** - Régénère l'autoloader

## Détection automatique de Sail

Les tâches détectent automatiquement si vous utilisez Laravel Sail et adaptent les commandes :

- **Sans Sail** : `php artisan serve`
- **Avec Sail** : `./vendor/bin/sail artisan serve`

## Personnalisation

Vous pouvez éditer `.zed/tasks.json` pour :
- Ajouter vos propres tâches
- Modifier les commandes existantes
- Ajuster les arguments par défaut

## Documentation Zed Tasks

Pour en savoir plus sur le système de tâches de Zed :
https://zed.dev/docs/tasks
