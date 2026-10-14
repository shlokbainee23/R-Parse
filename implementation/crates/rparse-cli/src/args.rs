//! Command-line argument parsing and configuration.

use std::path::PathBuf;

use rparse_dataframe::DataFrameSeed;
use rparse_runtime::config::AnalysisMode;

use crate::error::CliError;

/// Desired output format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputFormat {
    /// Human-readable text output.
    #[default]
    Text,
    /// Machine-readable JSON output.
    Json,
    /// Graphviz DOT export format.
    Dot,
    /// Standalone interactive HTML report.
    Html,
}

/// Target graph topology for export during analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphType {
    /// Unified Program Dependence Graph.
    Pdg,
    /// Fine-grained Column Provenance Graph.
    Provenance,
    /// Control-Flow Graph.
    Cfg,
    /// Abstract Syntax Tree / Concrete Syntax Tree Explorer.
    Ast,
}

/// Dispatched CLI command with its parameters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CliCommand {
    /// Run static analysis pipeline on an R script.
    Analyze {
        file: PathBuf,
        format: OutputFormat,
        mode: AnalysisMode,
        seeds: Vec<DataFrameSeed>,
        graph: Option<GraphType>,
        output: Option<PathBuf>,
    },
    /// Show fine-grained column derivation provenance chain.
    Provenance {
        file: PathBuf,
        column: String,
        format: OutputFormat,
        mode: AnalysisMode,
        seeds: Vec<DataFrameSeed>,
        output: Option<PathBuf>,
    },
    /// Compute column-aware or object-level static backward slice.
    Slice {
        file: PathBuf,
        criterion: String,
        format: OutputFormat,
        mode: AnalysisMode,
        seeds: Vec<DataFrameSeed>,
        output: Option<PathBuf>,
    },
    /// Launch the interactive Scientific Analysis Studio local server.
    Studio {
        file: Option<PathBuf>,
        port: Option<u16>,
        open: bool,
    },
    /// Print project version information.
    Version,
    /// Print help information.
    Help(Option<String>),
}

/// Project version string.
pub const RPARSE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Parses command-line arguments into a structured [`CliCommand`].
pub fn parse_args<I: IntoIterator<Item = String>>(args: I) -> Result<CliCommand, CliError> {
    let mut iter = args.into_iter();
    // Skip binary name if present
    let _bin_name = iter.next();

    let first_arg = match iter.next() {
        Some(arg) => arg,
        None => return Ok(CliCommand::Help(None)),
    };

    if first_arg == "--help" || first_arg == "-h" || first_arg == "help" {
        let topic = iter.next();
        return Ok(CliCommand::Help(topic));
    }

    if first_arg == "--version" || first_arg == "-V" || first_arg == "version" {
        return Ok(CliCommand::Version);
    }

    match first_arg.as_str() {
        "analyze" => parse_analyze_args(iter),
        "provenance" => parse_provenance_args(iter),
        "slice" => parse_slice_args(iter),
        "studio" | "serve" => parse_studio_args(iter),
        other => {
            if other.starts_with('-') {
                Err(CliError::Usage(format!("Unknown flag '{}'", other)))
            } else {
                Err(CliError::Usage(format!(
                    "Unknown command '{}'. Expected: 'analyze', 'provenance', 'slice', 'studio'",
                    other
                )))
            }
        }
    }
}

fn parse_studio_args<I: Iterator<Item = String>>(iter: I) -> Result<CliCommand, CliError> {
    let mut file: Option<PathBuf> = None;
    let mut port: Option<u16> = None;
    let mut open = false;

    let args: Vec<String> = iter.collect();
    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--help" || arg == "-h" {
            return Ok(CliCommand::Help(Some("studio".to_string())));
        } else if arg == "--port" || arg == "-p" {
            i += 1;
            if i >= args.len() {
                return Err(CliError::Usage("Missing value for '--port'".to_string()));
            }
            let p = args[i]
                .parse::<u16>()
                .map_err(|_| CliError::Usage(format!("Invalid port number '{}'", args[i])))?;
            port = Some(p);
        } else if let Some(val) = arg.strip_prefix("--port=") {
            let p = val
                .parse::<u16>()
                .map_err(|_| CliError::Usage(format!("Invalid port number '{}'", val)))?;
            port = Some(p);
        } else if arg == "--open" {
            open = true;
        } else if arg.starts_with('-') {
            return Err(CliError::Usage(format!(
                "Unknown flag '{}' for 'studio'",
                arg
            )));
        } else if file.is_none() {
            file = Some(PathBuf::from(arg));
        } else {
            return Err(CliError::Usage(format!("Unexpected argument '{}'", arg)));
        }
        i += 1;
    }

    Ok(CliCommand::Studio { file, port, open })
}

