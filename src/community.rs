use crate::model::*;

pub enum CommunityError {
    IoError(std::io::Error),
}

#[derive(Debug, Clone)]
pub struct Community {
    pub name: Option<String>,
    pub members: Vec<Model>,
    pub compartments: Vec<Compartment>,
    pub species: Vec<Species>,
    pub parameters: Vec<Parameter>,
    pub reactions: Vec<Reaction>,
}

impl Community {
    pub fn new(name: Option<String>) -> Self {
        Community {
            name,
            members: Vec::new(),
            compartments: Vec::new(),
            species: Vec::new(),
            parameters: Vec::new(),
            reactions: Vec::new(),
        }
    }

    pub fn add_member(&mut self, model: Model) {
        self.members.push(model);
    }

    pub fn add_compartment(&mut self, compartment: Compartment) {
        self.compartments.push(compartment);
    }

    pub fn add_species(&mut self, species: Species) {
        self.species.push(species);
    }

    pub fn add_parameter(&mut self, parameter: Parameter) {
        self.parameters.push(parameter);
    }

    pub fn add_reaction(&mut self, reaction: Reaction) {
        self.reactions.push(reaction);
    }

    pub fn list_of_parameters(&self) -> Vec<String> {
        self.parameters.iter().map(|p| p.id.clone()).collect()
    }
}

impl crate::traits::BiologicalModel for Community {
    fn list_of_species(&self) -> Vec<String> {
        self.species.iter().map(|s| s.id.clone()).collect()
    }

    fn list_of_reactions(&self) -> Vec<String> {
        self.reactions.iter().map(|r| r.id.clone()).collect()
    }

    fn list_of_compartments(&self) -> Vec<String> {
        self.compartments.iter().map(|c| c.id.clone()).collect()
    }

    fn get_species(&self) -> &Vec<Species> {
        &self.species
    }

    fn get_reactions(&self) -> &Vec<Reaction> {
        &self.reactions
    }

    fn get_compartments(&self) -> &Vec<Compartment> {
        &self.compartments
    }

    fn get_species_by_id(&self, species_id: &str) -> Option<&Species> {
        self.species.iter().find(|s| s.id == species_id)
    }

    fn get_reaction_by_id(&self, reaction_id: &str) -> Option<&Reaction> {
        self.reactions.iter().find(|r| r.id == reaction_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::BiologicalModel;

    #[test]
    fn test_community_creation() {
        let community = Community::new(Some("Test Community".to_string()));
        assert_eq!(community.name, Some("Test Community".to_string()));
        assert!(community.members.is_empty());
        assert!(community.compartments.is_empty());
        assert!(community.species.is_empty());
        assert!(community.parameters.is_empty());
        assert!(community.reactions.is_empty());
    }

    #[test]
    fn test_add_member() {
        let mut community = Community::new(None);
        let model = Model {
            id: Some("model1".to_string()),
            name: Some("Model 1".to_string()),
            compartments: Vec::new(),
            species: Vec::new(),
            parameters: Vec::new(),
            reactions: Vec::new(),
        };
        community.add_member(model.clone());
        assert_eq!(community.members.len(), 1);
        assert_eq!(community.members[0].id, Some("model1".to_string()));
    }

    #[test]
    fn test_add_compartment() {
        let mut community = Community::new(None);
        let compartment = Compartment {
            id: "comp1".to_string(),
            name: Some("Compartment 1".to_string()),
            size: Some(1.0),
            spatial_dimensions: Some(3),
        };
        community.add_compartment(compartment.clone());
        assert_eq!(community.compartments.len(), 1);
        assert_eq!(community.compartments[0].id, "comp1".to_string());
        assert_eq!(
            community.compartments[0].name,
            Some("Compartment 1".to_string())
        );
        assert_eq!(community.compartments[0].size, Some(1.0));
        assert_eq!(community.compartments[0].spatial_dimensions, Some(3));
    }

    #[test]
    fn test_get_species_by_id() {
        let mut community = Community::new(None);
        let species = Species {
            id: "species1".to_string(),
            name: Some("Species 1".to_string()),
            compartment: "comp1".to_string(),
            initial_amount: Some(10.0),
            initial_concentration: None,
            boundary_condition: false,
            has_only_substance_units: false,
        };
        community.add_species(species.clone());
        let retrieved_species = community.get_species_by_id("species1");
        assert!(retrieved_species.is_some());
        assert_eq!(retrieved_species.unwrap().id, "species1".to_string());
    }
}
