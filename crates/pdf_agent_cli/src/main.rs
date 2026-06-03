use clap::Parser;
use pdf_agent_core::config::PipelineConfig;
use pdf_agent_core::context::{PipelineContext, ServiceRegistry};
use pdf_agent_core::pipeline::cancel_token::CancelToken;
use pdf_agent_core::pipeline::converter::PdfConverter;
use pdf_agent_pdf::PdfProvider;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "PPT_Agent_Rust CLI - PDF to Markdown Converter", long_about = None)]
struct Args {
    #[arg(short, long)]
    input: PathBuf,

    #[arg(short, long)]
    output: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("Loading PDF: {:?}", args.input);
    let provider = PdfProvider::open(&args.input)?;

    let mut registry = ServiceRegistry::new();
    let ocr_predictor = std::sync::Arc::new(pdf_agent_inference::predictors::OcrPredictor::new());
    let ocr_service = std::sync::Arc::new(pdf_agent_core::providers::traits::OcrService::new(ocr_predictor));
    registry.register(ocr_service);

    let layout_predictor = std::sync::Arc::new(pdf_agent_inference::predictors::LayoutPredictor::new());
    let layout_service = std::sync::Arc::new(pdf_agent_core::providers::traits::LayoutService::new(layout_predictor));
    registry.register(layout_service);

    let ctx = PipelineContext::new(PipelineConfig::default(), registry);

    let cancel_token = CancelToken::new();
    let converter = PdfConverter::new();

    println!("Starting conversion...");
    let (markdown, _document) = converter
        .convert("cli_job", &provider, &ctx, &cancel_token, None)
        .await?;

    let output_path = args.output.unwrap_or_else(|| args.input.with_extension("md"));

    println!("Writing output to: {:?}", output_path);
    std::fs::write(output_path, markdown)?;

    println!("Conversion completed successfully!");
    Ok(())
}
