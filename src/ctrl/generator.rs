use std::string::String;
use std::env::current_dir;
use std::path::PathBuf;
use std::path::Path;
use std::fmt::{self};
use std::io::prelude::*;
use std::fs::File;
use std::fs::create_dir_all;
use ansi_term::Colour::*;
use ctrl::input_handler::{verify_dir};

static COMPONENT_TPL: &'static str = include_str!("../../res/component/component.tpl");
static COMPONENT_TEST_TPL: &'static str = include_str!("../../res/component/spec.tpl");
static CONTAINER_TPL: &'static str = include_str!("../../res/container/container.tpl");
static HANDLER_ACTION_TPL: &'static str = include_str!("../../res/handler/actions.tpl");
static HANDLER_CONTAINER_TPL: &'static str = include_str!("../../res/handler/container.tpl");
static HANDLER_REDUCER_TPL: &'static str = include_str!("../../res/handler/reducer.tpl");

#[derive(Debug)]
enum ComponentType {
    COMPONENT,
    CONTAINER,
    HANDLER
}

impl fmt::Display for ComponentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Generator {
    pub name: String,
    pub verbose: bool,
    pub current_dir: String,
    pub extension: String,
    component_type: ComponentType
}

impl Generator {
    pub fn new(command: String, verbose: bool, dir: String, extension: String) -> Generator {
        let current_dir_buf: PathBuf = match current_dir() {
            Ok(path) => path,
            Err(e) => {
                println!("{}", e);
                PathBuf::new()
            }
        };
        let mut current_dir = match current_dir_buf.into_os_string().into_string() {
            Ok(dir) => dir,
            Err(e) => panic!("Could not find the current working directory, {:?}", e)
        };
        if !dir.eq("") {
            current_dir = dir;
        }
        Generator { name: command, verbose: verbose, component_type: ComponentType::COMPONENT, current_dir: current_dir, extension: extension }
    }

    pub fn run(&mut self, component_type: String) {
        if component_type.eq("component") {
            self.component_type = ComponentType::COMPONENT;
        }

        if component_type.eq("container") {
            self.component_type = ComponentType::CONTAINER;
        }

        if component_type.eq("handler") {
            self.component_type = ComponentType::HANDLER;
        }

        if self.verbose {
            println!("Generating {} of type {}", Red.bold().paint(self.name.clone()),
                     Red.paint(self.component_type.to_string()));
        }

        self.generate_component();
    }

    fn generate_component(&self) {
        match self.component_type {
            ComponentType::COMPONENT => self.function_component(),
            ComponentType::HANDLER => self.route_component(),
            ComponentType::CONTAINER => self.class_component()
        };
    }

    fn function_component(&self) {
        if self.verbose {
            println!("COMPONENT {:?}", self.name);
        }

        let mut tpl: String = String::from(COMPONENT_TPL.clone());
        tpl = tpl.replace("${class_name}", self.name.as_str());

        let mut tpl_test: String = String::from(COMPONENT_TEST_TPL.clone());
        tpl_test = tpl_test.replace("${name}", self.name.as_str());
        tpl_test = tpl_test.replace("${name_lower}", self.name.to_lowercase().as_str());

        if self.verbose {
            println!("\n{}\n", Green.paint(tpl.clone()));
        }

        let file_name = self.name.clone().to_lowercase() + "." + &self.extension.clone();
        let components_dir = Path::new(&self.current_dir).join("client/components");
        let dir = verify_dir("Create component in ".to_string(), components_dir.into_os_string().into_string().unwrap());
        write_file(Path::new(&dir), file_name, tpl, true);

        let test_file_name = self.name.clone().to_lowercase() + ".spec." + &self.extension.clone();
        let test_dir = Path::new(&self.current_dir).join("test/client/components");
        let dir = verify_dir("Create component test in ".to_string(), test_dir.into_os_string().into_string().unwrap());
        write_file(Path::new(&dir), test_file_name, tpl_test, true);
    }

    fn class_component(&self) {
        if self.verbose {
            println!("CONTAINER {:?}", self.name);
        }

        let mut tpl: String = String::from(CONTAINER_TPL.clone());
        tpl = tpl.replace("${class_name}", self.name.as_str());

        if self.verbose {
            println!("\n{}\n", Green.paint(tpl.clone()));
        }

        let file_name = self.name.clone() + "." + &self.extension.clone();
        let dir = verify_dir("Create container in ".to_string(), self.current_dir.clone());
        write_file(Path::new(&dir), file_name, tpl, true);
    }

    fn route_component(&self) {
        if self.verbose {
            println!("HANDLER {:?}", self.name);
        }

        let mut tpl_actions: String = String::from(HANDLER_ACTION_TPL.clone());
        let mut tpl_container: String = String::from(HANDLER_CONTAINER_TPL.clone());
        let mut tpl_reducer: String = String::from(HANDLER_REDUCER_TPL.clone());

        let name: String = self.name.clone();
        let action_var_name: String = self.name.clone().to_uppercase() + "_SUBMIT";
        let lower_name: String = self.name.clone().to_lowercase();

        tpl_actions = tpl_actions.replace("${action_var_name}", action_var_name.as_str());
        tpl_actions = tpl_actions.replace("${action_name}", action_var_name.as_str());

        tpl_container = tpl_container.replace("${name}", name.as_str());

        tpl_reducer = tpl_reducer.replace("${action_var_name}", action_var_name.as_str());
        tpl_reducer = tpl_reducer.replace("${action_name}", action_var_name.as_str());
        tpl_reducer = tpl_reducer.replace("${lower_name}", lower_name.as_str());

        let actions_file_name = self.name.clone() + "Actions" + "." + &self.extension.clone();
        let actions_dir = verify_dir("Create actions in ".to_string(), self.current_dir.clone());
        write_file(Path::new(&actions_dir), actions_file_name, tpl_actions, true);

        let container_file_name = self.name.clone() + "Container" + "." + &self.extension.clone();
        let container_dir = verify_dir("Create Redux container in ".to_string(), self.current_dir.clone());
        write_file(Path::new(&container_dir), container_file_name, tpl_container, true);

        let reducer_file_name = self.name.clone() + "Reducer" + "." + &self.extension.clone();
        let reducer_dir = verify_dir("Create reducer in ".to_string(), self.current_dir.clone());
        write_file(Path::new(&reducer_dir), reducer_file_name, tpl_reducer, true);
    }
}

fn write_file(dir: &Path, file_name: String, content: String, show_confirmation: bool) {
    match create_dir_all(dir) {
        Ok(ok) => ok,
        Err(e) => panic!("Could not create directories in path {}\n {}", dir.to_str().unwrap(), e)
    };

    let file_path: PathBuf = Path::new(&dir).join(Path::new(&file_name));
    let mut buffer = match File::create(&file_path) {
        Ok(buf) => buf,
        Err(e) => panic!("Error creating file at path {}\n {}", file_path.into_os_string().into_string().unwrap(), e)
    };

    match buffer.write_all(&content.as_bytes()) {
        Ok(res) => res,
        Err(e) => panic!("Error writing to file at path {}\n {}", file_path.into_os_string().into_string().unwrap(), e)
    };

    if show_confirmation {
        println!("File {} written with {} bytes", file_path.into_os_string().into_string().unwrap(), content.len());
    }
}