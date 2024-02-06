use lsp_types::request::GotoDefinitionParams;
use lsp_types::Locations;
use std::error::Error;

pub fn GoToDefinition(params: GotoDefinitionParams) -> Result<Vec<Location>, Box<dyn Error>> {
    let locations = vec![
        lsp_types::Location {
            uri: "file:///path/to/file".to_string(),
            range: lsp_types::Range {
                start: lsp_types::Position {
                    line: 1,
                    character: 1,
                },
                end: lsp_types::Position {
                    line: 1,
                    character: 1,
                },
            },
        },
    ];

    Ok(locations)
}
