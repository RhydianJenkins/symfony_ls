use std::error::Error;

use lsp_types::{
    request::GotoDefinition, GotoDefinitionResponse, InitializeParams, ServerCapabilities,
};
use lsp_types::{HoverProviderCapability, Location, OneOf, Position, Range, Url};

use lsp_server::{Connection, ExtractError, Message, Request, RequestId, Response};

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let (connection, io_threads) = Connection::stdio();
    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        definition_provider: Some(OneOf::Left(true)),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        ..Default::default()
    })
    .unwrap();
    let initialization_params = match connection.initialize(server_capabilities) {
        Ok(it) => it,
        Err(e) => {
            if e.channel_is_disconnected() {
                io_threads.join()?;
            }
            return Err(e.into());
        }
    };

    main_loop(connection, initialization_params)?;

    io_threads.join()?;

    Ok(())
}

fn main_loop(
    connection: Connection,
    params: serde_json::Value,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let locations: Vec<lsp_types::Location> = vec![
        Location {
            uri: Url::parse("file:/home/rhydian/dotfiles/nvim/after/plugin/lspconfig.lua")?,
            range: Range {
                start: Position {
                    line: 1,
                    character: 1,
                },
                end: Position {
                    line: 1,
                    character: 1,
                },
            },
        },
        Location {
            uri: Url::parse("file:/home/rhydian/dotfiles/nvim/after/plugin/lspconfig.lua")?,
            range: Range {
                start: Position {
                    line: 1,
                    character: 1,
                },
                end: Position {
                    line: 1,
                    character: 1,
                },
            },
        },
    ];

    let _params: InitializeParams = serde_json::from_value(params).unwrap();
    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                match cast::<GotoDefinition>(req.clone()) {
                    Ok((id, params)) => {
                        eprintln!("got gotoDefinition request #{id}: {params:?}");
                        let result = Some(GotoDefinitionResponse::Array(locations.clone()));
                        let result = serde_json::to_value(&result).unwrap();
                        let resp = Response {
                            id,
                            result: Some(result),
                            error: None,
                        };
                        connection.sender.send(Message::Response(resp))?;
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
                    Err(ExtractError::MethodMismatch(req)) => req,
                };
                match cast::<lsp_types::request::HoverRequest>(req.clone()) {
                    Ok((id, params)) => {
                        eprintln!("got hover request #{id}: {params:?}");
                        let result = Some("Hello, world!".to_string());
                        let result = serde_json::to_value(&result).unwrap();
                        let resp = Response {
                            id,
                            result: Some(result),
                            error: None,
                        };
                        connection.sender.send(Message::Response(resp))?;
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
                    Err(ExtractError::MethodMismatch(req)) => req,
                };
            }
            Message::Response(resp) => {
                eprintln!("got response: {resp:?}");
            }
            Message::Notification(not) => {
                eprintln!("got notification: {not:?}");
            }
        }
    }
    Ok(())
}

fn cast<R>(req: Request) -> Result<(RequestId, R::Params), ExtractError<Request>>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    req.extract(R::METHOD)
}
