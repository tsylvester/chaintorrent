mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use interface::{
    AdmitOutputDeps, AdmitOutputErrorReturn, AdmitOutputParams, AdmitOutputPayload,
    AdmitOutputReturn, AdmitOutputSuccessReturn, OutputDestination,
};

pub fn admit_output(
    _deps: &AdmitOutputDeps,
    _params: AdmitOutputParams,
    payload: AdmitOutputPayload<'_>,
) -> AdmitOutputReturn {
    let output = payload.output;
    if output.destinations.is_empty() {
        return Err(AdmitOutputErrorReturn::Empty);
    }
    let mut destinations = Vec::with_capacity(output.destinations.len());
    for (index, entry) in output.destinations.iter().enumerate() {
        destinations.push(match (entry.kind.as_str(), &entry.path) {
            ("standard_output", None) => OutputDestination::StandardOutput,
            ("standard_output", Some(_)) => {
                return Err(AdmitOutputErrorReturn::UnexpectedPath { index });
            }
            ("file", Some(path)) => OutputDestination::File { path: path.clone() },
            ("file", None) => {
                return Err(AdmitOutputErrorReturn::MissingPath { index });
            }
            _ => {
                return Err(AdmitOutputErrorReturn::UnknownKind {
                    index,
                    text: entry.kind.clone(),
                });
            }
        });
    }
    Ok(AdmitOutputSuccessReturn { destinations })
}
