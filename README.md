# 1. Dépendances et leur rôle
Voici les bibliothèques principales avec leurs responsabilités dans le projet :

## Bibliothèques principales
## 1. Bevy

- Moteur de jeu ECS (Entity-Component-System) pour la gestion des entités et rendu 3D/2D.
- Utilisé pour la logique de jeu, le rendu graphique, l’interface utilisateur et la synchronisation du monde.
## 2. Tokio

- Gestion des tâches asynchrones, indispensable pour les communications réseau via UDP.
- Utilisé pour créer et gérer un serveur UDP performant.
## 3. Serde

- Sérialisation et désérialisation des données (JSON ou bincode).
- Facilite les échanges de données structurées entre le serveur et les clients.
## 4. Rand

- Génération procédurale des labyrinthes (algorithmes comme Prim ou Recursive Backtracking).
## 5. Crossbeam

- Pour la communication entre threads (si vous séparez les tâches réseau et le rendu graphique).
- Assure un échange de données performant entre les sous-systèmes.
- Bevy FPS Plugin (ou implémentation maison simple)

## 6. Affichage des FPS pour le HUD.
- Bibliothèques optionnelles
- Quinn : Si vous souhaitez explorer un protocole basé sur UDP avec des garanties supplémentaires (QUIC).
- Petgraph : Pour la génération et l’analyse du graphe du labyrinthe (facultatif mais utile pour des labyrinthes complexes).
# 2. Architecture globale
### Modules principaux
Le projet sera divisé en modules distincts :

# Module réseau

- Responsable de la communication client-serveur via UDP.
- Sérialisation/désérialisation des messages avec Serde.
- Maintien de la latence minimale avec gestion des paquets UDP (p. ex., perte de paquets).
# Module logique de jeu

- Gestion des entités du jeu : joueur, labyrinthes, obstacles, niveaux.
- Génération dynamique des labyrinthes (augmentant en difficulté).
- Mécanique des collisions, progression, et conditions de victoire.
# Module graphique

- Rendu des labyrinthes et du monde.
- Caméra et mini-carte synchronisées avec les positions des joueurs.
- Affichage du HUD (FPS, pseudo, niveau).
# Module client

- Gestion de l’interface utilisateur, y compris l'entrée clavier et souris.
- Intégration avec Bevy pour afficher les positions synchronisées avec le serveur.
# Module serveur

- Maintien de l’état global du jeu (position des joueurs, progression).
- Diffusion périodique des mises à jour aux clients connectés.
- Gestion des connexions multiples (au moins 10 joueurs).
## 3. Organisation des fichiers
Voici une structure recommandée :

```bash
src/
├── main.rs                # Point d'entrée principal.
├── client.rs              # Code principal du client.
├── server.rs              # Code principal du serveur.
├── network/               # Gestion réseau.
│   ├── udp.rs             # Gestion des connexions UDP.
│   ├── messages.rs        # Structures de messages réseau.
├── game/                  # Logique du jeu.
│   ├── maze.rs            # Génération du labyrinthe.
│   ├── player.rs          # Gestion des joueurs.
│   └── levels.rs          # Gestion des niveaux et difficulté.
├── graphics/              # Rendu graphique.
│   ├── ui.rs              # Interface utilisateur (HUD, mini-carte).
│   ├── rendering.rs       # Rendu des labyrinthes et des joueurs.
├── utils.rs               # Fonctions utilitaires partagées.
├── Cargo.toml             # Dépendances.
```
# 3. Répartition logique des tâches
### A. Module réseau
- Responsable : Communication UDP client-serveur.
#### Tâches principales :
- Implémentation du serveur UDP avec Tokio.
- Sérialisation des données (positions des joueurs, état du jeu) avec Serde.
- Gestion des connexions simultanées (10+ joueurs).
### B. Module logique de jeu
Responsable : Génération procédurale et gestion des règles.
#### Tâches principales :
- Génération dynamique des labyrinthes (niveau 1, 2, 3).
- Gestion des collisions et mouvements.
- Implémentation des conditions de victoire.
### C. Module graphique
Responsable : Affichage et interaction utilisateur.
#### Tâches principales :
- Rendu des labyrinthes et des positions des joueurs avec Bevy.
- Synchronisation de la caméra avec les mouvements du joueur.
- Création du HUD (mini-carte, affichage FPS, pseudo).
### D. Module client
Responsable : Gestion des interactions utilisateur.
#### Tâches principales :
- Interface en ligne de commande pour demander l’adresse IP et le pseudo.
- Gestion des entrées utilisateur (déplacement, interactions).
- Synchronisation avec l’état du serveur (via UDP).
### E. Module serveur
Responsable : Gestion centralisée du jeu.
#### Tâches principales :
- Maintien de l’état global du jeu (labyrinthe, positions, scores).
- Gestion des connexions réseau et des mises à jour synchronisées.
- Diffusion régulière des états aux clients connectés.
# 4. Points importants pour une intégration cohérente
Cohérence entre les modules réseau et graphique

- Assurez que le module réseau envoie uniquement les données nécessaires (positions des joueurs, état des labyrinthes).
- Le module graphique doit être capable de mettre à jour l’affichage à partir des données reçues.
Synchronisation client-serveur

- Utilisez des ID uniques pour chaque joueur pour associer les mises à jour.
- Implémentez une logique de prediction côté client pour masquer les effets de latence.
Performances et modularité

- Séparez clairement les responsabilités des modules pour faciliter les tests unitaires.
- Optimisez les algorithmes de labyrinthe et minimisez les calculs inutiles dans la boucle principale.
Testing

## Créez des tests automatisés pour chaque module (connexion réseau, génération du labyrinthe, etc.).
Résumé des tâches pour l’équipe
- Développeur A : Réseau (module serveur et UDP client).
- Développeur B : Logique du jeu (génération des labyrinthes, gestion des règles).
- Développeur C : Graphisme et rendu (HUD, mini-carte, caméra).
- Développeur D : Intégration et tests (synchronisation client-serveur, fluidité).
## Avec cette approche, vous pouvez avancer efficacement tout en garantissant une architecture maintenable et évolutive.

[drive assets](https://drive.google.com/drive/folders/1z8VZv3aCziesocR3_6-HBNd8VNLD9gaP?usp=drive_link)