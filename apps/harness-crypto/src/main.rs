use std::ffi::OsString;
use std::process::ExitCode;

use harness_crypto::{
    ConfigDeps, ConfigParams, ConfigPayload, ConfigSuccessReturn, OutputDeps, OutputParams,
    OutputPayload, RunDeps, RunParams, RunPayload, SHIPPED_DEFAULTS, admit_generate, admit_output,
    admit_selections, admit_statements, config, layers, output, run,
};

fn main() -> ExitCode {
    let configuration = match config(
        &ConfigDeps {
            layers,
            admit_selections,
            admit_statements,
            admit_generate,
            admit_output,
        },
        ConfigParams {
            arguments: std::env::args_os().collect::<Vec<OsString>>(),
        },
        ConfigPayload {
            defaults: SHIPPED_DEFAULTS.to_owned(),
        },
    ) {
        Ok(ConfigSuccessReturn { configuration }) => configuration,
        Err(error) => {
            eprintln!("{error:?}");
            return ExitCode::FAILURE;
        }
    };

    let destinations = configuration.output.destinations.clone();
    let outcome = run(&RunDeps, RunParams, RunPayload { configuration });
    let succeeded = outcome.is_ok();

    match output(
        &OutputDeps,
        OutputParams { destinations },
        OutputPayload { outcome },
    ) {
        Ok(_) => {
            if succeeded {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
        Err(error) => {
            eprintln!("{error:?}");
            ExitCode::FAILURE
        }
    }
}
