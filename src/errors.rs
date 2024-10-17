use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShellpageError {
    #[error("I/O error")]
    Io(#[from] io::Error),

    #[error("Configurantion file parsing error")]
    TomlConfigurantion(#[from] toml::de::Error),

    #[error("required arg `{0}` was not provided")]
    RequiredArg(String),
    #[error("required field in config file `{0}` is required")]
    RequiredConfigField(String),

    #[error("file `{0}` was not found")]
    FileNotFound(String),
    #[error("could not write to file `{0}`")]
    UnableToWrite(String),
    
    #[error("could not configure the template renderer using `{0}`")]
    RenderConfigurationError(String),

    #[error("could not render template `{0}`: {1}")]
    RenderExecutionError(String, String),
}
