use anyhow::Error;
use lsp_types::{GotoDefinitionParams, GotoDefinitionResponse, Location, Position, Range};

pub fn find_definitions(params: GotoDefinitionParams) -> Result<GotoDefinitionResponse, Error> {
    let locations = vec![Location {
        uri: params.text_document_position_params.text_document.uri,
        range: Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: Position {
                line: 0,
                character: 0,
            },
        },
    }];

    Ok(GotoDefinitionResponse::Array(locations))
}
