use std::env;

mod jour1;
mod jour2;
mod jour3;
mod jour4;

mod jour5;
mod jour6;
mod jour7;
mod jour8;
mod jour9;
mod jour10;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run <jour>");
        std::process::exit(1);
    }

    let jour = &args[1];

    match jour.as_str() {
        "1" => affiche_resultats(jour1::p1(), jour1::p2()),
        "2" => affiche_resultats(jour2::p1(), jour2::p2()),
        "3" => affiche_resultats(jour3::p1(), jour3::p2()),
        "4" => affiche_resultats(jour4::p1(), jour4::p2()),
        "5" => affiche_resultats(jour5::p1(), jour5::p2()),
        "6" => affiche_resultats(jour6::p1(), jour6::p2()),
        "7" => affiche_resultats(jour7::p1(), jour7::p2()),
        "8" => affiche_resultats(jour8::p1(), jour8::p2()),
        "9" => affiche_resultats(jour9::p1(), jour9::p2()),
        "10" => affiche_resultats(jour10::p1(), jour10::p2()),
        _ => {
            eprintln!("Jour inconnu !");
            std::process::exit(1);
        }
    }
}

fn affiche_resultats(p1: impl std::fmt::Display, p2: impl std::fmt::Display) {
    println!("Résultat de p1 : {}", p1);
    println!("Résultat de p2 : {}", p2);
}