fn parse_analyze_args<I: Iterator<Item = String>>(iter: I) -> Result<CliCommand, CliError> {
    let mut file: Option<PathBuf> = None;
    let mut format = OutputFormat::Text;
    let mut format_specified = false;
    let mut mode = AnalysisMode::Static;
    let mut seeds = Vec::new();
    let mut graph: Option<GraphType> = None;
    let mut output: Option<PathBuf> = None;

    let args: Vec<String> = iter.collect();
    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--help" || arg == "-h" {
            return Ok(CliCommand::Help(Some("analyze".to_string())));
        } else if arg == "--format" {
            i += 1;
            if i >= args.len() {
                return Err(CliError::Usage("Missing value for '--format'".to_string()));
            }
            format = parse_format(&args[i])?;
            format_specified = true;
        } else if let Some(val) = arg.strip_prefix("--format=") {
            format = parse_format(val)?;
            format_specified = true;
        } else if arg == "--output" || arg == "-o" {
            i += 1;
            if i >= args.len() {
                return Err(CliError::Usage("Missing value for '--output'".to_string()));
            }
            output = Some(PathBuf::from(&args[i]));
        } else if let Some(val) = arg.strip_prefix("--output=") {
            output = Some(PathBuf::from(val));
        } else if let Some(val) = arg.strip_prefix("-o=") {
            output = Some(PathBuf::from(val));
        } else if arg == "--graph" || arg == "-g" {
            i += 1;
            if i >= args.len() {
                return Err(CliError::Usage("Missing value for '--graph'".to_string()));
            }
            graph = Some(parse_graph(&args[i])?);
        } else if let Some(val) = arg.strip_prefix("--graph=") {
            graph = Some(parse_graph(val)?);
        } else if arg == "--mode" {
            i += 1;
            if i >= args.len() {
                return Err(CliError::Usage("Missing value for '--mode'".to_string()));
            }
            mode = parse_mode(&args[i])?;
        } else if let Some(val) = arg.strip_prefix("--mode=") {
            mode = parse_mode(val)?;
        } else if arg == "--seed" || arg == "-s" {
            i += 1;
            if i >= args.len() {
                return Err(CliError::Usage("Missing value for '--seed'".to_string()));
            }
            seeds.push(parse_seed(&args[i])?);
        } else if let Some(val) = arg.strip_prefix("--seed=") {
            seeds.push(parse_seed(val)?);
        } else if arg.starts_with('-') && arg != "-" {
            return Err(CliError::Usage(format!(
                "Unknown flag '{}' for 'analyze'",
                arg
            )));
        } else if file.is_none() {
            file = Some(PathBuf::from(arg));
        } else {
            return Err(CliError::Usage(format!(
                "Unexpected positional argument '{}'",
                arg
            )));
        }
        i += 1;
    }

    let file = file.ok_or_else(|| {
        CliError::Usage("Missing required <FILE> argument for 'analyze'".to_string())
    })?;

    if format == OutputFormat::Dot && graph.is_none() {
        return Err(CliError::Usage(
            "'--format dot' requires '--graph <pdg|provenance|cfg>'".to_string(),
        ));
    }

    if graph.is_some() {
        if !format_specified {
            format = OutputFormat::Dot;
        } else if format == OutputFormat::Text {
            return Err(CliError::Usage(
                "'--graph' does not support 'text' format. Use '--format dot' or '--format json'"
                    .to_string(),
            ));
        }
    }

    Ok(CliCommand::Analyze {
        file,
        format,
        mode,
        seeds,
        graph,
        output,
    })
}

