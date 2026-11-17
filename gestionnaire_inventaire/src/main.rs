//Ce prototype a été fait assisté par l'intelligence artificiel (Chat GPT)

use serde::{Serialize, Deserialize};
use std::fs;

use std::io::{self, Write};

//structure qui représente les articles dans l'inventaire.
#[derive(Serialize, Deserialize)]
struct Article {
    nom: String,
    quantite: u32,
    prix: f64,
}

fn sauvegarder_inventaire(inventaire: &[Article]) {
    match serde_json::to_string_pretty(inventaire) {
        Ok(json) => {
            match fs::write("inventaire.json", json) {
                Ok(_) => println!("Inventaire sauvegardé avec succès!"),
                Err(e) => println!("Erreur de sauvegarde : {}", e),
            }
        }
        Err(e) => println!("Erreur de conversion JSON : {}", e),
    }
}

//lecture d'un fichier json expliquer par l'IA
fn charger_inventaire() -> Result<Vec<Article>, Box<dyn std::error::Error>> {
    match fs::read_to_string("inventaire.json") {
        Ok(contenu) => {
            let inventaire: Vec<Article> = serde_json::from_str(&contenu)?;
            Ok(inventaire)
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            Ok(Vec::new())
        }
        Err(e) => Err(e.into()),
    }
}

//fonction de lecture en partie généré par IA
fn lire_texte(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();

    let mut entree = String::new();

    io::stdin()
        .read_line(&mut entree)
        .expect("Erreur de lecture");

    entree.trim().to_string()
}


fn lire_entier(message: &str) -> u32 {
    loop {
        let entree = lire_texte(message);

        match entree.parse::<u32>() {
            Ok(nombre) => return nombre,
            Err(_) => println!("Veuillez entrer un nombre entier valide."),
        }
    }
}


fn lire_prix(message: &str) -> f64 {
    loop {
        let entree = lire_texte(message).replace(',', ".");

        match entree.parse::<f64>() {
            Ok(prix) if prix.is_finite() && prix >= 0.0 => {
                return prix;
            }
            _ => println!("Veuillez entrer un prix valide."),
        }
    }
}
//fin code IA

fn ajouter_article(inventaire: &mut Vec<Article>) {
    println!("\n--- Ajouter un article ---");

    let nom = lire_texte("Nom : ");

    if nom.is_empty() {
        println!("Le nom ne peut pas être vide.");
        return;
    }

    let quantite = lire_entier("Quantité : ");
    let prix = lire_prix("Prix unitaire : ");

    inventaire.push(Article {
        nom,
        quantite,
        prix,
    });

    println!("Article ajouté avec succès!");
}


fn afficher_inventaire(inventaire: &[Article]) {
    println!("\n--- Inventaire ---");

    if inventaire.is_empty() {
        println!("L'inventaire est vide.");
        return;
    }

    for (index, article) in inventaire.iter().enumerate() {
        println!(
            "{}. {} | Quantité : {} | Prix : {:.2} $",
            index + 1,
            article.nom,
            article.quantite,
            article.prix
        );
    }
}


fn rechercher_article(inventaire: &[Article]) {
    println!("\n--- Rechercher un article ---");

    let recherche = lire_texte("Nom de l'article : ");

    let mut trouve = false;

    for article in inventaire {
        if article
            .nom
            .to_lowercase()
            .contains(&recherche.to_lowercase())
        {
            println!(
                "{} | Quantité : {} | Prix : {:.2} $",
                article.nom,
                article.quantite,
                article.prix
            );

            trouve = true;
        }
    }

    if !trouve {
        println!("Aucun article trouvé.");
    }
}


fn modifier_article(inventaire: &mut Vec<Article>) {
    println!("\n--- Modifier un article ---");

    let recherche = lire_texte("Nom de l'article à modifier : ");

    let position = inventaire.iter().position(|article| {
        article.nom.eq_ignore_ascii_case(&recherche)
    });

    match position {
        Some(index) => {
            let article = &mut inventaire[index];

            println!("Article sélectionné : {}", article.nom);
            println!("1. Modifier le nom");
            println!("2. Modifier la quantité");
            println!("3. Modifier le prix");
            println!("4. Retour");

            let choix = lire_texte("Votre choix : ");

            match choix.as_str() {
                "1" => {
                    let nouveau_nom = lire_texte("Nouveau nom : ");

                    if !nouveau_nom.is_empty() {
                        article.nom = nouveau_nom;
                        println!("Nom modifié.");
                    } else {
                        println!("Le nom ne peut pas être vide.");
                    }
                }
                "2" => {
                    article.quantite = lire_entier("Nouvelle quantité : ");
                    println!("Quantité modifiée.");
                }
                "3" => {
                    article.prix = lire_prix("Nouveau prix : ");
                    println!("Prix modifié.");
                }
                "4" => println!("Retour au menu."),
                _ => println!("Choix invalide."),
            }
        }
        None => println!("Article introuvable."),
    }
}

//Fonction principal du projet, la loop permet de réafficher le menu apres chaque opération.
fn main() {
    let mut inventaire: Vec<Article> = match charger_inventaire() {
        Ok(donnees) => {
            println!("Inventaire chargé avec succès!");
            donnees
        }
        Err(e) => {
            println!("Erreur lors du chargement : {}", e);
            return;
        }
    };

    loop {
        println!("\n===== GESTIONNAIRE D'INVENTAIRE =====");
        println!("1. Ajouter un article");
        println!("2. Afficher l'inventaire");
        println!("3. Rechercher un article");
        println!("4. Modifier un article");
        println!("5. Quitter");

        let choix = lire_texte("Votre choix : ");


        match choix.as_str() {
            "1" => {
                ajouter_article(&mut inventaire);
                sauvegarder_inventaire(&inventaire);
            },
            "2" => afficher_inventaire(&inventaire),
            "3" => rechercher_article(&inventaire),
            "4" => {modifier_article(&mut inventaire);
                    sauvegarder_inventaire(&inventaire);
            },
            "5" => {
                println!("Fermeture du programme.");
                break;
            }
            _ => println!("Choix invalide. Réessayez."),
        }
    }
}
