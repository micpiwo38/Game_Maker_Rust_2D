# Game_Maker_Rust_2D
Un logiciel de création de jeux video 2D type Unity, Godot, etc... codé avec le langage rust
Étape 1 : Le squelette "Fenêtre & Rendu de base"

    Initialisez une fenêtre avec winit.

    Configurez un contexte wgpu pour effacer l'écran avec une couleur unie.

    Affichez un premier sprite 2D texturé à l'écran en écrivant vos propres shaders (WGSL).

Étape 2 : L'architecture de l'éditeur (UI)

    Intégrez egui dans votre boucle de rendu wgpu.

    Créez une interface avec un layout de type "IDE" :

        Une barre de menu en haut (File, Edit, Window).

        Un panneau "Hierarchy" (liste des objets de la scène).

        Un panneau "Inspector" (propriétés de l'objet sélectionné).

        Une vue centrale "Viewport" (où le jeu est rendu, idéalement via une texture wgpu partagée avec egui).

Étape 3 : Le modèle de données (Scene & Entities)

    Définissez la structure d'un GameObject / Node (position, rotation, échelle, composants graphiques, scripts).

    Mettez en place un système de sérialisation avec serde pour enregistrer la scène dans un fichier .ron ou .json.

Étape 4 : Le Gameplay et la Boucle de Jeu

    Séparez la boucle de l'éditeur (pause, édition) de la boucle de jeu (runtime, update de la physique, scripts).

    Pour le scripting, vous pouvez intégrer un langage de script externe comme Rhai ou Lua (mlua), ou coder les comportements directement en Rust si votre moteur compile des plugins dynamiques (.dll / .so).

4. Conseils pour réussir

   Pratiquez le "Dogfooding" tôt : Dès que votre éditeur peut afficher un carré et le bouger, essayez de créer un tout petit mini-jeu (type Pong ou clone de Flappy Bird) uniquement à l'aide de vos outils. Cela vous montrera immédiatement ce qui ergonomiquement ne va pas dans votre architecture.

   Maîtrisez le borrow checker pour l'arbre de scènes : Gérer des relations parent-enfant (un objet attaché à un autre) en Rust peut être délicat à cause des références mutuelles. L'utilisation d'un stockage basé sur des indices (IDs) ou un pattern ECS (bevy_ecs) simplifie grandement la vie par rapport aux pointeurs traditionnels.

   Regardez les projets existants : Inspirez-vous de moteurs open-source en Rust comme Bevy, Fyrox (qui possède un éditeur complet très instructif) ou ggez pour voir comment ils gèrent la boucle de jeu et les ressources.
