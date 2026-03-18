#[derive(Debug, Clone)]
pub struct Compartment {
    pub id: String,
    pub name: Option<String>,
    pub size: Option<f64>,
    pub spatial_dimensions: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct Species {
    pub id: String,
    pub name: Option<String>,
    pub compartment: String,
    pub boundary_condition: bool,
    pub has_only_substance_units: bool,
    pub initial_concentration: Option<f64>,
    pub initial_amount: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub id: String,
    pub name: Option<String>,
    pub value: Option<f64>,
    pub constant: bool,
}

#[derive(Debug, Clone)]
pub struct SpeciesReference {
    pub species: String,
    pub stoichiometry: Option<f64>,
    pub role: String, // "reactant", "product"
}

#[derive(Debug, Clone)]
pub struct Reaction {
    pub id: String,
    pub name: Option<String>,
    pub reactants: Vec<SpeciesReference>,
    pub products: Vec<SpeciesReference>,
    pub reversible: bool,
    pub fast: bool,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub id: Option<String>,
    pub name: Option<String>,
    pub compartments: Vec<Compartment>,
    pub species: Vec<Species>,
    pub parameters: Vec<Parameter>,
    pub reactions: Vec<Reaction>,
}

impl Model {
    pub fn new() -> Self {
        Model {
            id: None,
            name: None,
            compartments: Vec::new(),
            species: Vec::new(),
            parameters: Vec::new(),
            reactions: Vec::new(),
        }
    }

    pub fn set_id(&mut self, id: &str) {
        self.id = Some(id.to_string());
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_string());
    }

    pub fn list_of_species(&self) -> Vec<String> {
        self.species.iter().map(|s| s.id.clone()).collect()
    }

    pub fn list_of_reactions(&self) -> Vec<String> {
        self.reactions.iter().map(|r| r.id.clone()).collect()
    }

    pub fn list_of_compartments(&self) -> Vec<String> {
        self.compartments.iter().map(|c| c.id.clone()).collect()
    }

    pub fn get_species_by_id(&self, species_id: &str) -> Option<&Species> {
        self.species.iter().find(|s| s.id == species_id)
    }

    pub fn get_reaction_by_id(&self, reaction_id: &str) -> Option<&Reaction> {
        self.reactions.iter().find(|r| r.id == reaction_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_creation() {
        let mut model = Model::new();
        model.set_id("test_model");
        model.set_name("Test Model");

        assert_eq!(model.id, Some("test_model".to_string()));
        assert_eq!(model.name, Some("Test Model".to_string()));
    }

    #[test]
    fn test_list_of_species() {
        let mut model = Model::new();
        model.species.push(Species {
            id: "s1".to_string(),
            name: Some("Species 1".to_string()),
            compartment: "c1".to_string(),
            boundary_condition: false,
            has_only_substance_units: false,
            initial_concentration: Some(1.0),
            initial_amount: None,
        });
        model.species.push(Species {
            id: "s2".to_string(),
            name: Some("Species 2".to_string()),
            compartment: "c1".to_string(),
            boundary_condition: false,
            has_only_substance_units: false,
            initial_concentration: Some(2.0),
            initial_amount: None,
        });

        let species_ids = model.list_of_species();
        assert_eq!(species_ids, vec!["s1".to_string(), "s2".to_string()]);
    }

    #[test]
    fn test_get_species_by_id() {
        let mut model = Model::new();
        model.species.push(Species {
            id: "s1".to_string(),
            name: Some("Species 1".to_string()),
            compartment: "c1".to_string(),
            boundary_condition: false,
            has_only_substance_units: false,
            initial_concentration: Some(1.0),
            initial_amount: None,
        });

        let species = model.get_species_by_id("s1").unwrap();
        assert_eq!(species.id, "s1");
        assert_eq!(species.name, Some("Species 1".to_string()));
    }
}
