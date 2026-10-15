//! EVCS — Engineering Vector Canvas System
//! A clean-room, cross-platform, infinite-canvas engineering ideation tool.
//! 
//! The binary is dual-mode:
//! * GUI mode (default): launches the `iced` desktop window.
//! * CLI / MCP mode (`--mcp`): exposes a Model Context Protocol server over
//!   `stdio`, letting AI agents inspect and mutate the canvas.

use clap::{Parser, Subcommand};

mod app;
mod engine;
mod mcp;

#[derive(Parser, Debug, Clone)]
#[command(name = "evcs", about = "Engineering Vector Canvas System")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug, Clone)]
enum Command {
    /// Run the embedded MCP server over stdio (agent integration).
    Mcp {
        /// Path to the project workspace to open.
        #[arg(long, short)]
        project: Option<String>,
    },
    /// Export the current workspace to a document format.
    Export {
        /// Output format: svg, png, pdf, json.
        #[arg(long, short, default_value = "svg")]
        format: String,
        /// Input `.evcs` archive path.
        input: String,
        /// Output file path.
        #[arg(long, short)]
        output: String,
    },
    /// Print the current engine version.
    Version,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Mcp { project }) => {
            mcp::run(project).await;
        }
        Some(Command::Export {
            format,
            input,
            output,
        }) => {
            run_export(&format, &input, &output);
        }
        Some(Command::Version) | None => {
            println!("evcs {}", env!("CARGO_PKG_VERSION"));
        }
    }
}

fn run_export(format: &str, input: &str, output: &str) {
    let doc = match engine::open_archive(input) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("error: failed to open archive: {e}");
            std::process::exit(1);
        }
    };

    let bytes = match format {
        "svg" => engine::export_svg(&doc),
        "png" => engine::export_png(&doc),
        "pdf" => engine::export_pdf(&doc),
        "json" => engine::export_json(&doc),
        other => {
            eprintln!("error: unsupported export format '{other}'");
            std::process::exit(1);
        }
    };

    if let Err(e) = std::fs::write(output, bytes) {
        eprintln!("error: failed to write output: {e}");
        std::process::exit(1);
    }
    println!("exported {} -> {}", format.to_uppercase(), output);
}