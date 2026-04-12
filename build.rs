fn main() {
    slint_build::compile_with_config(
            "src/main_ui.slint",
                    slint_build::CompilerConfiguration::new()
                                .with_style("fluent".into())
                                    ).unwrap();
                                    }
                                    
