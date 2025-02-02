mod uproject;
mod commands;

use clap::{Arg, Command};

fn main() {
    let matches = Command::new("MyTool")
        .version("1.0")
        .about("Unreal Engine Automation Tool")
        .arg(Arg::new("file")
            .help("Chemin du fichier UPROJECT")
            .required(true)
            .index(1))
        .subcommand(Command::new("show-infos")
            .about("Affiche les informations du fichier UPROJECT"))
        .subcommand(Command::new("build")
            .about("Compile le projet Unreal"))
        .subcommand(Command::new("package")
            .about("Package le projet Unreal")
            .arg(Arg::new("output")
                .help("Chemin de sortie du package")
                .required(true)))
        .get_matches();

    let file = matches.get_one::<String>("file").unwrap();

    match matches.subcommand_name() {
        Some("show-infos") => {
            uproject::show_infos(file);  // Appel de la fonction show_infos
        }
        Some("build") => {
            commands::build_project(file);
        }
        Some("package") => {
            if let Some(args) = matches.subcommand_matches("package") {
                let file = matches.get_one::<String>("file").unwrap();
                if let Some(output) = args.get_one::<String>("output") {
                    println!("Packaging du projet : {}\nChemin de sortie : {}", file, output);
                    commands::package_project(file, output);
                } else {
                    eprintln!("Erreur : L'argument <output> est requis pour la commande `package`.");
                }
            }
        }
        _ => eprintln!("Commande inconnue"),
    }
}