fn parse_provenance_args<I: Iterator<Item = String>>(iter: I) -> Result<CliCommand, CliError> {
    let mut file: Option<PathBuf> = None;
    let mut column: Option<String> = None;
    let mut format = OutputFormat::Text;
    let mut mode = AnalysisMode::Static;
    let mut seeds = Vec::new();
    let mut output: Option<PathBuf> = None;

    let args: Vec<String> = iter.collect();
    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--help" || arg == "-h" {
            return Ok(CliCommand::Help(Some("provenance".to_string())));
        } else if arg == "--column" || arg == "-c" {
            i += 1;
            if i >= args.len() {
                return Err(CliError::Usage("Missing value for '--column'".to_string()));
            }
            column = Some(args[i].clone());
        } else if let Some(val) = arg.strip_prefix("--column=") {
            column = Some(val.to_string());
        } else if arg == "--format" {
            i += 1;
            if i >= args.len() {
                return Err(CliError::Usage("Missing value for '--format'".to_string()));
            }
            format = parse_format(&args[i])?;
        } else if let Some(val) = arg.strip_prefix("--format=") {
            format = parse_format(val)?;
        } else if arg == "--output" || arg == "-o" {
            i += 1;
            if i >= args.len() {
                return Err(CliError::Usage("Missing value for '--output'".to_string()));
            }
            output = Some(PathBuf::from(&args[i]));
        } else if let Some(val) = arg.strip_prefix("--output=") {
            output = Some(PathBuf::from(val));
        } else if let Some(val) = arg.strip_prefix("-o=") {
            output = Some(PathBuf::from(val));
        } else if arg == "--mode" {
            i += 1;
            if i >= args.len() {
                return Err(CliError::Usage("Missing value for '--mode'".to_string()));
            }
            mode = parse_mode(&args[i])?;
        } else if let Some(val) = arg.strip_prefix("--mode=") {
            mode = parse_mode(val)?;
        } else if arg == "--seed" || arg == "-s" {
            i += 1;
            if i >= args.len() {
                return Err(CliError::Usage("Missing value for '--seed'".to_string()));
            }
            seeds.push(parse_seed(&args[i])?);
        } else if let Some(val) = arg.strip_prefix("--seed=") {
            seeds.push(parse_seed(val)?);
        } else if arg.starts_with('-') && arg != "-" {
            return Err(CliError::Usage(format!(
                "Unknown flag '{}' for 'provenance'",
                arg
            )));
        } else if file.is_none() {
            file = Some(PathBuf::from(arg));
        } else {
            return Err(CliError::Usage(format!(
                "Unexpected positional argument '{}'",
                arg
            )));
        }
        i += 1;
    }

    let file = file.ok_or_else(|| {
        CliError::Usage("Missing required <FILE> argument for 'provenance'".to_string())
    })?;
    let column = column.ok_or_else(|| {
        CliError::Usage("Missing required '--column <COLUMN>' flag for 'provenance'".to_string())
    })?;

    if format == OutputFormat::Dot {
        return Err(CliError::Usage(
            "Format 'dot' is only supported with 'analyze --graph <pdg|provenance|cfg>'"
                .to_string(),
        ));
    }

    Ok(CliCommand::Provenance {
        file,
        column,
        format,
        mode,
        seeds,
        output,
    })
}

