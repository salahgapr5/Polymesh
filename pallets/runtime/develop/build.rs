use wasm_builder_runner::WasmBuilder;

fn main() {
    WasmBuilder::new()
        .with_current_project()
        .with_wasm_builder_from_git(
            "https://github.com/paritytech/substrate.git",
            "b161460bf73e4cc73e29b8e01ab0323bb3d95e84",
        )
        .export_heap_base()
        .import_memory()
        .build()
}
