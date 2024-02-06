use lsp_types::{GotoDefinitionParams, GotoDefinitionResponse};
use serde::de::Error;

pub fn find_definitions(
    params: GotoDefinitionParams,
) -> Result<GotoDefinitionResponse, Box<dyn Error>> {
    let locations = vec![lsp_types::Location {
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
    }];

    Ok(GotoDefinitionResponse::Array(locations))
}
