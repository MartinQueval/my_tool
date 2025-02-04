# MyTool - Unreal Engine Automation Tool

## Description
MyTool est un outil en ligne de commande permettant de lire un fichier uproject pour afficher ses informations, pour compiler le projet ou même le packager.

## Installation
1. Assurez-vous d'avoir Rust installé sur votre machine.
2. Clonez le dépôt ou téléchargez le projet et placer à la racine d'Unreal from Source.
3. Exécutez l'outil en ligne de commande.

## Utilisation

### Afficher les informations d'un projet Unreal
Affiche les informations principales d'un fichier `.uproject`.
```sh
cargo run -- <chemin_du_projet.uproject> show-infos
```
Dans le répertoire du tool.

**Exemple :**
```sh
cargo run -- D:\dev\UnrealEngine\Project\Test\Test.uproject show-infos
```

### Compiler un projet Unreal
Compile le projet Unreal spécifié.
```sh
cargo run -- <chemin_du_projet.uproject> build
```
Dans le répertoire du tool.

**Exemple :**
```sh
cargo run -- D:\dev\UnrealEngine\Project\Test\Test.uproject build
```

### Packager un projet Unreal
Génère un package du projet Unreal et le stocke dans le dossier de sortie spécifié.
```sh
cargo run -- <chemin_du_projet.uproject> package <chemin_de_sortie>
```
Dans le répertoire du tool.

**Exemple :**
```sh
cargo run -- D:\dev\UnrealEngine\Project\Test\Test.uproject package D:\Output\Package
```

## Remarque
- Assurez-vous que les chemins indiqués sont corrects, accessibles et de préférences absolus.
- L'outil nécessite que les scripts `RunUAT.bat` et `UnrealEditor-Cmd.exe` soient présents et configurés correctement.

## Auteur
Développé par Martin QUEVAL

