use std::fs::read_to_string;
use pyo3::{prepare_freethreaded_python, Python};

fn main() {
    let python_file = read_to_string("python_program.py").unwrap();

    // Setup the Python Environment
    prepare_freethreaded_python();

    Python::with_gil(|py| {
        // Execute the program to set the values in the Python runtime
        py.run(&python_file, None, None).unwrap();

        // Extract the values
        let my_config_entry = py.eval("myConfigEntry", None, None)
            .unwrap()
            .extract::<i32>()
            .unwrap();
        let my_config_string = py.eval("myConfigString", None, None)
            .unwrap()
            .extract::<String>()
            .unwrap();

        println!("myConfigEntry: {}", my_config_entry);
        println!("myConfigString: {}", my_config_string);

        // Run the DoIt function
        let do_it = py.eval("DoSomething()", None, None).unwrap();
        println!("DoIt: {:?}", do_it);
    });
}
