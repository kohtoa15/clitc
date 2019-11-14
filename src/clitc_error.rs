use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Clone, Debug)]
pub struct MissingInformationError;

impl Error for MissingInformationError {
    fn description(&self) -> &str {
        "Necessary information for Param missing!"
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl Display for MissingInformationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Necessary information for Param missing!")
    }
}

#[derive(Clone, Debug)]
pub struct WrongFormatError;

impl Error for WrongFormatError {
    fn description(&self) -> &str {
        "Wrong format for params!"
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl Display for WrongFormatError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Wrong format for params!")
    }
}

#[derive(Clone, Debug)]
pub struct CommandModeError;

impl Error for CommandModeError {
    fn description(&self) -> &str {
        "Multiple commands entered, but single command mode configured!"
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl Display for CommandModeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Multiple commands entered, but single command mode configured!")
    }
}

#[derive(Clone, Debug)]
pub struct NoEventError;

impl Error for NoEventError {
    fn description(&self) -> &str {
        "No event attached to this command!"
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl Display for NoEventError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Multiple commands entered, but single command mode configured!")
    }
}

#[derive(Clone, Debug)]
pub struct UnknownCommandError;

impl Error for UnknownCommandError
 {
    fn description(&self) -> &str {
        "Could not find a known command in the statement"
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl Display for UnknownCommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Could not find a known command in the statement")
    }
}
