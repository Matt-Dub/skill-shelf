# Tauri pour l'application de bureau

Le widget reste ouvert toute la journée (panneau dans la zone de notification, éventuellement épinglé sur le bureau) et est distribué en open source sur Windows, macOS et Linux. Nous le construisons donc avec Tauri (backend Rust, interface web en TypeScript) plutôt qu'avec Electron ou une application native par OS. Tauri fournit une icône de notification native, des fenêtres sans cadre ou transparentes et un binaire d'environ 10 Mo à faible empreinte mémoire ; Electron coûterait environ 150 Mo de RAM pour une liste de skills, et des applications natives par OS tripleraient la base de code.

## Conséquences

- Le scan et la surveillance de la Library se font côté Rust ; l'interface ne fait qu'afficher.
- Le Pinned mode dépend de comportements de fenêtre propres à chaque OS, que Tauri n'abstrait pas. Sous Linux Wayland, il passe par le protocole layer-shell appliqué à la fenêtre GTK (pris en charge par KDE Plasma et les compositeurs wlroots comme Sway ou Hyprland, pas par GNOME). C'est donc un chemin de code spécifique à Linux dans la même application, pas un build séparé.
