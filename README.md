# Web to Markdown Converter

This tool fetches the content of a web page, extracts the content within the `<main>` tags, and converts it to Markdown format.

## Installation

To install this tool from GitHub using Cargo, follow these steps:

1. Make sure you have Rust and Cargo installed on your system. If not, you can install them from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

2. Open a terminal and run the following command:

   ```
   cargo install --git https://github.com/ghardin1314/web2md.git
   ```

3. Cargo will download the source code, compile the project, and install the binary in your system.

## Usage

After installation, you can use the tool from the command line:

### Arguments:

- `<URL>`: The web page URL you want to convert to Markdown. This is a required argument.
- `--out <OUTPUT_DIRECTORY>`: (Optional) Specify an output directory for the Markdown file. If not provided, the content will be printed to the console.

### Examples:

1. Convert a webpage and print the Markdown to the console:

   ```
   web2md https://example.com
   ```

2. Convert a webpage and save the Markdown to a specific directory:
   ```
   web2md https://example.com --out ./output
   ```

### Features:

- Extracts content from within `<main>` tags of the webpage.
- Removes CSS styles and `<style>` tags from the HTML before conversion.
- Sanitizes the URL to create a valid filename when saving to a file.
- Creates the output directory if it doesn't exist.

### Output:

- If an output directory is specified, the tool will save the Markdown content to a file named after the sanitized URL (e.g., `example-com.md`).
- If no output directory is specified, the Markdown content will be printed to the console.

### Error Handling:

- The tool will display appropriate error messages for invalid URLs, network issues, or file system errors.
- If no `<main>` tags are found in the document, it will inform the user.

## Development

To contribute to this project or run it from source:

1. Clone the repository:

   ```
   git clone https://github.com/ghardin1314/web2md.git
   ```

2. Navigate to the project directory:

   ```
   cd your-repo-name
   ```

3. Build the project:

   ```
   cargo build
   ```

4. Run the project:
   ```
   cargo run -- <URL> [--out <OUTPUT_DIRECTORY>]
   ```
