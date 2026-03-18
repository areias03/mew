use crate::model::{Compartment, Model, Parameter, Reaction, Species, SpeciesReference};
use std::collections::HashMap;
use std::fs;
use std::io;
use thiserror::Error;
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug, Error)]
pub enum SBMLError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error("XML parsing error: {0}")]
    XmlError(String),
    #[error("Invalid SBML: {0}")]
    InvalidSbml(String),
}

/// Reads an SBML file and parses it into a Model struct
///
/// # Arguments
/// * `filepath` - Path to the SBML file
///
/// # Returns
/// * `Result<Model, SbmlError>` - The parsed model or an error
pub fn read_sbml(filepath: &str) -> Result<Model, SBMLError> {
    let content = fs::read_to_string(filepath)?;
    parse_sbml(&content)
}

fn parse_sbml(xml_content: &str) -> Result<Model, SBMLError> {
    let mut model = Model::new();
    let parser = EventReader::from_str(xml_content);
    let mut current_element: Vec<String> = Vec::new();
    let mut current_reaction: Option<Reaction> = None;
    let mut attr_map: HashMap<String, String> = HashMap::new();

    for event in parser {
        match event.map_err(|e| SBMLError::XmlError(e.to_string()))? {
            XmlEvent::StartElement {
                name, attributes, ..
            } => {
                current_element.push(name.local_name.clone());
                attr_map.clear();
                for attr in attributes {
                    attr_map.insert(attr.name.local_name.clone(), attr.value.clone());
                }

                let path = current_element.join("/");

                match path.as_str() {
                    "sbml/model" => {
                        model.id = attr_map.get("id").cloned();
                        model.name = attr_map.get("name").cloned();
                    }
                    "sbml/model/listOfCompartments/compartment" => {
                        let compartment = Compartment {
                            id: attr_map.get("id").cloned().unwrap_or_default(),
                            name: attr_map.get("name").cloned(),
                            size: attr_map.get("size").and_then(|s| s.parse().ok()),
                            spatial_dimensions: attr_map
                                .get("spatialDimensions")
                                .and_then(|s| s.parse().ok()),
                        };
                        model.compartments.push(compartment);
                    }
                    "sbml/model/listOfSpecies/species" => {
                        let species = Species {
                            id: attr_map.get("id").cloned().unwrap_or_default(),
                            name: attr_map.get("name").cloned(),
                            compartment: attr_map.get("compartment").cloned().unwrap_or_default(),
                            boundary_condition: attr_map
                                .get("boundaryCondition")
                                .map(|v| v == "true")
                                .unwrap_or(false),
                            has_only_substance_units: attr_map
                                .get("hasOnlySubstanceUnits")
                                .map(|v| v == "true")
                                .unwrap_or(false),
                            initial_concentration: attr_map
                                .get("initialConcentration")
                                .and_then(|s| s.parse().ok()),
                            initial_amount: attr_map
                                .get("initialAmount")
                                .and_then(|s| s.parse().ok()),
                        };
                        model.species.push(species);
                    }
                    "sbml/model/listOfParameters/parameter" => {
                        let parameter = Parameter {
                            id: attr_map.get("id").cloned().unwrap_or_default(),
                            name: attr_map.get("name").cloned(),
                            value: attr_map.get("value").and_then(|s| s.parse().ok()),
                            constant: attr_map
                                .get("constant")
                                .map(|v| v == "true")
                                .unwrap_or(true),
                        };
                        model.parameters.push(parameter);
                    }
                    "sbml/model/listOfReactions/reaction" => {
                        current_reaction = Some(Reaction {
                            id: attr_map.get("id").cloned().unwrap_or_default(),
                            name: attr_map.get("name").cloned(),
                            reactants: Vec::new(),
                            products: Vec::new(),
                            reversible: attr_map
                                .get("reversible")
                                .map(|v| v == "true")
                                .unwrap_or(false),
                            fast: attr_map.get("fast").map(|v| v == "true").unwrap_or(false),
                        });
                    }
                    "sbml/model/listOfReactions/reaction/listOfReactants/speciesReference" => {
                        if let Some(ref mut reaction) = current_reaction {
                            let species_ref = SpeciesReference {
                                species: attr_map.get("species").cloned().unwrap_or_default(),
                                stoichiometry: attr_map
                                    .get("stoichiometry")
                                    .and_then(|s| s.parse().ok()),
                                role: "reactant".to_string(),
                            };
                            reaction.reactants.push(species_ref);
                        }
                    }
                    "sbml/model/listOfReactions/reaction/listOfProducts/speciesReference" => {
                        if let Some(ref mut reaction) = current_reaction {
                            let species_ref = SpeciesReference {
                                species: attr_map.get("species").cloned().unwrap_or_default(),
                                stoichiometry: attr_map
                                    .get("stoichiometry")
                                    .and_then(|s| s.parse().ok()),
                                role: "product".to_string(),
                            };
                            reaction.products.push(species_ref);
                        }
                    }
                    _ => {}
                }
            }
            XmlEvent::EndElement { name } => {
                let element_name = name.local_name.clone();

                if let Some(last) = current_element.last() {
                    if last == &element_name {
                        current_element.pop();

                        if element_name == "reaction" {
                            if let Some(reaction) = current_reaction.take() {
                                model.reactions.push(reaction);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Ok(model)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sbml() {
        let sbml = r#"<?xml version="1.0"?>
<sbml xmlns="http://www.sbml.org/sbml/level3/version1/core" level="3" version="1">
  <model id="test_model" name="Test Model">
    <listOfCompartments>
      <compartment id="c1" name="cytoplasm" size="1.0"/>
    </listOfCompartments>
    <listOfSpecies>
      <species id="s1" name="Species1" compartment="c1" initialAmount="10.0"/>
      <species id="s2" name="Species2" compartment="c1" initialConcentration="5.0"/>
    </listOfSpecies>
    <listOfParameters>
      <parameter id="p1" name="k1" value="0.5" constant="true"/>
    </listOfParameters>
    <listOfReactions>
      <reaction id="r1" name="Reaction1" reversible="false" fast="false">
        <listOfReactants>
          <speciesReference species="s1" stoichiometry="1.0"/>
        </listOfReactants>
        <listOfProducts>
          <speciesReference species="s2" stoichiometry="1.0"/>
        </listOfProducts>
      </reaction>
    </listOfReactions>
  </model>
</sbml>"#;

        let model = parse_sbml(sbml).unwrap();

        assert_eq!(model.id, Some("test_model".to_string()));
        assert_eq!(model.name, Some("Test Model".to_string()));
        assert_eq!(model.compartments.len(), 1);
        assert_eq!(model.species.len(), 2);
        assert_eq!(model.parameters.len(), 1);
        assert_eq!(model.reactions.len(), 1);

        assert_eq!(model.compartments[0].id, "c1");
        assert_eq!(model.species[0].id, "s1");
        assert_eq!(model.species[0].initial_amount, Some(10.0));
        assert_eq!(model.reactions[0].reactants.len(), 1);
        assert_eq!(model.reactions[0].products.len(), 1);

        assert_eq!(model.get_species_by_id("s1").unwrap().id, "s1");
        assert_eq!(model.get_reaction_by_id("r1").unwrap().id, "r1");
        assert_eq!(
            model.list_of_species(),
            vec!["s1".to_string(), "s2".to_string()]
        );
        assert_eq!(model.list_of_reactions(), vec!["r1".to_string()]);
        assert_eq!(model.list_of_compartments(), vec!["c1".to_string()]);
    }
}
