//! `rparse-cli`: Command-line interface and unified pipeline orchestration for R-PARSE.

pub mod args;
pub mod error;
pub mod exit_code;
pub mod format_ast;
pub mod format_html;
pub mod format_html_assets;
pub mod format_json;
pub mod format_text;
pub mod pipeline;
pub mod provenance;
pub mod server;

#[cfg(test)]
mod tests;

use rparse_slicing::{compute_backward_slice_with_source, SliceCriterion};

use crate::args::{get_help_text, parse_args, CliCommand, GraphType, OutputFormat, RPARSE_VERSION};
use crate::error::CliError;
use crate::exit_code::*;
use crate::format_html::format_html_report;
use crate::format_json::{
    format_analyze_json, format_error_json, format_provenance_json, format_slice_json,
};
use crate::format_text::{format_analyze_text, format_provenance_text, format_slice_text};
use crate::pipeline::AnalysisContext;
use crate::provenance::trace_column_provenance;

use std::io::Read;

/// Top-level entry point executing command-line arguments and returning an exit code and formatted output string.
pub fn run_cli<I: IntoIterator<Item = String>>(args: I) -> (i32, String) {
    run_cli_with_reader(args, std::io::stdin())
}

/// Entry point executing command-line arguments with a custom reader for stdin.
pub fn run_cli_with_reader<I: IntoIterator<Item = String>, R: Read>(
    args: I,
    reader: R,
) -> (i32, String) {
    let command = match parse_args(args) {
        Ok(cmd) => cmd,
        Err(err) => {
            let code = err.exit_code();
            let msg = format!(
                "Error: {}\n\nRun 'rparse --help' for usage instructions.",
                err
            );
            return (code, msg);
        }
    };

    match command {
        CliCommand::Help(topic) => (EXIT_SUCCESS, get_help_text(topic.as_deref())),

        CliCommand::Version => (EXIT_SUCCESS, format!("rparse {}", RPARSE_VERSION)),

        CliCommand::Analyze {
            file,
            format,
            mode,
            seeds,
            graph,
            output,
        } => {
            let ctx = match AnalysisContext::run_with_reader(&file, reader, mode, seeds) {
                Ok(c) => c,
                Err(err) => {
                    let out = match format {
                        OutputFormat::Text | OutputFormat::Dot | OutputFormat::Html => {
                            format!("Error: {}", err)
                        }
                        OutputFormat::Json => {
                            format_error_json("analyze", Some(&file.to_string_lossy()), &err)
                        }
                    };
                    return (err.exit_code(), out);
                }
            };

            let out = match (graph, format) {
                (_, OutputFormat::Html) => format_html_report(&ctx, None, None, graph),
                (Some(GraphType::Pdg), OutputFormat::Dot) => ctx.pdg.to_dot(),
                (Some(GraphType::Pdg), OutputFormat::Json) => ctx.pdg.to_json(),
                (Some(GraphType::Provenance), OutputFormat::Dot) => {
                    ctx.df_analysis.provenance_to_dot()
                }
                (Some(GraphType::Provenance), OutputFormat::Json) => {
                    ctx.df_analysis.provenance_to_json()
                }
                (Some(GraphType::Cfg), OutputFormat::Dot) => ctx.cfg.to_dot(),
                (Some(GraphType::Cfg), OutputFormat::Json) => ctx.cfg.to_json(),
                (None, OutputFormat::Json) => format_analyze_json(&ctx),
                (None, OutputFormat::Text) => format_analyze_text(&ctx),
                _ => unreachable!(
                    "Invalid format/graph combinations are rejected during argument parsing"
                ),
            };

            let code = if ctx.has_errors() {
                EXIT_ANALYSIS_FAILURE
            } else {
                EXIT_SUCCESS
            };

            if let Some(out_path) = output {
                if let Err(err) = std::fs::write(&out_path, &out) {
                    return (
                        EXIT_ANALYSIS_FAILURE,
                        format!(
                            "Error: Failed to write output file '{}': {}",
                            out_path.display(),
                            err
                        ),
                    );
                }
            }

            (code, out)
        }

        CliCommand::Provenance {
            file,
            column,
            format,
            mode,
            seeds,
            output,
        } => {
            let ctx = match AnalysisContext::run_with_reader(&file, reader, mode, seeds) {
                Ok(c) => c,
                Err(err) => {
                    let out = match format {
                        OutputFormat::Text | OutputFormat::Dot | OutputFormat::Html => {
                            format!("Error: {}", err)
                        }
                        OutputFormat::Json => {
                            format_error_json("provenance", Some(&file.to_string_lossy()), &err)
                        }
                    };
                    return (err.exit_code(), out);
                }
            };

            let prov_res = match trace_column_provenance(&column, &ctx) {
                Ok(res) => res,
                Err(err) => {
                    let out = match format {
                        OutputFormat::Text | OutputFormat::Dot | OutputFormat::Html => {
                            format!("Error: {}", err)
                        }
                        OutputFormat::Json => {
                            format_error_json("provenance", Some(&file.to_string_lossy()), &err)
                        }
                    };
                    return (err.exit_code(), out);
                }
            };

            let out = match format {
                OutputFormat::Html => {
                    format_html_report(&ctx, None, Some(&column), Some(GraphType::Provenance))
                }
                OutputFormat::Text | OutputFormat::Dot => format_provenance_text(&prov_res),
                OutputFormat::Json => format_provenance_json(&prov_res, &ctx),
            };

            if let Some(out_path) = output {
                if let Err(err) = std::fs::write(&out_path, &out) {
                    return (
                        EXIT_ANALYSIS_FAILURE,
                        format!(
                            "Error: Failed to write output file '{}': {}",
                            out_path.display(),
                            err
                        ),
                    );
                }
            }

            (EXIT_SUCCESS, out)
        }

        CliCommand::Slice {
            file,
            criterion,
            format,
            mode,
            seeds,
            output,
        } => {
            let ctx = match AnalysisContext::run_with_reader(&file, reader, mode, seeds) {
                Ok(c) => c,
                Err(err) => {
                    let out = match format {
                        OutputFormat::Text | OutputFormat::Dot | OutputFormat::Html => {
                            format!("Error: {}", err)
                        }
                        OutputFormat::Json => {
                            format_error_json("slice", Some(&file.to_string_lossy()), &err)
                        }
                    };
                    return (err.exit_code(), out);
                }
            };

            let slice_crit = parse_slice_criterion(&criterion, &ctx);
            let slice_res = match compute_backward_slice_with_source(
                &slice_crit,
                &ctx.pdg,
                &ctx.df_analysis,
                &ctx.scope_model,
                Some(&ctx.program),
                Some(ctx.source_file.content()),
            ) {
                Ok(res) => res,
                Err(err) => {
                    let cli_err = CliError::AnalysisFailed(err.to_string());
                    let out = match format {
                        OutputFormat::Text | OutputFormat::Dot | OutputFormat::Html => {
                            format!("Error: {}", cli_err)
                        }
                        OutputFormat::Json => {
                            format_error_json("slice", Some(&file.to_string_lossy()), &cli_err)
                        }
                    };
                    return (cli_err.exit_code(), out);
                }
            };

            let out = match format {
                OutputFormat::Html => format_html_report(&ctx, Some(&criterion), None, None),
                OutputFormat::Text | OutputFormat::Dot => {
                    format_slice_text(&slice_res, &slice_crit, &ctx)
                }
                OutputFormat::Json => format_slice_json(&slice_res, &slice_crit, &ctx),
            };

            if let Some(out_path) = output {
                if let Err(err) = std::fs::write(&out_path, &out) {
                    return (
                        EXIT_ANALYSIS_FAILURE,
                        format!(
                            "Error: Failed to write output file '{}': {}",
                            out_path.display(),
                            err
                        ),
                    );
                }
            }

            (EXIT_SUCCESS, out)
        }

        CliCommand::Studio { file, port, open } => {
            crate::server::run_studio_server(file, port, open)
        }
    }
}

pub(crate) fn parse_slice_criterion(crit_str: &str, ctx: &AnalysisContext) -> SliceCriterion {
    let trimmed = crit_str.trim();
    if let Some((df, col)) = trimmed.split_once('$') {
        SliceCriterion::ColumnByName {
            dataframe_symbol: df.trim().to_string(),
            column_name: col.trim().to_string(),
        }
    } else if ctx.df_analysis.active_symbol_states.contains_key(trimmed) {
        SliceCriterion::WholeDataFrameByName {
            dataframe_symbol: trimmed.to_string(),
        }
    } else {
        SliceCriterion::ScalarByName {
            symbol_name: trimmed.to_string(),
        }
    }
}
