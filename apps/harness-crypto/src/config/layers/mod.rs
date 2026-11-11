#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test_basic;
#[cfg(test)]
mod test_error;

use clap::{CommandFactory, FromArgMatches};
use interface::{
    ConfigArguments, ConfigDocument, LayersDeps, LayersErrorReturn, LayersFn, LayersParams,
    LayersPayload, LayersReturn, LayersSuccessReturn,
};
use std::path::PathBuf;

enum Layer {
    File(PathBuf),
    Assignment(String),
}

pub fn layers(_deps: &LayersDeps, params: LayersParams, payload: LayersPayload) -> LayersReturn {
    let _: LayersFn = layers;

    let command = ConfigArguments::command();
    let matches = match command.try_get_matches_from(params.arguments) {
        Ok(matches) => matches,
        Err(error) => return Err(LayersErrorReturn::Arguments(error)),
    };
    let arguments = match ConfigArguments::from_arg_matches(&matches) {
        Ok(arguments) => arguments,
        Err(error) => return Err(LayersErrorReturn::Arguments(error)),
    };

    let mut ordered: Vec<(usize, Layer)> = Vec::new();
    if let Some(indices) = matches.indices_of("config") {
        for (position, path) in indices.zip(arguments.config) {
            ordered.push((position, Layer::File(path)));
        }
    }
    if let Some(indices) = matches.indices_of("set") {
        for (position, assignment) in indices.zip(arguments.set) {
            ordered.push((position, Layer::Assignment(assignment)));
        }
    }
    ordered.sort_by_key(|(position, _)| *position);

    let mut merged = match toml::from_str::<toml::Table>(&payload.defaults) {
        Ok(table) => table,
        Err(error) => return Err(LayersErrorReturn::ParseDefaults(error)),
    };

    for (_, layer) in ordered {
        match layer {
            Layer::File(path) => {
                let text = match std::fs::read_to_string(&path) {
                    Ok(text) => text,
                    Err(error) => return Err(LayersErrorReturn::ReadFile { path, error }),
                };
                match toml::from_str::<toml::Table>(&text) {
                    Ok(table) => merge_into(&mut merged, table),
                    Err(error) => return Err(LayersErrorReturn::ParseFile { path, error }),
                }
            }
            Layer::Assignment(assignment) => {
                let well_formed = match assignment.split_once('=') {
                    Some((key, _)) => !key.trim().is_empty(),
                    None => false,
                };
                if !well_formed {
                    return Err(LayersErrorReturn::MalformedAssignment { assignment });
                }
                match toml::from_str::<toml::Table>(&assignment) {
                    Ok(table) => merge_into(&mut merged, table),
                    Err(error) => {
                        return Err(LayersErrorReturn::ParseAssignment { assignment, error });
                    }
                }
            }
        }
    }

    let document = match toml::Value::Table(merged).try_into::<ConfigDocument>() {
        Ok(document) => document,
        Err(error) => return Err(LayersErrorReturn::Document(error)),
    };
    Ok(LayersSuccessReturn { document })
}

fn merge_into(base: &mut toml::Table, layer: toml::Table) {
    for (key, value) in layer {
        match (base.get_mut(&key), value) {
            (Some(toml::Value::Table(existing)), toml::Value::Table(incoming)) => {
                merge_into(existing, incoming);
            }
            (_, value) => {
                base.insert(key, value);
            }
        }
    }
}
