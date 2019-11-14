extern crate serde_json;

use std::{
    cmp::{Ord},
    error::Error,
    collections::{HashMap, HashSet},
};
use serde_json::{Value};
use super::clitc_error::{
    MissingInformationError,
    WrongFormatError,
};

#[derive(Clone)]
pub enum ParamValue {
    Array(Vec<String>),
    Int(i64),
    String(String),
    Num(f64),
}

impl std::string::ToString for ParamValue {
    fn to_string(&self) -> String {
        match self {
            ParamValue::Array(args) => args.join(", "),
            ParamValue::Int(val) => val.to_string(),
            ParamValue::String(s) => s.clone(),
            ParamValue::Num(num) => num.to_string(),
        }
    }
}

#[derive(Clone)]
enum SubParamType {
    Array,
    Int,
    String,
    Num,
}

impl SubParamType {
    pub fn info(&self) -> &str {
        match self {
            SubParamType::Array => "array",
            SubParamType::Int => "int",
            SubParamType::String => "string",
            SubParamType::Num => "num",
        }
    }
}

impl SubParamType {
    pub fn from(s: &str) -> Result<SubParamType, Box<Error>> {
        if s == "array" {
            return Ok(SubParamType::Array);
        } else if s == "int" {
            return Ok(SubParamType::Int);
        } else if s == "string" {
            return Ok(SubParamType::String);
        } else if s == "num" {
            return Ok(SubParamType::Num);
        } else {
            println!("Wrong subparam type!");
            return Err(Box::new(WrongFormatError));
        }
    }
}

#[derive(Clone)]
pub struct SubParam {
    ord: u8,
    name: Option<String>,
    ptype: SubParamType,
}

impl SubParam {
    pub fn from(val: Value) -> Result<SubParam, Box<Error>> {
        let ord = val["ord"].as_u64().map(|x| x as u8).ok_or(Box::new(MissingInformationError))?;
        let name = val["name"].as_str().map(|x| x.to_string());
        let ptype = SubParamType::from(val["type"].as_str().ok_or(Box::new(MissingInformationError))?)?;

        return Ok(SubParam{ord, name, ptype});
    }

    pub fn get_name(&self) -> String {
        let ret;
        if let Some(name) = self.name.clone() {
            ret = name;
        } else {
            ret = self.ord.to_string();
        }
        return ret;
    }

    pub fn match_with(&self, args: &mut Vec<String>) -> Option<(String, ParamValue)> {
        // Check associated Value
        let ret = match &self.ptype {
            SubParamType::Array => ParamValue::Array(args.clone()),
            SubParamType::Int => {
                let val = args.remove(0).parse::<i64>();
                if val.is_err() {
                    return None;
                }
                ParamValue::Int(val.unwrap())
            },
            SubParamType::Num => {
                let val = args.remove(0).parse::<f64>();
                if val.is_err() {
                    return None;
                }
                ParamValue::Num(val.unwrap())
            },
            SubParamType::String => ParamValue::String(args.remove(0)),
        };

        return Some((self.get_name(), ret));
    }
}

#[derive(Clone)]
pub struct Param {
    short: Option<String>,
    pub name: String,
    descr: Option<String>,
    params: Vec<SubParam>,
}

impl Param {
    pub fn from(val: Value) -> Result<Param, Box<Error>> {
        let short = val["short"].as_str().map(|x| x.to_string());
        let name = val["name"].as_str().map(|x| x.to_string()).ok_or(Box::new(MissingInformationError))?;
        let descr = val["descr"].as_str().map(|x| x.to_string());
        let raw_params = val["params"].as_array();

        let mut ord_set = HashSet::new();
        let mut params = Vec::new();
        if raw_params.is_some() {
            for v in raw_params.unwrap() {
                let subparam = SubParam::from(v.clone())?;
                let ord_cp = subparam.ord.clone();
                if let Some(_) = ord_set.get(&ord_cp) {
                    // ord not unique!
                    println!("Ord not unique!");
                    return Err(Box::new(WrongFormatError));
                } else {
                    ord_set.insert(ord_cp);
                }
                params.push(subparam);
            }
            params.sort_by(|a, b| a.ord.cmp(&b.ord));
        }

        return Ok(Param{short, name, descr, params});
    }

    pub fn match_with(&self, args: &mut Vec<String>) -> (HashMap<String, ParamValue>) {
        let mut ret = HashMap::new();
        for param in self.params.iter() {
            if args.len() > 0 {
                let res = param.match_with(args);
                if let Some((key, val)) = res {
                    ret.insert(key, val);
                }
            }
        }
        return ret;
    }

    pub fn info(&self) -> Vec<String> {
        let mut info: Vec<String> = Vec::new();
        let mut name = self.name.clone();
        if let Some(short) = self.short.clone() {
            name.push_str(&format!("/ {}", short)[..]);
        }
        info.push(format!("\t{}\t{}", name, self.descr.clone().unwrap_or(String::new())));
        for subparam in self.params.iter() {
            let name = match subparam.name.clone() {
                Some(val) => val,
                None => subparam.ord.to_string(),
            };
            info.push(format!("\t\t{}:\t{}", name,subparam.ptype.info()));
        }
        return info;
    }
}