fn parse_slice_args<I: Iterator<Item = String>>(iter: I) -> Result<CliCommand, CliError> {
    let mut file: Option<PathBuf> = None;
    let mut criterion: Option<String> = None;
    let mut format = OutputFormat::Text;
    let mut mode = AnalysisMode::Static;
    let mut seeds = Vec::new();
    let mut output: Option<PathBuf> = None;

    let args: Vec<String> = iter.collect();
    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--help" || arg == "-h" {
            return Ok(CliCommand::Help(Some("slice".to_string())));
        } else if arg == "--criterion" || arg == "-c" {
            i += 1;
            if i >= args.len() {
                return Err(CliError::Usage(
                    "Missing value for '--criterion'".to_string(),
                ));
            }
            criterion = Some(args[i].clone());
        } else if let Some(val) = arg.strip_prefix("--criterion=") {
            criterion = Some(val.to_string());
        } else if arg == "--format" {
            i += 1;
            if i >= args.len() {
                return Err(CliError::Usage("Missing value for '--format'".to_string()));
            }
            format = parse_format(&args[i])?;
        } else if let Some(val) = arg.strip_prefix("--format=") {
            format = parse_format(val)?;
        } else if arg == "--output" || arg == "-o" {
            i += 1;
            if i >= args.len() {
                return Err(CliError::Usage("Missing value for '--output'".to_string()));
            }
            output = Some(PathBuf::from(&args[i]));
        } else if let Some(val) = arg.strip_prefix("--output=") {
            output = Some(PathBuf::from(val));
        } else if let Some(val) = arg.strip_prefix("-o=") {
            output = Some(PathBuf::from(val));
        } else if arg == "--mode" {
            i += 1;
            if i >= args.len() {
                return Err(CliError::Usage("Missing value for '--mode'".to_string()));
            }
            mode = parse_mode(&args[i])?;
        } else if let Some(val) = arg.strip_prefix("--mode=") {
            mode = parse_mode(val)?;
        } else if arg == "--seed" || arg == "-s" {
            i += 1;
            if i >= args.len() {
                return Err(CliError::Usage("Missing value for '--seed'".to_string()));
            }
            seeds.push(parse_seed(&args[i])?);
        } else if let Some(val) = arg.strip_prefix("--seed=") {
            seeds.push(parse_seed(val)?);
        } else if arg.starts_with('-') && arg != "-" {
            return Err(CliError::Usage(format!(
                "Unknown flag '{}' for 'slice'",
                arg
            )));
        } else if file.is_none() {
            file = Some(PathBuf::from(arg));
        } else {
            return Err(CliError::Usage(format!(
                "Unexpected positional argument '{}'",
                arg
            )));
        }
        i += 1;
    }

    let file = file.ok_or_else(|| {
        CliError::Usage("Missing required <FILE> argument for 'slice'".to_string())
    })?;
    let criterion = criterion.ok_or_else(|| {
        CliError::Usage("Missing required '--criterion <CRITERION>' flag for 'slice'".to_string())
    })?;

    if format == OutputFormat::Dot {
        return Err(CliError::Usage(
            "Format 'dot' is only supported with 'analyze --graph <pdg|provenance|cfg>'"
                .to_string(),
        ));
    }

    Ok(CliCommand::Slice {
        file,
        criterion,
        format,
        mode,
        seeds,
        output,
    })
}

fn parse_seed(val: &str) -> Result<DataFrameSeed, CliError> {
    let (sym, cols_str) = if let Some(pair) = val.split_once('=') {
        pair
    } else if let Some(pair) = val.split_once(':') {
        pair
    } else {
        return Err(CliError::Usage(format!(
            "Invalid seed format '{}'. Expected 'df=col1,col2' or 'df:col1,col2'",
            val
        )));
    };
    let cols: Vec<String> = cols_str
        .split(',')
        .map(|c| c.trim().to_string())
        .filter(|c| !c.is_empty())
        .collect();
    Ok(DataFrameSeed::new(sym.trim(), cols))
}

fn parse_format(val: &str) -> Result<OutputFormat, CliError> {
    match val.to_ascii_lowercase().as_str() {
        "text" => Ok(OutputFormat::Text),
        "json" => Ok(OutputFormat::Json),
        "dot" => Ok(OutputFormat::Dot),
        "html" => Ok(OutputFormat::Html),
        other => Err(CliError::Usage(format!(
            "Invalid format '{}'. Expected 'text', 'json', 'dot', or 'html'",
            other
        ))),
    }
}

fn parse_graph(val: &str) -> Result<GraphType, CliError> {
    match val.to_ascii_lowercase().as_str() {
        "pdg" => Ok(GraphType::Pdg),
        "provenance" => Ok(GraphType::Provenance),
        "cfg" => Ok(GraphType::Cfg),
        "ast" => Ok(GraphType::Ast),
        other => Err(CliError::Usage(format!(
            "Invalid graph type '{}'. Expected 'pdg', 'provenance', 'cfg', or 'ast'",
            other
        ))),
    }
}

