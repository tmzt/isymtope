
#![recursion_limit="1000"]
#![feature(plugin)]
#![plugin(indoc)]
#![feature(box_patterns)]

extern crate wren;
extern crate uuid;
extern crate itertools;
extern crate linked_hash_map;
extern crate broadcast;

mod parser;
mod scope;
// mod processing;
// mod output;

use wren::{VM, Configuration};

pub fn main() {
    let source = r#"
        class Unicorn {
            static hasHorn() {
                return true
            }
        }
    "#;

    let mut vm = VM::new(Configuration::new());

    vm.interpret(source);

    vm.get_variable("main", "Unicorn", 0);
    let class_handle = vm.get_slot_handle(0);

    let has_horn = vm.make_call_handle("hasHorn()");
    vm.set_slot_handle(0, &class_handle);
    vm.call(&has_horn);

    let ty = vm.get_slot_type(0);
    println!("Type {:?}", ty);

    let horn_result = vm.get_slot_bool(0);
    println!("Has horn? {:?}", horn_result);

    // let greets = vm.get_slot_handle(0).unwrap();
    // println!("Greets {:?}", greets);

    // let vm = VM::new(Configuration::new());
    // match vm.interpret("Test", source) {
    // Err(Error::CompileError(msg)) => println("Compile Error: {}", msg),
    // Err(Error::RuntimeError(msg)) => println("Compile Error: {}", msg),
    // Err(Error::UnknownError(msg)) => println("Compile Error: {}", msg),
    // _ => println!("Successfully ran")
    // }
    //

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
