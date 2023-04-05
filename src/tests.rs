use crate::{
    annuite_remboursement, calcul_capital, capital_rembourse, capital_rembourse_annuite, duree,
    interet_simple, montant_capital, placement_mensuel, taux_mensuel, valeur_acquise,
    valeur_actuelle,
};

#[test]
fn test_valeur_acquise() {
    assert_eq!(valeur_acquise(1000, 6, 1), 1060f32); // Integration tests (and benchmarks) 'depend' to the crate like
    assert_eq!(valeur_acquise(11000, 3, 20), 19867f32);
}

#[test]
fn test_interet_simple() {
    assert_eq!(interet_simple(1000000, 2.9, 99), 7975 as f32);
}

#[test]
fn test_valeur_actuelle() {
    assert_eq!(valeur_actuelle(35000, 3.5, 10), 24812 as f32);
    assert_eq!(duree(20000, 25000, 4.0), 6 as f32);
}

#[test]
fn test_duree() {
    assert_eq!(duree(20000, 25000, 4.0), 6 as f32);
}

#[test]
fn test_calcul_capital() {
    assert_eq!(calcul_capital(460.0, 6.0, 20), 65036.758);
}

#[test]
fn test_taux_mensuel() {
    assert_eq!(taux_mensuel(6.0), 0.0048675537);
}

#[test]
fn test_placement_mensuel() {
    assert_eq!(placement_mensuel(2000.0, 3.0, 20, 42), 362.755);
}

#[test]
fn test_amortissement() {
    assert_eq!(annuite_remboursement(80000.0, 7.0, 5), 19511.242);
}

#[test]
fn test_capital_rembourse() {
    assert_eq!(capital_rembourse(80000.0, 7.0, 5), 13911.242);
}

#[test]
fn test_amortissement_annuite() {
    assert_eq!(capital_rembourse_annuite(80000.0, 7.0, 5, 3), 15926.982);
}

#[test]
fn test_montant_capital() {
    assert_eq!(montant_capital(80000.0, 7.0, 5), 61763.0);
}