fn parse_mode(val: &str) -> Result<AnalysisMode, CliError> {
    match val.to_ascii_lowercase().as_str() {
        "static" => Ok(AnalysisMode::Static),
        "assisted" => Ok(AnalysisMode::Assisted),
        "execute" => Ok(AnalysisMode::Execute),
        other => Err(CliError::Usage(format!(
            "Invalid analysis mode '{}'. Expected 'static', 'assisted', or 'execute'",
            other
        ))),
    }
}

/// Generates help text for the top-level CLI or subcommands.
pub fn get_help_text(topic: Option<&str>) -> String {
    match topic {
        Some("analyze") => r#"rparse analyze — Run the static analysis pipeline on an R script

USAGE:
    rparse analyze <FILE> [OPTIONS]

ARGUMENTS:
    <FILE>                          Path to the R script to analyze (or '-' for stdin)

OPTIONS:
    --format <text|json|dot|html>   Output format (default: text, or dot when --graph is used)
    -o, --output <FILE>             Write output to specified file instead of stdout
    --graph <pdg|provenance|cfg>    Graph topology to export (PDG, column provenance, or CFG)
    --mode <MODE>                   Analysis mode: static, assisted, execute (default: static)
    --seed <DF=C1,C2>               Explicit initial dataframe schema seed (e.g. 'df=price,quantity')
    -h, --help                      Print this help information
"#
        .to_string(),


        Some("provenance") => r#"rparse provenance — Inspect fine-grained column derivation lineage

USAGE:
    rparse provenance <FILE> --column <COLUMN> [OPTIONS]

ARGUMENTS:
    <FILE>                          Path to the R script to analyze (or '-' for stdin)

OPTIONS:
    -c, --column <COLUMN>           Target column port (e.g. 'df$total' or 'total') [REQUIRED]
    --format <text|json|html>       Output format (default: text)
    -o, --output <FILE>             Write output to specified file instead of stdout
    --mode <MODE>                   Analysis mode: static, assisted, execute (default: static)
    --seed <DF=C1,C2>               Explicit initial dataframe schema seed (e.g. 'df=price,quantity')
    -h, --help                      Print this help information
"#
        .to_string(),

        Some("slice") => {
            r#"rparse slice — Compute column-aware or object-level backward program slice

USAGE:
    rparse slice <FILE> --criterion <CRITERION> [OPTIONS]

ARGUMENTS:
    <FILE>                          Path to the R script to analyze (or '-' for stdin)

OPTIONS:
    -c, --criterion <CRIT>          Slicing criterion (e.g. 'df$total', 'df', or 'var') [REQUIRED]
    --format <text|json|html>       Output format (default: text)
    -o, --output <FILE>             Write output to specified file instead of stdout
    --mode <MODE>                   Analysis mode: static, assisted, execute (default: static)
    --seed <DF=C1,C2>               Explicit initial dataframe schema seed (e.g. 'df=price,quantity')
    -h, --help                      Print this help information
"#
            .to_string()
        }

        Some("studio") | Some("serve") => {
            r#"rparse studio — Launch interactive Scientific Analysis Studio local server

USAGE:
    rparse studio [FILE] [OPTIONS]

ARGUMENTS:
    [FILE]                          Initial R script to preload in studio (optional)

OPTIONS:
    -p, --port <PORT>               Local port to bind server (default: 8080)
    --open                          Automatically open browser after server start
    -h, --help                      Print this help information
"#
            .to_string()
        }

        _ => format!(
            r#"R-PARSE — Robust R Language Parsing & Program Analysis Engine v{}

USAGE:
    rparse <COMMAND> [OPTIONS]

COMMANDS:
    analyze         Run complete static analysis pipeline and summarize AST/CFG/PDG state
    provenance      Trace fine-grained input-to-output dataframe column derivations
    slice           Compute column-aware static backward slice from targeted criteria
    studio          Launch interactive Scientific Analysis Studio local server (alias: serve)

GLOBAL OPTIONS:
    -h, --help      Print help information (or 'rparse help <COMMAND>')
    -V, --version   Print version information

EXAMPLES:
    rparse analyze script.R
    rparse analyze script.R --format html -o report.html
    rparse provenance script.R --column df$total
    rparse slice script.R --criterion df$total --format json
    rparse studio script.R --open
"#,
            RPARSE_VERSION
        ),
    }
}
