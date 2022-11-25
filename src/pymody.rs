use rustpython_pylib as pylib;
use rustpython_stdlib as stdlib;
use rustpython_vm as vm;
use std::process::ExitCode;

fn main() -> std::process::ExitCode {
    let mut settings: vm::Settings = Default::default();
    settings.path_list.extend(get_paths("RUSTPYTHONPATH"));
    settings.path_list.extend(get_paths("PYTHONPATH"));
    // settings.path_list.push(pylib::LIB_PATH.to_owned());

    let interp = vm::Interpreter::with_init(settings, |vm| {
        vm.add_native_modules(stdlib::get_module_inits());
        vm.add_frozen(pylib::frozen_stdlib());
    });

    let result = py_main(&interp);
    println!("RUST, py_main___{result:?}___");
    let result = py_main(&interp);
    println!("RUST, py_main___{result:?}___");
    let result = result.map(|ret| {
        println!("RUST, name: {:?}", ret);
    });
    ExitCode::from(interp.run(|_vm| result))
}

const SCRIPT_PATH: &str = "src/scripts/pymodx.py";
const SRC_1: &str = r#"
import pymodx
count=0
ctx = embed_import.context()
count += 1
print(f"SOURCE#{count}", __name__, ctx)
"#;

fn py_main(interp: &vm::Interpreter) -> vm::PyResult<vm::PyObjectRef> {
    interp.enter(|vm| {
        vm.insert_sys_path(vm.new_pyobj("src/scripts"))
            .expect("add path");

        let _result = {
            {
                let scope = vm.new_scope_with_builtins();
                let value = vm.ctx.new_str(format!("{}", "世界")).into();
                scope.globals.set_item("_hi___", value, vm).unwrap();
                vm.run_script(scope, SCRIPT_PATH)?;
                // let src = "print('foo-bar')";
                // vm.run_code_string(vm.new_scope_with_builtins(), src, "<...>".to_owned())?;
            }
            let pycode = vm
                .compile(SRC_1, vm::compiler::Mode::Exec, "<embedded>".to_owned())
                .map_err(|err| vm.new_syntax_error(&err))?;
            vm.run_code_obj(pycode, vm.new_scope_with_builtins())?
        };
        let result = {
            let module = vm.import("pymodx", None, 0)?;
            let name_func = module.get_attr("context", vm)?;
            let result = vm.invoke(&name_func, ())?;
            result.get_attr("name", vm)?
            //; let result:: vm::builtins::PyStrRef = result.try_into_value(vm)?;
        };
        return vm::PyResult::Ok(result);
    })
}

fn get_paths(env_variable_name: &str) -> impl Iterator<Item = String> + '_ {
    std::env::var_os(env_variable_name)
        .into_iter()
        .flat_map(move |paths| {
            std::env::split_paths(&paths)
                .map(|path| {
                    path.into_os_string()
                        .into_string()
                        .unwrap_or_else(|_| panic!("{} isn't valid unicode", env_variable_name))
                })
                .collect::<Vec<_>>()
        })
}
