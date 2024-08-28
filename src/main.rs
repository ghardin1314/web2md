use clap::Parser;
use url::Url;
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use regex::Regex;
use html2md::parse_html;
use std::path::PathBuf;
use std::fs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// URL to process
    url: String,

    /// Output directory for Markdown file
    #[arg(short, long)]
    out: Option<PathBuf>,
}

fn remove_css(html: &str) -> String {
    // Remove <style> tags
    let re_style = Regex::new(r"<style[^>]*>[\s\S]*?</style>").unwrap();
    let html_without_style = re_style.replace_all(html, "");

    // Remove style attributes
    let re_style_attr = Regex::new(r#"\s*style\s*=\s*"[^"]*""#).unwrap();
    re_style_attr.replace_all(&html_without_style, "").to_string()
}

fn sanitize_filename(url: &str) -> String {
    url.trim_start_matches("https://")
       .trim_start_matches("http://")
       .replace('.', "-")
       .replace('/', "_")
       .replace(':', "_")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Validate the URL
    let valid_url = Url::parse(&cli.url)?;

    // Fetch the HTML content
    let client = Client::new();
    let response = client.get(valid_url.clone()).send()?;
    let html_content = response.text()?;

    // Parse the HTML
    let document = Html::parse_document(&html_content);
    let main_selector = Selector::parse("main").unwrap();

    // Find and process the content inside <main> tags
    if let Some(main_element) = document.select(&main_selector).next() {
        let main_content = main_element.inner_html();
        let cleaned_content = remove_css(&main_content);
        let markdown_content = parse_html(&cleaned_content);
        
        if let Some(out_dir) = cli.out {
            // Create the output directory if it doesn't exist
            fs::create_dir_all(&out_dir)?;

            // Generate a filename based on the full URL, without the protocol
            let filename = sanitize_filename(&valid_url.as_str()) + ".md";
            let file_path = out_dir.join(filename);

            // Write the Markdown content to the file
            fs::write(&file_path, markdown_content)?;
            println!("Markdown content saved to: {:?}", file_path);
        } else {
            println!("Content inside <main> tags (converted to Markdown):");
            println!("{}", markdown_content);
        }
    } else {
        println!("No <main> tags found in the document.");
    }

    Ok(())
}