pub struct CliParameters(Vec<Param>, bool);

impl CliParameters {
    fn from(val: Value) -> Result<CliParameters, Box<Error>> {
        if let Some(vec) = val["options"].as_array() {
            let mut params = Vec::new();
            for val in vec.iter() {
                let param = Param::from(val.clone())?;
                params.push(param);
            }
            return Ok(CliParameters(params, false));
        }
        println!("options root not found!");
        return Err(Box::new(WrongFormatError));
    }

    pub fn from_str<'a>(data: &'a str) -> Result<CliParameters, Box<Error>> {
        let val: Value = serde_json::from_str(data)?;
        CliParameters::from(val)
    }

    pub fn from_slice<'a>(data: &'a [u8]) -> Result<CliParameters, Box<Error>> {
        let val: Value = serde_json::from_slice(data)?;
        CliParameters::from(val)
    }

    pub fn from_reader<R>(reader: R) -> Result<CliParameters, Box<Error>>
        where R: std::io::Read,
    {
        let val: Value = serde_json::from_reader(reader)?;
        CliParameters::from(val)
    }

    pub fn set_sequential_processing(&mut self, seq: bool) {
        self.1 = seq;
    }

    pub fn iter(&self) -> std::slice::Iter<Param> {
        return self.0.iter();
    }

    fn get_named_locations(&self, args: &Vec<String>) -> Vec<(usize, &Param)> {
        let mut locations: Vec<(usize, &Param)> = Vec::new();
        for (i, arg) in (0..args.len()).zip(args.iter()) {
            let res = self.0.iter().find(|x| {
                let mut ret: bool = false;
                if x.short.is_some() {
                    ret = x.short.clone().unwrap() == *arg
                }
                if !ret {
                    ret = x.name.clone() == *arg
                }
                ret
            });
            // Param matched to argument:
            if let Some(matching) = res {
                locations.push((i, matching));
            }
        }
        // Sort locations by index
        locations.sort_by(|a, b| a.0.cmp(&b.0));
        return locations;
    }

    fn process_locations(&self, locations: Vec<(usize, &Param)>, args: &mut Vec<String>, ret: &mut HashMap<String, HashMap<String, ParamValue>>) {
        for i in 0..locations.len() {
            let (index, param) = locations[i];
            let mut interval;
            if (i + 1) < locations.len() {
                let next_index = locations[i + 1].0;
                interval = next_index - index;
            } else {
                interval = args.len();
            }
            // Leave out named identifier argument
            args.remove(0);
            interval -= 1;
            // Parse Subparams
            let res = param.match_with(&mut args.drain(..interval).collect());
            ret.insert(param.name.clone(), res);
        }
    }

    fn process_sequentially(&self, args: &mut Vec<String>, ret: &mut HashMap<String, HashMap<String, ParamValue>>) {
        while args.len() > 0 {
            // Iterate through arguments to find match with params
            let res = (0..args.len()).zip(args.iter()).find_map(|(i, x)| {
                let mut param = None;
                // Compare to params
                for p in self.0.iter() {
                    let mut matched = false;
                    if p.short.is_some() {
                        matched = p.short.clone().unwrap() == **x;
                    }
                    if !matched {
                        matched = p.name == **x;
                    }
                    if matched {
                        // Return current argument index and matched param
                        param = Some((i, p));
                        break;
                    }
                }
                return param;
            });
            // Check if match was found
            if let Some((index, matching)) = res {
                // Cut off arguments to current location
                let boundary = index + 1;
                args.drain(..boundary);
                // Parse Subparams
                let result = matching.match_with(args);
                ret.insert(matching.name.clone(), result);

            } else {
                // Abort if no more can be found
                break;
            }
        }
    }

    pub fn parse_vec(&self, mut args: Vec<String>) -> HashMap<String, HashMap<String, ParamValue>> {
        let mut ret: HashMap<String, HashMap<String, ParamValue>> = HashMap::new();

        if self.1 { // checking set sequentiality member variable
            self.process_sequentially(&mut args, &mut ret);
        } else {
            // Get param locations
            let locations = self.get_named_locations(&args);

            // Process named parameters
            self.process_locations(locations, &mut args, &mut ret);
        }

        return ret;
    }

    pub fn parse_args(&self) -> HashMap<String, HashMap<String, ParamValue>> {

        let mut args: Vec<String> = std::env::args().collect();
        args.remove(0); // remove program path cl argument

        return self.parse_vec(args);
    }

    pub fn parse_str<'a, F>(&self, data: &'a str, split: F) -> HashMap<String, HashMap<String, ParamValue>>
        where F: Fn(&'a str) -> (Vec<String>)
    {
        let args: Vec<String> = split(data);
        self.parse_vec(args)
    }

    pub fn parse_str_whitespace<'a>(&self, data: &'a str) -> HashMap<String, HashMap<String, ParamValue>> {
        self.parse_str(data, |args| args.split_whitespace().map(|x| x.to_string()).collect())
    }
}
