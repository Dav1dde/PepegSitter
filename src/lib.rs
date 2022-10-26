#![doc = include_str!("../README.md")]

macro_rules! pepegsit {
    ($lang:ident, $feature:expr, $path:expr) => {
        /// Tree-Sitter parser for this grammar.
        #[cfg(feature = $feature)]
        pub mod $lang {
            include!(concat!(env!("OUT_DIR"), $path));
        }
    };
}

pepegsit!(bash, "bash", "/lang_bash.rs");
pepegsit!(c, "c", "/lang_c.rs");
pepegsit!(cpp, "cpp", "/lang_cpp.rs");
pepegsit!(css, "css", "/lang_css.rs");
pepegsit!(d, "d", "/lang_d.rs");
pepegsit!(go, "go", "/lang_go.rs");
pepegsit!(haskell, "haskell", "/lang_haskell.rs");
pepegsit!(html, "html", "/lang_html.rs");
pepegsit!(java, "java", "/lang_java.rs");
pepegsit!(javascript, "javascript", "/lang_javascript.rs");
pepegsit!(json, "json", "/lang_json.rs");
pepegsit!(lua, "lua", "/lang_lua.rs");
pepegsit!(python, "python", "/lang_python.rs");
pepegsit!(rust, "rust", "/lang_rust.rs");
pepegsit!(toml, "toml", "/lang_toml.rs");
pepegsit!(tsx, "typescript-tsx", "/lang_tsx.rs");
pepegsit!(typescript, "typescript-typescript", "/lang_typescript.rs");
pepegsit!(yaml, "yaml", "/lang_yaml.rs");
