use std::env;
use std::process::Command;
use std::path::PathBuf;

// Fonction qui récupère le chemin d'Unreal basé sur le chemin du binaire
fn get_unreal_engine_path() -> PathBuf {
    let exe_path = env::current_exe().expect("Impossible de récupérer le chemin de l'exécutable");
    let mut current_path = exe_path.parent();

    // On remonte jusqu'à trouver le dossier "UnrealEngine"
    while let Some(path) = current_path {
        if path.file_name().map_or(false, |name| name == "UnrealEngine") {
            return path.join("Engine");
        }
        current_path = path.parent();
    }

    panic!("Impossible de trouver le dossier UnrealEngine");
}

// Fonction qui récupère le chemin de RunUAT.bat
fn get_run_uat_path() -> String {
    get_unreal_engine_path()
        .join("Build/BatchFiles/RunUAT.bat")
        .to_string_lossy()
        .replace("/", "\\")
}

// Fonction qui récupère le chemin de UnrealEditor-Cmd.exe
fn get_unreal_cmd_path() -> String {
    get_unreal_engine_path()
        .join("Binaries/Win64/UnrealEditor-Cmd.exe")
        .to_string_lossy()
        .replace("/", "\\")
}

//Build le projet
pub fn build_project(uproject_path: &str) {

    println!("🛠 Compilation du projet : {}", uproject_path);

    let command = Command::new(get_run_uat_path())
        .args(&[
            "BuildCookRun",
            &format!("-project={}", uproject_path),
            "-noP4",
            "-clientconfig=Development",
            "-serverconfig=Development",
            "-installed",
            &format!("-unrealexe={}", get_unreal_cmd_path()),
            "-utf8output",
            "-platform=Win64",
            "-build",
        ])
        .output()
        .expect("Erreur lors de l'exécution de la commande RunUAT.bat");

    if command.status.success() {
        println!("✅ Compilation réussie !");
    } else {
        eprintln!("❌ Erreur : {}", String::from_utf8_lossy(&command.stderr));
    }
}

//Package le projet
pub fn package_project(uproject_path: &str, output_path: &str) {

    let command = Command::new(get_run_uat_path())
        .args(&[
            "BuildCookRun",
            &format!("-project={}", uproject_path),
            "-noP4",
            "-clientconfig=Development",
            "-serverconfig=Development",
            "-nocompile",
            "-installed",
            &format!("-unrealexe={}", get_unreal_cmd_path()),
            "-utf8output",
            "-platform=Win64",
            "-build",
            "-cook",
            "-unversionedcookedcontent",
            "-stage",
            "-package",
            "-archive",
            &format!("-archivedirectory={}", output_path),
        ])
        .output()
        .expect("Erreur lors de l'exécution de la commande RunUAT.bat");

    if command.status.success() {
        println!("✅ Packaging réussi !");
    } else {
        eprintln!("❌ Erreur : {}", String::from_utf8_lossy(&command.stderr));
    }
}