use std::collections::HashMap;
use std::fmt;

// ── Value Objects ────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
pub struct ParamValue {
    pub name: String,
    pub value: f64,
    pub min: f64,
    pub max: f64,
}

impl ParamValue {
    pub fn new(name: &str, value: f64, min: f64, max: f64) -> Self {
        Self {
            name: name.to_string(),
            value: value.clamp(min, max),
            min,
            max,
        }
    }

    pub fn set(&mut self, value: f64) -> Result<(), PatchError> {
        if value < self.min || value > self.max {
            return Err(PatchError::ParamOutOfRange {
                name: self.name.clone(),
                min: self.min,
                max: self.max,
                got: value,
            });
        }
        self.value = value;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PortId {
    pub module_id: String,
    pub port_name: String,
}

impl PortId {
    pub fn new(module_id: &str, port_name: &str) -> Self {
        Self {
            module_id: module_id.to_string(),
            port_name: port_name.to_string(),
        }
    }

    pub fn parse(s: &str) -> Result<Self, PatchError> {
        let parts: Vec<&str> = s.splitn(2, '.').collect();
        if parts.len() != 2 {
            return Err(PatchError::InvalidPortId(s.to_string()));
        }
        Ok(Self::new(parts[0], parts[1]))
    }
}

// ── Enums ────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
pub enum ModuleKind {
    Oscillator,
    Filter,
    Envelope,
    Output,
    Mixer,
}

impl ModuleKind {
    pub fn from_str(s: &str) -> Result<Self, PatchError> {
        match s.to_lowercase().as_str() {
            "oscillator" => Ok(ModuleKind::Oscillator),
            "filter" => Ok(ModuleKind::Filter),
            "envelope" => Ok(ModuleKind::Envelope),
            "output" => Ok(ModuleKind::Output),
            "mixer" => Ok(ModuleKind::Mixer),
            _ => Err(PatchError::UnknownModuleKind(s.to_string())),
        }
    }

    pub fn default_params(&self) -> Vec<ParamValue> {
        match self {
            ModuleKind::Oscillator => vec![
                ParamValue::new("frequency", 440.0, 20.0, 20000.0),
                ParamValue::new("gain", 0.8, 0.0, 1.0),
            ],
            ModuleKind::Filter => vec![
                ParamValue::new("cutoff", 1000.0, 20.0, 20000.0),
                ParamValue::new("resonance", 0.5, 0.0, 1.0),
            ],
            ModuleKind::Envelope => vec![
                ParamValue::new("attack", 0.01, 0.001, 5.0),
                ParamValue::new("decay", 0.1, 0.001, 5.0),
                ParamValue::new("sustain", 0.7, 0.0, 1.0),
                ParamValue::new("release", 0.3, 0.001, 10.0),
            ],
            ModuleKind::Output => vec![
                ParamValue::new("gain", 0.8, 0.0, 1.0),
            ],
            ModuleKind::Mixer => vec![
                ParamValue::new("gain", 0.5, 0.0, 1.0),
            ],
        }
    }
}

// ── Entities ─────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct Module {
    pub id: String,
    pub kind: ModuleKind,
    pub params: Vec<ParamValue>,
}

impl Module {
    pub fn new(id: &str, kind: ModuleKind) -> Self {
        let params = kind.default_params();
        Self {
            id: id.to_string(),
            kind,
            params,
        }
    }

    pub fn with_params(id: &str, kind: ModuleKind, params: Vec<ParamValue>) -> Self {
        Self {
            id: id.to_string(),
            kind,
            params,
        }
    }

    pub fn get_param(&self, name: &str) -> Option<&ParamValue> {
        self.params.iter().find(|p| p.name == name)
    }

