use rand::{distributions::Alphanumeric, Rng};
use wasmtime::*;
use chrono;
use md5;

fn generate_random_wat() -> String {
    let random_comment: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    format!(
        r#"(module
            ;; {}
            (func (export "run")
                call $print_hello)
            (func $print_hello
                (call $print_h)
                (call $print_e)
                (call $print_l)
                (call $print_l)
                (call $print_o))
            (func $print_h (drop (i32.const 72)))
            (func $print_e (drop (i32.const 69)))
            (func $print_l (drop (i32.const 76)))
            (func $print_o (drop (i32.const 79)))
        )"#,
        random_comment
    )
}

fn main() -> anyhow::Result<()> {
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let wat = generate_random_wat();
    let mutated_code = wat.clone(); // <<<<<<<<<< WE NEEDED THIS LINE

    let module = Module::new(&engine, wat)?;
    let instance = Instance::new(&mut store, &module, &[])?;
    let run = instance.get_typed_func::<(), ()>(&mut store, "run")?;
    run.call(&mut store, ())?;

    // Print info
    println!("[*] Mutated WASM payload:\n{}", mutated_code);
    std::fs::write(
        format!("mutated_{}.wat", chrono::Utc::now().timestamp()),
        mutated_code.clone()
    ).expect("Failed to save mutated file");

    println!(
        "[*] Length: {} bytes | Hash: {:x}",
        mutated_code.len(),
        md5::compute(&mutated_code)
    );

    println!("[*] Mutation complete and executed!");
    Ok(())
}
