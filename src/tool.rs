use crate::prelude::*;

use rhai::{ Engine, Scope, AST, Dynamic, Map };

#[derive(Clone)]
pub struct Tool {
    pub script                  : String,
    pub ast                     : Option<AST>,
    pub this_map                : Dynamic,
    pub widget_values           : Vec<WidgetValue>
}

impl Tool {
    pub fn new(script: String) -> Self {

        let this_map = rhai::Map::new();

        Self {
            script,
            ast                 : None,
            this_map            : this_map.into(),
            widget_values       : vec![],
        }
    }

    pub fn init(&mut self, engine: &Engine, file_name: &str) {

        let this_map = rhai::Map::new();

        self.this_map = this_map.into();

        let result = engine.compile(self.script.as_str());

        if result.is_ok() {
            if let Some(ast) = result.ok() {
                let result = engine.eval_ast::<Dynamic>(&ast);

                if result.is_err() {
                    println!("{} : {:?}", file_name, result.err().unwrap().to_string());

                    /*
                    if let Some(err) = result.err() {
                        let mut string = err.to_string();
                        let mut parts = string.split("(");
                        if let Some(first) = parts.next() {
                            string = first.to_owned();
                        }
                        return Some((string, err.position().line()));
                    }*/
                } else {
                    #[allow(deprecated)]
                    let result = engine.call_fn_raw(
                                    &mut Scope::new(),
                                    &ast,
                                    false,
                                    true,
                                    "init",
                                    Some(&mut self.this_map),
                                    []
                                );

                    if result.is_err() {
                        println!("{} : {:?}", file_name, result.err().unwrap().to_string());
                    }

                    let map = self.this_map.clone().cast::<rhai::Map>();

                    let mut values : Vec<WidgetValue> = vec![];

                    if let Some(v) = map.get("parameters") {
                        if let Some(array) = v.clone().into_array().ok() {
                            for v in array {
                                if let Some(value) = v.read_lock::<ScriptValue>() {
                                    match value.value_type {
                                        ScriptValueType::Color => {
                                            values.push(WidgetValue::Color(value.name.clone(), value.index));
                                        }
                                    }
                                }
                            }
                        }
                    }

                    //println!("{:?}", values);
                    self.widget_values = values;
                }

                self.ast = Some(ast);
            }
        } else {
            println!("{} : {:?}", file_name, result.err().unwrap().to_string());
        }
    }

    /// Returns the name of the tool
    pub fn name(&self) -> String  {
        if let Some(name) = self.get_string("name") {
            name
        } else {
            "".into()
        }
    }

    /// Apply the tool
    pub fn apply(&mut self, engine: &Engine, key: Vec3i) {
        println!("apply");

        if let Some(ast) = &self.ast {

            let location : Dynamic = Dynamic::from(ScriptVec3i::from_vec3i(key));

            #[allow(deprecated)]
            let result = engine.call_fn_raw(
                            &mut Scope::new(),
                            &ast,
                            false,
                            true,
                            "apply",
                            Some(&mut self.this_map),
                            [location]//[(pos.0 as i32).into(), (pos.1 as i32).into()]
                        );

            println!("{:?}", result);
        }
    }

    /// Apply the tool
    pub fn hit(&mut self, engine: &Engine, hit_record: HitRecord) {

        if let Some(ast) = &self.ast {

            let hit : Dynamic = Dynamic::from(hit_record);

            #[allow(deprecated)]
            let result = engine.call_fn_raw(
                            &mut Scope::new(),
                            &ast,
                            false,
                            true,
                            "hit",
                            Some(&mut self.this_map),
                            [(hit),]
                        );

            println!("{:?}", result);
        }
    }

    /// Return a string from the map
    pub fn get_string(&self, key: &str) -> Option<String> {
        if let Some(map) = self.this_map.read_lock::<Map>() {
            if let Some(value) = map.get(key) {
                return Some(value.to_string());
            }
        }
        None
    }

}