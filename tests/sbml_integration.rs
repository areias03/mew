use mew::sbml::read_sbml;
use mew::traits::BiologicalModel;

#[test]
fn parses_ecoli_core_sbml_fixture() {
    let model = read_sbml("tests/fixtures/ecoli_core.xml").expect("fixture should parse");

    assert_eq!(model.id.as_deref(), Some("ecoli_core"));
    assert_eq!(model.name.as_deref(), Some("E. coli core model"));
    assert_eq!(model.compartments.len(), 2);
    assert_eq!(model.species.len(), 7);
    assert_eq!(model.reactions.len(), 3);

    assert!(model.get_species_by_id("glc__D_e").is_some());
    assert!(model.get_species_by_id("g6p_c").is_some());
    assert!(model.get_reaction_by_id("GLCpts").is_some());
    assert!(model.get_reaction_by_id("PGI").is_some());

    let glcpts = model
        .get_reaction_by_id("GLCpts")
        .expect("GLCpts should exist");
    assert_eq!(glcpts.reactants.len(), 2);
    assert_eq!(glcpts.products.len(), 3);
    assert!(!glcpts.reversible);
}
