mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use clap::Parser;
use std::fs::read_to_string;
use std::path::PathBuf;
use toml::{Table, Value};

use interface::{
    ConfigArguments, HarnessConfigDocument, LayersDeps, LayersErrorReturn, LayersParams,
    LayersPayload, LayersReturn, LayersSuccessReturn,
};

pub fn layers(
    _deps: &LayersDeps,
    _params: LayersParams,
    payload: LayersPayload<'_>,
) -> LayersReturn {
    enum MergeStep {
        Table(Vec<String>),
        Insert(Vec<String>, Value),
    }

    let apply_step = |accumulated: Table, step: MergeStep| -> Table {
        let (parents, leaf) = match step {
            MergeStep::Table(path) => (path, None),
            MergeStep::Insert(mut path, value) => {
                let key = path.split_off(path.len() - 1).swap_remove(0);
                (path, Some((key, value)))
            }
        };
        let mut stack: Vec<(String, Table)> = Vec::new();
        let mut current = accumulated;
        for key in parents {
            let next = match current.remove(&key) {
                Some(Value::Table(table)) => table,
                _ => Table::new(),
            };
            stack.push((key, current));
            current = next;
        }
        if let Some((key, value)) = leaf {
            current.insert(key, value);
        }
        while let Some((key, mut table)) = stack.pop() {
            table.insert(key, Value::Table(current));
            current = table;
        }
        current
    };

    let merge = |mut accumulated: Table, overlay: Table| -> Table {
        let mut work = std::collections::VecDeque::from([(Vec::new(), overlay)]);
        while let Some((path, table)) = work.pop_front() {
            for (key, value) in table {
                let mut extended = path.clone();
                extended.push(key);
                match value {
                    Value::Table(inner) => {
                        accumulated = apply_step(accumulated, MergeStep::Table(extended.clone()));
                        work.push_back((extended, inner));
                    }
                    _ => {
                        accumulated = apply_step(accumulated, MergeStep::Insert(extended, value));
                    }
                }
            }
        }
        accumulated
    };

    let arguments =
        ConfigArguments::try_parse_from(payload.arguments).map_err(LayersErrorReturn::Arguments)?;
    let mut accumulated = payload
        .defaults
        .parse::<Table>()
        .map_err(LayersErrorReturn::ParseDefaults)?;
    let files: Vec<PathBuf> = arguments.config;
    for path in files {
        let text = match read_to_string(&path) {
            Ok(text) => text,
            Err(error) => return Err(LayersErrorReturn::ReadFile { path, error }),
        };
        let overlay = match text.parse::<Table>() {
            Ok(table) => table,
            Err(error) => return Err(LayersErrorReturn::ParseFile { path, error }),
        };
        accumulated = merge(accumulated, overlay);
    }
    for (index, text) in arguments.set.into_iter().enumerate() {
        let overlay = match text.parse::<Table>() {
            Ok(table) => table,
            Err(error) => return Err(LayersErrorReturn::ParseAssignment { index, error }),
        };
        let keys = overlay.len();
        if keys != 1 {
            return Err(LayersErrorReturn::AssignmentNotKeyValue { index, keys });
        }
        accumulated = merge(accumulated, overlay);
    }
    let document = accumulated
        .try_into::<HarnessConfigDocument>()
        .map_err(LayersErrorReturn::Document)?;
    Ok(LayersSuccessReturn { document })
}
