use std::fs;
use std::path::Path;
use serde::Deserialize;
#[derive(Deserialize, Debug)]
struct UProject {
    #[serde(rename = "EngineAssociation")]
    engine_association: String,
    #[serde(rename = "Plugins")]
    plugins: Option<Vec<Plugin>>,
}

#[derive(Deserialize, Debug)]
struct Plugin {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Enabled")]
    enabled: bool,
}

pub fn show_infos(file_path: &str) {
    let data = fs::read_to_string(file_path).expect("Impossible de lire le fichier UPROJECT");
    let project: Result<UProject, _> = serde_json::from_str(&data);

    match project {
        Ok(project) => {
            // Récupération du nom du jeu à partir du nom du fichier
            let game_name = Path::new(file_path)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Nom inconnu");

            println!("Nom du jeu : {}", game_name);

            // Vérification correcte de l'EngineAssociation
            if project.engine_association.starts_with("5.") {
                println!("Association de l'engine : {}", project.engine_association);
            } else {
                println!("Association de l'engine : Unreal from Source");
            }

            if let Some(plugins) = project.plugins {
                println!("Plugins actifs :");
                for plugin in plugins {
                    if plugin.enabled {
                        println!("- {}", plugin.name);
                    }
                }
            } else {
                println!("Aucun plugin actif.");
            }
        }
        Err(e) => {
            eprintln!("Erreur de parsing JSON : {}", e);
        }
    }
}