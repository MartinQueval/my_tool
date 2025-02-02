use std::process::Command;

pub fn build_project(uproject_path: &str) {
    println!("Compilation du projet : {}", uproject_path);

    let command = Command::new("D:\\dev\\UnrealEngine\\Engine\\Build\\BatchFiles\\RunUAT.bat")
        .args(&[
            "BuildCookRun",
            &format!("-project={}", uproject_path),
            "-noP4",
            "-clientconfig=Development",
            "-serverconfig=Development",
            "-nocompileeditor",
            "-installed",
            "-unrealexe=D:\\dev\\UnrealEngine\\Engine\\Binaries\\Win64\\UnrealEditor-Cmd.exe",
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
pub fn package_project(uproject_path: &str, output_path: &str) {
    let command = Command::new("D:\\dev\\UnrealEngine\\Engine\\Build\\BatchFiles\\RunUAT.bat")
        .args(&[
            "BuildCookRun",
            &format!("-project={}", uproject_path),
            "-noP4",
            "-clientconfig=Development",
            "-serverconfig=Development",
            "-nocompile",
            "-nocompileeditor",
            "-installed",
            "-unrealexe=D:\\dev\\UnrealEngine\\Engine\\Binaries\\Win64\\UnrealEditor-Cmd.exe",
            "-utf8output",
            "-platform=Win64",
            "-build",
            "-cook",
            "-map=ThirdPersonMap+StarterMap+Minimal_Default+Advanced_Lighting",
            "-CookCultures=en",
            "-unversionedcookedcontent",
            "-stage",
            "-package",
            "-archive",  // 🔥 Ajout de l'option d'archivage
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

