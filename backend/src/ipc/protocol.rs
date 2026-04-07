use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Response {
    Ok {
        request_id: String,
    },
    Error {
        request_id: String,
        code: ErrorCode,
        message: String,
    },
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Command {
    Play {
        request_id: String,
    },
    Stop {
        request_id: String,
    },
    SetParam {
        request_id: String,
        path: String,
        value: f64,
    },
    AddNote {
        request_id: String,
        note: Note,
    },
    MoveNote {
        request_id: String,
        id: String,
        pitch: u8,
        start: f64,
    },
    ResizeNote {
        request_id: String,
        length: f64,
        id: String,
    },
    DeleteNote {
        request_id: String,
        id: String,
    },
    PatchReplace {
        request_id: String,
        patch: Patch,
    },
    PatchValidate {
        request_id: String,
    },
    WavetableSet {
        request_id: String,
        osc_id: String,
        table: Vec<f64>,
    },
}

#[derive(Deserialize, Serialize)]
pub enum ErrorCode {
    InvalidMessage,
    UnknownCommand,
    ParamNotFound,
    ParamOutOfRange,
    PatchInvalid,
    EditDeniedPlaying,
    ExportFailed,
    PresetNotFound,
    InternalError,
}

#[derive(Deserialize)]
pub struct Note {
    pub start: f64,
    pub pitch: u8,
    pub velocity: f64,
    pub length: f64,
}

#[derive(Deserialize)]
pub struct Patch {
    pub modules: Vec<Module>,
    pub connections: Vec<Connection>,
}

#[derive(Deserialize)]

pub struct Module {
    pub id: String,
    pub kind: String,
    pub params: Vec<Param>,
}
type Param = HashMap<String, serde_json::Value>;

#[derive(Deserialize)]
pub struct Connection {
    pub from: String,
    pub to: String,
}
