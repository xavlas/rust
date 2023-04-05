mod tests;



fn valeur_acquise(montant: u16, taux: u16, duree: u8) -> f32 {
    let taux_decimal = taux as f32 / 100.0;
    let facteur_actualisation = (1.0 + taux_decimal).powf(duree as f32);
    let valeur_actualisee = (montant as f32) * facteur_actualisation;
    valeur_actualisee.round()
}

// quelle somme dois je placer pour obtenir le montant "montant" au taux "taux" poour la "durée"
fn valeur_actuelle(montant: u16, taux: f32, duree: u8) -> f32 {
    let taux_decimal = taux / 100.0;
    let facteur_actualisation = (1.0 + taux_decimal).powf(-(duree as f32));
    let valeur_actualisee = (montant as f32) * facteur_actualisation;
    valeur_actualisee.round()
}

fn interet_simple(montant: u32, taux: f32, duree_jour: u8) -> f32 {
    let taux_decimal = taux * (duree_jour as f32 / 360.0 / 100.0);
    let interet = (montant as f32) * taux_decimal;
    interet.round()
}

//pendant combien d'année dois je placer une somme de S au taux T si on veut disposer de montant
fn duree(montant_origine: i16, montant_voulue: i16, taux: f32) -> f32 {
    let taux_decimal = taux / 100.0;
    let duree_remboursement =
        ((montant_origine as f32 / montant_voulue as f32).ln()) / (1.0 + taux_decimal).ln();
    duree_remboursement.abs().round()
}

fn calcul_capital(rente: f32, taux: f32, duree: i16) -> f32 {
    let taux_mois = taux_mensuel(taux);
    let vo = rente as f32 * (1.0 - (1.0 + taux_mois).powf(-(duree as f32 * 12.0))) / taux_mois;
    return vo;
}

fn placement_mensuel(
    rente_souhaitee: f32,
    taux: f32,
    duree_rente: i16,
    duree_cotisation: i16,
) -> f32 {
    let nominateur = taux_mensuel(taux) * calcul_capital(rente_souhaitee, taux, duree_rente);
    let denominateur = ((1.0 + taux_mensuel(taux)).powf(duree_cotisation as f32 * 12.0)) - 1.0;
    let m = nominateur / denominateur;
    return m;
}

fn taux_mensuel(taux: f32) -> f32 {
    let base = (1.0 + taux / 100.0) as f32;
    let exponent = (1.0 / 12.0) as f32;
    let result = base.powf(exponent as f32) - 1.0;
    return result;
}

fn annuite_remboursement(nominal: f32, taux: f32, duree: i8) -> f32 {
    let taux_decimal = 1.0 + taux / 100.0;
    let denominateur = 1.0 - (taux_decimal).powf(duree as f32 * -1.0);
    let resultat = (nominal * (taux as f32 / 100.0)) / denominateur;
    return resultat;
}

fn capital_rembourse(nominal: f32, taux: f32, duree: i8) -> f32 {
    let taux_decimal = 1.0 + taux / 100.0;
    let annuite = annuite_remboursement(nominal, taux, duree);
    let resultat = annuite * (taux_decimal).powf(duree as f32 * -1.0);
    return resultat;
}

fn capital_rembourse_annuite(nominal: f32, taux: f32, duree: i8, annuite: i8) -> f32 {
    let taux_decimal = 1.0 + taux / 100.0;
    let capital = capital_rembourse(nominal, taux, duree);
    let resultat = capital * taux_decimal.powf((annuite - 1) as f32);
    return resultat;
}

fn montant_capital(nominal: f32, taux: f32, duree: i8) -> f32 {
    let mut x: i8 = 1;
    let mut capital = 0;
    while x < duree {
        capital = capital + capital_rembourse_annuite(nominal, taux, duree, x) as i32;
        x = x + 1;
    }
    return capital as f32;
}

fn main() {

}