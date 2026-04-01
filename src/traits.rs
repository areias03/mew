use crate::model::*;

pub trait BiologicalModel {
    fn list_of_species(&self) -> Vec<String>;
    fn list_of_reactions(&self) -> Vec<String>;
    fn list_of_compartments(&self) -> Vec<String>;
    fn get_species(&self) -> &Vec<Species>;
    fn get_reactions(&self) -> &Vec<Reaction>;
    fn get_compartments(&self) -> &Vec<Compartment>;
    fn get_species_by_id(&self, species_id: &str) -> Option<&Species>;
    fn get_reaction_by_id(&self, reaction_id: &str) -> Option<&Reaction>;
}
