use crate::model::*;
use std::collections::HashMap;

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

pub fn create_community(name: Option<String>, models: Vec<Model>) -> Community {
    let mut community = Community::new(name);
    community.add_compartment(Compartment {
        id: "m".to_string(),
        name: Some("medium".to_string()),
        size: None,
        spatial_dimensions: None,
    });

    for (index, model) in models.into_iter().enumerate() {
        let origin = model_origin_prefix(&model, index);
        let mut compartment_id_map: HashMap<String, String> = HashMap::new();
        let mut species_id_map: HashMap<String, String> = HashMap::new();

        for compartment in &model.compartments {
            if is_medium_compartment(compartment) {
                compartment_id_map.insert(compartment.id.clone(), "m".to_string());
                continue;
            }

            let new_id = prefixed_id(&origin, &compartment.id);
            compartment_id_map.insert(compartment.id.clone(), new_id.clone());

            community.add_compartment(Compartment {
                id: new_id,
                name: compartment.name.clone(),
                size: compartment.size,
                spatial_dimensions: compartment.spatial_dimensions,
            });
        }

        for species in &model.species {
            let is_medium = species.boundary_condition
                || compartment_id_map
                    .get(&species.compartment)
                    .is_some_and(|mapped| mapped == "m");

            let new_id = if is_medium {
                species.id.clone()
            } else {
                prefixed_id(&origin, &species.id)
            };
            species_id_map.insert(species.id.clone(), new_id.clone());

            let mapped_compartment = if is_medium {
                "m".to_string()
            } else {
                compartment_id_map
                    .get(&species.compartment)
                    .cloned()
                    .unwrap_or_else(|| prefixed_id(&origin, &species.compartment))
            };

            if !community
                .species
                .iter()
                .any(|existing| existing.id == new_id)
            {
                community.add_species(Species {
                    id: new_id,
                    name: species.name.clone(),
                    compartment: mapped_compartment,
                    boundary_condition: species.boundary_condition,
                    has_only_substance_units: species.has_only_substance_units,
                    initial_concentration: species.initial_concentration,
                    initial_amount: species.initial_amount,
                });
            }
        }

        for parameter in &model.parameters {
            community.add_parameter(Parameter {
                id: prefixed_id(&origin, &parameter.id),
                name: parameter.name.clone(),
                value: parameter.value,
                constant: parameter.constant,
            });
        }

        for reaction in &model.reactions {
            let remap_species_ref = |species_ref: &SpeciesReference| SpeciesReference {
                species: species_id_map
                    .get(&species_ref.species)
                    .cloned()
                    .unwrap_or_else(|| prefixed_id(&origin, &species_ref.species)),
                stoichiometry: species_ref.stoichiometry,
                role: species_ref.role.clone(),
            };

            community.add_reaction(Reaction {
                id: prefixed_id(&origin, &reaction.id),
                name: reaction.name.clone(),
                reactants: reaction.reactants.iter().map(remap_species_ref).collect(),
                products: reaction.products.iter().map(remap_species_ref).collect(),
                reversible: reaction.reversible,
                fast: reaction.fast,
            });
        }

        community.add_member(model);
    }

    community
}

fn model_origin_prefix(model: &Model, index: usize) -> String {
    let raw_origin = model
        .id
        .as_deref()
        .or(model.name.as_deref())
        .map(str::to_string)
        .unwrap_or_else(|| format!("model_{}", index + 1));
    sanitize_identifier(&raw_origin)
}

fn sanitize_identifier(value: &str) -> String {
    let sanitized: String = value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                ch
            } else {
                '_'
            }
        })
        .collect();

    sanitized.trim_matches('_').to_string()
}

fn prefixed_id(origin: &str, id: &str) -> String {
    format!("{id}__{origin}")
}

