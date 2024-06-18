use std::io::Write;
use base64::{
    alphabet, engine::{GeneralPurpose as Base64, GeneralPurposeConfig as Base64Config}, Engine
};

type BoxError = Box<dyn std::error::Error + Send + Sync>;

fn main() -> Result<(), BoxError> {
    let cwd = std::env::current_dir()
        .map_err(|e| format!("Can't obtain current directory: {e}"))?;

    let mut args = std::env::args();

    // Skip over binary name as first arg:
    args.next();

    // Path to the pkg dir containing merkleized_metadata_sys.js and merkleized_metadata_sys_bg.wasm
    let path_to_pkg = args.next().ok_or_else(|| {
        format!("Expected exactly 2 arguments (first being the command name), got {}", args.len())
    })?;

    // Make package path relative to cwd.
    let path_to_pkg = cwd.join(path_to_pkg);

    let path_to_js_entry = path_to_pkg.join("merkleized_metadata_sys.js");
    let path_to_types = path_to_pkg.join("merkleized_metadata_sys.d.ts");
    let path_to_wasm = path_to_pkg.join("merkleized_metadata_sys_bg.wasm");

    let wasm_bytes = std::fs::read(&path_to_wasm)
        .map_err(|e| format!("Can't read WASM file: {e}"))?;

    let wasm_base64 = Base64::new(&alphabet::STANDARD, Base64Config::new())
        .encode(&wasm_bytes);

    // rewrite the entry point file to this:
    let js_string = format!(r#"
        import * as bg from "./merkleized_metadata_sys_bg.js";

        function base64ToBytes(base64) {{
            const binString = atob(base64);
            return Uint8Array.from(binString, (m) => m.codePointAt(0));
        }}

        const wasmUintArray = base64ToBytes("{wasm_base64}");

        let initPromise = undefined;
        export function init() {{
            if (!initPromise) {{
                let imports = {{
                    // the WASM file expects to be given access to a few functions from here:
                    './merkleized_metadata_sys_bg.js': bg
                }}
                initPromise = WebAssembly.instantiate(wasmUintArray, imports).then((wasm) => {{
                    // __wbg_set_wasm then expects to be given the exports from the instantiated WASM:
                    bg.__wbg_set_wasm(wasm.instance.exports);
                }});
            }}
            return initPromise
        }}

        export * from "./merkleized_metadata_sys_bg.js";
    "#);

    std::fs::write(&path_to_js_entry, js_string)
        .map_err(|e| format!("Can't write replacement JS file: {e}"))?;

    std::fs::remove_file(&path_to_wasm)
        .map_err(|e| format!("Can't delete WASM file: {e}"))?;

    // append to the types definitions the init function:
    let types_addition = r#"
        export function init(): Promise<void>;
    "#;

    let mut types_file = std::fs::File::options().append(true).open(&path_to_types)
        .map_err(|e| format!("Can#'t open merkleized_metadata_sys.d.ts: {e}"))?;
    writeln!(&mut types_file, "{types_addition}")
        .map_err(|e| format!("Can#'t open merkleized_metadata_sys.d.ts: {e}"))?;

    Ok(())
}