    pub fn set_param(&mut self, name: &str, value: f64) -> Result<(), PatchError> {
        let param = self.params.iter_mut().find(|p| p.name == name);
        match param {
            Some(p) => p.set(value),
            None => Err(PatchError::ParamNotFound {
                module_id: self.id.clone(),
                param_name: name.to_string(),
            }),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Connection {
    pub from: PortId,
    pub to: PortId,
}

impl Connection {
    pub fn new(from: PortId, to: PortId) -> Self {
        Self { from, to }
    }
}

// ── Aggregate Root : Patch ───────────────────────────────────────

#[derive(Clone, Debug)]
pub struct Patch {
    modules: Vec<Module>,
    connections: Vec<Connection>,
}

impl Patch {
    pub fn modules(&self) -> &[Module] {
        &self.modules
    }

    pub fn connections(&self) -> &[Connection] {
        &self.connections
    }

    pub fn find_module(&self, id: &str) -> Option<&Module> {
        self.modules.iter().find(|m| m.id == id)
    }

    pub fn find_module_mut(&mut self, id: &str) -> Option<&mut Module> {
        self.modules.iter_mut().find(|m| m.id == id)
    }

    pub fn set_param(&mut self, path: &str, value: f64) -> Result<(), PatchError> {
        let port = PortId::parse(path)?;
        let module = self.find_module_mut(&port.module_id).ok_or_else(|| {
            PatchError::ModuleNotFound(port.module_id.clone())
        })?;
        module.set_param(&port.port_name, value)
    }

    pub fn module_ids(&self) -> Vec<&str> {
        self.modules.iter().map(|m| m.id.as_str()).collect()
    }

    pub fn adjacency(&self) -> HashMap<String, Vec<String>> {
        let mut adj: HashMap<String, Vec<String>> = HashMap::new();
        for m in &self.modules {
            adj.entry(m.id.clone()).or_default();
        }
        for c in &self.connections {
            adj.entry(c.from.module_id.clone())
                .or_default()
                .push(c.to.module_id.clone());
        }
        adj
    }
}

// ── Builder Pattern (Création) ───────────────────────────────────

pub struct PatchBuilder {
    modules: Vec<Module>,
    connections: Vec<Connection>,
}

impl PatchBuilder {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
            connections: Vec::new(),
        }
    }

    pub fn add_module(mut self, module: Module) -> Self {
        self.modules.push(module);
        self
    }

    pub fn add_connection(mut self, connection: Connection) -> Self {
        self.connections.push(connection);
        self
    }

    pub fn build(self) -> Result<Patch, PatchError> {
        let patch = Patch {
            modules: self.modules,
            connections: self.connections,
        };
        Ok(patch)
    }
}

// ── Anti-corruption layer : IPC → Domain ─────────────────────────

impl Patch {
    pub fn from_ipc(ipc_patch: &crate::ipc::protocol::Patch) -> Result<Self, PatchError> {
        let mut builder = PatchBuilder::new();

        for ipc_module in &ipc_patch.modules {
            let kind = ModuleKind::from_str(&ipc_module.kind)?;
            let mut module = Module::new(&ipc_module.id, kind);

            for param_map in &ipc_module.params {
                for (name, value) in param_map {
                    if let Some(v) = value.as_f64() {
                        module.set_param(name, v).ok();
                    }
                }
            }

            builder = builder.add_module(module);
        }

        for ipc_conn in &ipc_patch.connections {
            let from = PortId::parse(&ipc_conn.from)?;
            let to = PortId::parse(&ipc_conn.to)?;
            builder = builder.add_connection(Connection::new(from, to));
        }

        builder.build()
    }
}

// ── Errors ───────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum PatchError {
    ModuleNotFound(String),
    ParamNotFound { module_id: String, param_name: String },
    ParamOutOfRange { name: String, min: f64, max: f64, got: f64 },
    InvalidPortId(String),
    UnknownModuleKind(String),
    ValidationFailed(Vec<String>),
}

impl fmt::Display for PatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PatchError::ModuleNotFound(id) => write!(f, "Module not found: {}", id),
            PatchError::ParamNotFound { module_id, param_name } => {
                write!(f, "Param '{}' not found on module '{}'", param_name, module_id)
            }
            PatchError::ParamOutOfRange { name, min, max, got } => {
                write!(f, "Param '{}' out of range [{}, {}], got {}", name, min, max, got)
            }
            PatchError::InvalidPortId(s) => write!(f, "Invalid port id: '{}'", s),
            PatchError::UnknownModuleKind(s) => write!(f, "Unknown module kind: '{}'", s),
            PatchError::ValidationFailed(errors) => {
                write!(f, "Validation failed: {}", errors.join("; "))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patch_builder_simple() {
        let patch = PatchBuilder::new()
            .add_module(Module::new("osc1", ModuleKind::Oscillator))
            .add_module(Module::new("out1", ModuleKind::Output))
            .add_connection(Connection::new(
                PortId::new("osc1", "output"),
                PortId::new("out1", "input"),
            ))
            .build()
            .unwrap();

        assert_eq!(patch.modules().len(), 2);
        assert_eq!(patch.connections().len(), 1);
    }

    #[test]
    fn test_find_module() {
        let patch = PatchBuilder::new()
            .add_module(Module::new("osc1", ModuleKind::Oscillator))
            .build()
            .unwrap();

        assert!(patch.find_module("osc1").is_some());
        assert!(patch.find_module("nonexistent").is_none());
    }

    #[test]
    fn test_set_param_success() {
        let mut patch = PatchBuilder::new()
            .add_module(Module::new("osc1", ModuleKind::Oscillator))
            .build()
            .unwrap();

        assert!(patch.set_param("osc1.frequency", 880.0).is_ok());
        let freq = patch.find_module("osc1").unwrap().get_param("frequency").unwrap();
        assert!((freq.value - 880.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_set_param_module_not_found() {
        let mut patch = PatchBuilder::new()
            .add_module(Module::new("osc1", ModuleKind::Oscillator))
            .build()
            .unwrap();

        assert!(matches!(
            patch.set_param("unknown.frequency", 440.0),
            Err(PatchError::ModuleNotFound(_))
        ));
    }

    #[test]
    fn test_set_param_not_found() {
        let mut patch = PatchBuilder::new()
            .add_module(Module::new("osc1", ModuleKind::Oscillator))
            .build()
            .unwrap();

        assert!(matches!(
            patch.set_param("osc1.unknown_param", 1.0),
            Err(PatchError::ParamNotFound { .. })
        ));
    }

    #[test]
    fn test_set_param_out_of_range() {
        let mut patch = PatchBuilder::new()
            .add_module(Module::new("osc1", ModuleKind::Oscillator))
            .build()
            .unwrap();

        assert!(matches!(
            patch.set_param("osc1.gain", 5.0),
            Err(PatchError::ParamOutOfRange { .. })
        ));
    }

    #[test]
    fn test_module_kind_from_str() {
        assert_eq!(ModuleKind::from_str("oscillator").unwrap(), ModuleKind::Oscillator);
        assert_eq!(ModuleKind::from_str("Filter").unwrap(), ModuleKind::Filter);
        assert!(ModuleKind::from_str("invalid").is_err());
    }

    #[test]
    fn test_port_id_parse() {
        let port = PortId::parse("osc1.output").unwrap();
        assert_eq!(port.module_id, "osc1");
        assert_eq!(port.port_name, "output");
        assert!(PortId::parse("invalid").is_err());
    }
}
