use crate::ast::{Ast, ProvidedServiceImplementation};
use std::fs::{self, create_dir_all};
use std::path::PathBuf;

//deal with specializes
//no analog to generic in py

const GENERATED_PYTHON_EXAMPLES_DIR: &str = "examples/python";

pub struct GenPython {
    ast: Ast,
    path: Vec<String>,
    imports: Vec<ImportList>,
    component_name: String,
    required_services: Vec<RequiredService>,
    provided_services: Vec<ProvidedService>,
    part_instances: Vec<PartInstance>,
}

struct ImportList {
    path: Vec<String>,
}

struct RequiredService {
    name: String,
    type_name: String,
}

struct ProvidedService {
    name: String,
    type_name: String,
    implementation: ProvidedServiceImplementation,
}

struct PartInstance {
    name: String,
    type_name: String,
    target: Vec<String>,
}

impl GenPython {
    pub fn new(ast: Ast) -> Self {
        Self {
            ast,
            path: Vec::new(),
            imports: Vec::new(),
            component_name: String::new(),
            required_services: Vec::new(),
            provided_services: Vec::new(),
            part_instances: Vec::new(),
        }
    }

    pub fn generate(&mut self) {
        match self.ast.clone() {
            Ast::SEQ(v) => {
                self.namespace(&v, 0);
            }
            _ => {}
        }
    }

    fn namespace(&mut self, v: &Vec<Ast>, i: usize) {
        match v[i].clone() {
            Ast::Import { path } => {
                self.imports.push(ImportList { path });
                self.namespace(v, i + 1);
            }
            Ast::Namespace { path, body } => {
                self.path = path;
                self.component(body);
            }
            _ => {}
        }
    }

    fn component(&mut self, b: Box<Ast>) {
        match *b {
            Ast::Component { name, body, .. } => {
                self.component_name = name;
                self.service(body);
            }
            _ => {}
        }
    }

    fn service(&mut self, b: Box<Ast>) {
        match *b {
            Ast::SEQ(v) => {
                self.vec_service(&v, 0);
            }
            _ => {}
        }
    }

    fn vec_service(&mut self, v: &Vec<Ast>, i: usize) {
        if i < v.len() {
            match v[i].clone() {
                Ast::Requires { name, type_name } => {
                    self.required_services
                        .push(RequiredService { name, type_name });
                    self.vec_service(v, i + 1);
                }
                Ast::Provides {
                    name,
                    type_name,
                    implementation,
                } => {
                    self.provided_services.push(ProvidedService {
                        name,
                        type_name,
                        implementation,
                    });
                    self.vec_service(v, i + 1);
                }
                Ast::Part {
                    name,
                    type_name,
                    body,
                    ..
                } => {
                    match *body {
                        Ast::SEQ(v) => {
                            if v.len() != 0 {
                                match v[0].clone() {
                                    Ast::Bind { name, target } => {
                                        self.part_instances.push(PartInstance {
                                            name,
                                            type_name,
                                            target,
                                        });
                                    }
                                    _ => {
                                        self.part_instances.push(PartInstance {
                                            name,
                                            type_name,
                                            target: Vec::new(),
                                        });
                                    }
                                }
                            } else {
                                self.part_instances.push(PartInstance {
                                    name,
                                    type_name,
                                    target: Vec::new(),
                                });
                            }
                        }
                        _ => {}
                    }
                    self.vec_service(v, i + 1);
                }
                _ => {}
            }
        } else {
            self.write_file();
        }
    }

    fn write_file(&mut self) {
        //Create folder
        let mut f_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        f_path.push("..");
        f_path.push(GENERATED_PYTHON_EXAMPLES_DIR);

        let mut i = 0;
        while i < self.path.len() {
            f_path.push(&self.path[i]);
            i += 1;
        }

        create_dir_all(&f_path).expect("failed to create Python output directory");

        //Create file path
        f_path.push(format!("{}.py", self.component_name));

        let mut wr = String::new();
        //Add imports
        i = 0;
        while i < self.imports.len() {
            let import = &self.imports[i];

            wr.push_str("from ");
            wr += &import.path[0];
            let mut j = 1;
            while j < import.path.len() {
                wr.push('.');
                wr += &import.path[j];
                j += 1;
            }
            wr.push_str(" import *");
            wr.push('\n');
            i += 1;
        }

        //Create class
        wr.push_str("\nclass ");
        wr += &self.component_name;
        wr.push_str(" :\n");

        //Create init
        wr.push_str("\tdef __init__(self");
        let mut body = String::new();

        i = 0;
        while i < self.required_services.len() {
            let required_service = &self.required_services[i];

            wr.push_str(", ");
            wr += &required_service.name;
            wr.push_str(" : ");
            wr += &required_service.type_name;

            body.push_str("\t\tself.");
            body += &required_service.name;
            body.push_str(" = ");
            body += &required_service.name;
            body.push('\n');

            i += 1
        }

        i = 0;
        while i < self.part_instances.len() {
            let part_instance = &self.part_instances[i];

            body.push_str("\t\tself.");
            body += &part_instance.name;
            body.push_str(" = ");
            body += &part_instance.type_name;
            body.push('(');

            let mut j = 0;
            let target = &part_instance.target;

            if target.len() != 0 {
                body.push_str("self");
            }

            while j < target.len() {
                body.push('.');
                body += &target[j];

                j += 1;
            }

            body.push_str(")\n");

            i += 1;
        }

        wr.push_str("):\n");

        body.push_str("\t\treturn\n");
        wr += &body;

        //Add provided methods
        i = 0;
        while i < self.provided_services.len() {
            let provided_service = &self.provided_services[i];

            wr.push_str("\n\tdef ");
            wr += &provided_service.name;
            wr.push_str("(self) -> ");
            wr += &provided_service.type_name;
            wr.push_str(":\n\t\treturn");

            match &provided_service.implementation {
                ProvidedServiceImplementation::Local => {}
                ProvidedServiceImplementation::Delegated(service_reference) => {
                    wr.push_str(" self");
                    wr.push('.');
                    wr += &service_reference.part_name;
                    wr.push('.');
                    wr += &service_reference.service_name;
                    wr.push_str("()\n");
                }
            }

            i += 1;
        }

        //Create and fill file
        fs::write(&f_path, wr).expect("failed to write Python output file");
    }
}
