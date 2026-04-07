use std::collections::HashMap;

use super::model::{ModuleKind, Patch};

pub struct PatchValidator;

#[derive(PartialEq)]
enum NodeState {
    Unvisited,
    InProgress,
    Done,
}

impl PatchValidator {

    pub fn validate(patch: &Patch) -> Result<(), ValidationError> {
        Self::check_output(patch)?;
        Self::check_unknown_modules(patch)?;
        Self::check_cycles(patch)?;
        Ok(())

    }

    fn check_output(patch: &Patch) -> Result<(), ValidationError> {
       let count = patch.modules()
        .iter()
        .filter(|m| m.kind == ModuleKind::Output)
        .count();

        match count {
            0 => Err(ValidationError::NoOutputModule),
            1 => Ok(()),
            _ => Err(ValidationError::MultipleOutputModules),
        }
    }

    fn check_unknown_modules(patch: &Patch) -> Result<(), ValidationError> {
        for conn in patch.connections() {
            if patch.find_module(&conn.from.module_id).is_none() {
                return Err(ValidationError::UnknownModule(conn.from.module_id.clone()));
            }
            if patch.find_module(&conn.to.module_id).is_none() {
                return Err(ValidationError::UnknownModule(conn.to.module_id.clone()));
            }
        }
        Ok(())
    }

    fn check_cycles(patch: &Patch) -> Result<(), ValidationError> {
        let adj = patch.adjacency();
        let mut states: HashMap<String, NodeState> = adj.keys()
            .map(|k| (k.clone(), NodeState::Unvisited))
            .collect();

        for node in adj.keys() {
            if states[node] == NodeState::Unvisited {
                Self::dfs(node, &adj, &mut states)?;
            }
        }
        Ok(())
    }

    fn dfs(
        node: &str,
        adj: &HashMap<String, Vec<String>>,
        states: &mut HashMap<String, NodeState>,
    ) -> Result<(), ValidationError> {
        states.insert(node.to_string(), NodeState::InProgress);
        if let Some(neighbors) = adj.get(node) {
            for neighbor in neighbors {
                match states.get(neighbor.as_str()) {
                    Some(NodeState::InProgress) => return Err(ValidationError::CycleDetected),
                    Some(NodeState::Unvisited) => Self::dfs(neighbor, adj, states)?,
                    _ => (),
                }
            }
        }
        states.insert(node.to_string(), NodeState::Done);
        Ok(())
    }


}

#[derive(Debug)]
pub enum ValidationError {
    NoOutputModule,
    MultipleOutputModules,
    CycleDetected,
    UnknownModule(String),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::NoOutputModule => write!(f, "Patch must have exactly one output module"),
            ValidationError::MultipleOutputModules => write!(f, "Patch cannot have multiple output modules"),
            ValidationError::CycleDetected => write!(f, "Patch cannot contain cycles"),
            ValidationError::UnknownModule(id) => write!(f, "Connection references unknown module '{}'", id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::patch::model::{Connection, Module, ModuleKind, PatchBuilder, PortId};

    fn osc_out_patch() -> crate::domain::patch::model::Patch {
        PatchBuilder::new()
            .add_module(Module::new("osc1", ModuleKind::Oscillator))
            .add_module(Module::new("out1", ModuleKind::Output))
            .add_connection(Connection::new(
                PortId::new("osc1", "output"),
                PortId::new("out1", "input"),
            ))
            .build()
            .unwrap()
    }

    #[test]
    fn valid_patch_passes() {
        assert!(PatchValidator::validate(&osc_out_patch()).is_ok());
    }

    #[test]
    fn no_output_module_fails() {
        let patch = PatchBuilder::new()
            .add_module(Module::new("osc1", ModuleKind::Oscillator))
            .build()
            .unwrap();
        assert!(matches!(
            PatchValidator::validate(&patch),
            Err(ValidationError::NoOutputModule)
        ));
    }

    #[test]
    fn multiple_output_modules_fail() {
        let patch = PatchBuilder::new()
            .add_module(Module::new("out1", ModuleKind::Output))
            .add_module(Module::new("out2", ModuleKind::Output))
            .build()
            .unwrap();
        assert!(matches!(
            PatchValidator::validate(&patch),
            Err(ValidationError::MultipleOutputModules)
        ));
    }

    #[test]
    fn unknown_module_in_connection_fails() {
        let patch = PatchBuilder::new()
            .add_module(Module::new("out1", ModuleKind::Output))
            .add_connection(Connection::new(
                PortId::new("ghost", "output"),
                PortId::new("out1", "input"),
            ))
            .build()
            .unwrap();
        assert!(matches!(
            PatchValidator::validate(&patch),
            Err(ValidationError::UnknownModule(_))
        ));
    }

    #[test]
    fn cycle_detection_fails() {
        let patch = PatchBuilder::new()
            .add_module(Module::new("a", ModuleKind::Filter))
            .add_module(Module::new("b", ModuleKind::Filter))
            .add_module(Module::new("out1", ModuleKind::Output))
            .add_connection(Connection::new(PortId::new("a", "out"), PortId::new("b", "in")))
            .add_connection(Connection::new(PortId::new("b", "out"), PortId::new("a", "in")))
            .add_connection(Connection::new(PortId::new("b", "out"), PortId::new("out1", "in")))
            .build()
            .unwrap();
        assert!(matches!(
            PatchValidator::validate(&patch),
            Err(ValidationError::CycleDetected)
        ));
    }
}