use std::{
    ops::Fn,
    collections::HashMap,
    error::Error,
    rc::Rc,
    sync::{Arc, Mutex},
};

use super::params::{
    CliParameters,
    ParamValue,
};

use super::clitc_error::{
    CommandModeError,
    NoEventError,
    UnknownCommandError,
};

type ParamResult = HashMap<String, ParamValue>;
type CallbackFn = dyn Fn(ParamResult) -> ();
type InfoFn = dyn Fn(ParamResult, HashMap<String, Vec<String>>) -> ();
type EmitHandle = Arc<Mutex<Option<String>>>;
type EmitFn = dyn Fn(EmitHandle) -> ();

pub enum Event {
    Callback(Rc<CallbackFn>),
    InfoCallback(Rc<InfoFn>),
    Emit(EmitHandle, Rc<EmitFn>),
}

impl Clone for Event {
    fn clone(&self) -> Event {
        match self {
            Event::Callback(f) => Event::Callback(Rc::clone(&f)),
            Event::InfoCallback(f) => Event::InfoCallback(Rc::clone(&f)),
            Event::Emit(h, f) => Event::Emit(Arc::clone(&h), Rc::clone(&f)),
        }
    }
}

pub trait Split {
    fn split(&self, s: String) -> Vec<String>;
}

pub struct WhitespaceSplitter;

impl Split for WhitespaceSplitter {
    fn split(&self, s: String) -> Vec<String> {
        s.split_whitespace().map(|x| x.to_string()).collect()
    }
}

pub struct EventHandler<S>
    where S: Split
{
    cli_params: CliParameters,
    events: HashMap<String, Event>,
    split_fn: S,
    single_cmd: bool,
}

impl<S: Split> EventHandler<S> {
    pub fn new(cli_params: CliParameters, split_fn: S, single_cmd: bool) -> EventHandler<S> {
        let mut event_handler = EventHandler {
            cli_params,
            events: HashMap::new(),
            split_fn,
            single_cmd,
        };
        event_handler.cli_params.set_sequential_processing(true);
        return event_handler;
    }

    fn get_info(&self) -> HashMap<String, Vec<String>> {
        let mut text = HashMap::new();
        for param in self.cli_params.iter() {
            text.insert(param.name.clone(), param.info());
        }
        return text;
    }

    pub fn attach(&mut self, events: HashMap<String, Event>) {
        self.events = events;
    }

    pub fn disattach(&mut self) -> HashMap<String, Event> {
        let ret = self.events.clone();
        self.events = HashMap::new();
        return ret;
    }

    fn invoke_event(&self, key: String, args: HashMap<String, ParamValue>) -> Result<(), NoEventError> {
        if let Some(evt) = self.events.get(&key) {
            // Callback function called if connected event can be found
            match evt {
                Event::Callback(callback) => callback(args),
                // Return with  entire cmd info if requested (help cmds)
                Event::InfoCallback(callback) => callback(args, self.get_info()),
                // Return emit handle
                Event::Emit(handle, callback) => callback(Arc::clone(&handle)),
            };
        } else {
            // No Events with this identifier found
            return Err(NoEventError);
        }
        return Ok(());
    }

    pub fn pass_command(&self, data: String) -> Result<(), Box<dyn Error>> {
        let res = self.cli_params.parse_str(&data[..], |x| self.split_fn.split(x.to_string()));
        // Check if there were any known commands found
        if res.is_empty() {
            // Could not identify any known command
            return Err(Box::new(UnknownCommandError));
        }
        // Check if command count and single command mode don't collide
        if res.len() > 1 && self.single_cmd {
            // if multiple commands were entered, single command mode was infringed
            return Err(Box::new(CommandModeError));
        }
        // Find connected events for parsed commands
        for (cmd, args) in res.into_iter() {
            // Abort if invoking throws Error
            self.invoke_event(cmd, args)?;
        }
        return Ok(());
    }
}