fn is_medium_compartment(compartment: &Compartment) -> bool {
    if compartment.id == "m" || compartment.id == "e" {
        return true;
    }

    compartment
        .name
        .as_deref()
        .map(|name| {
            let lower = name.to_ascii_lowercase();
            lower.contains("medium") || lower.contains("extracellular")
        })
        .unwrap_or(false)
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

    #[test]
    fn test_create_community_merges_models_with_origin_prefix() {
        let model_a = Model {
            id: Some("ecoli".to_string()),
            name: Some("E. coli".to_string()),
            compartments: vec![
                Compartment {
                    id: "c".to_string(),
                    name: Some("cytosol".to_string()),
                    size: Some(1.0),
                    spatial_dimensions: Some(3),
                },
                Compartment {
                    id: "e".to_string(),
                    name: Some("extracellular".to_string()),
                    size: Some(1.0),
                    spatial_dimensions: Some(3),
                },
            ],
            species: vec![
                Species {
                    id: "glc_c".to_string(),
                    name: Some("glucose".to_string()),
                    compartment: "c".to_string(),
                    boundary_condition: false,
                    has_only_substance_units: false,
                    initial_concentration: None,
                    initial_amount: Some(1.0),
                },
                Species {
                    id: "glc_e".to_string(),
                    name: Some("glucose external".to_string()),
                    compartment: "e".to_string(),
                    boundary_condition: true,
                    has_only_substance_units: false,
                    initial_concentration: None,
                    initial_amount: Some(1.0),
                },
            ],
            parameters: vec![],
            reactions: vec![Reaction {
                id: "R1".to_string(),
                name: Some("uptake".to_string()),
                reactants: vec![SpeciesReference {
                    species: "glc_e".to_string(),
                    stoichiometry: Some(1.0),
                    role: "reactant".to_string(),
                }],
                products: vec![SpeciesReference {
                    species: "glc_c".to_string(),
                    stoichiometry: Some(1.0),
                    role: "product".to_string(),
                }],
                reversible: false,
                fast: false,
            }],
        };

        let model_b = Model {
            id: Some("yeast".to_string()),
            name: Some("Yeast".to_string()),
            compartments: vec![
                Compartment {
                    id: "c".to_string(),
                    name: Some("cytosol".to_string()),
                    size: Some(1.0),
                    spatial_dimensions: Some(3),
                },
                Compartment {
                    id: "e".to_string(),
                    name: Some("extracellular".to_string()),
                    size: Some(1.0),
                    spatial_dimensions: Some(3),
                },
            ],
            species: vec![
                Species {
                    id: "glc_c".to_string(),
                    name: Some("glucose".to_string()),
                    compartment: "c".to_string(),
                    boundary_condition: false,
                    has_only_substance_units: false,
                    initial_concentration: None,
                    initial_amount: Some(2.0),
                },
                Species {
                    id: "glc_e".to_string(),
                    name: Some("glucose external".to_string()),
                    compartment: "e".to_string(),
                    boundary_condition: true,
                    has_only_substance_units: false,
                    initial_concentration: None,
                    initial_amount: Some(1.0),
                },
            ],
            parameters: vec![],
            reactions: vec![Reaction {
                id: "R1".to_string(),
                name: Some("uptake".to_string()),
                reactants: vec![SpeciesReference {
                    species: "glc_e".to_string(),
                    stoichiometry: Some(1.0),
                    role: "reactant".to_string(),
                }],
                products: vec![SpeciesReference {
                    species: "glc_c".to_string(),
                    stoichiometry: Some(1.0),
                    role: "product".to_string(),
                }],
                reversible: true,
                fast: false,
            }],
        };

        let community = create_community(None, vec![model_a, model_b]);

        assert_eq!(community.members[0].id, Some("ecoli".to_string()));
        assert_eq!(community.members[1].id, Some("yeast".to_string()));
        assert_eq!(community.members.len(), 2);
        assert_eq!(community.compartments.len(), 3);
        assert_eq!(community.species.len(), 3);
        assert_eq!(community.reactions.len(), 2);

        assert!(community.get_compartments().iter().any(|c| c.id == "m"));
        assert!(community.get_species_by_id("glc_e").is_some());
        assert!(community.get_species_by_id("glc_c__ecoli").is_some());
        assert!(community.get_species_by_id("glc_c__yeast").is_some());
        assert!(community.get_reaction_by_id("R1__ecoli").is_some());
        assert!(community.get_reaction_by_id("R1__yeast").is_some());

        let ecoli_reaction = community.get_reaction_by_id("R1__ecoli").unwrap();
        assert_eq!(ecoli_reaction.reactants[0].species, "glc_e");
        assert_eq!(ecoli_reaction.products[0].species, "glc_c__ecoli");

        let medium_species = community.get_species_by_id("glc_e").unwrap();
        assert_eq!(medium_species.compartment, "m");
    }
}
