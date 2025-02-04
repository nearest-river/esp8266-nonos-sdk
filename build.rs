use bindgen::RustEdition;
use std::{
  fs,
  io,
  env,
  path::Path,
};



static CLANG_ARGS: [&str;3]=["-I","./include","-w"];
static XTENSA_CARGO_PATH: &str=concat!(env!("RUSTUP_HOME"),"toolchains/esp/bin/cargo");
static ENV_VARS_REMOVE: [&str;2]=["TARGET","TOOLCHAIN"];
static ENV_VARS_REASSIGN: [(&str,&str);1]=[
  ("CARGO",XTENSA_CARGO_PATH),
];



fn main()-> io::Result<()> {
  init();
  generate_bindings()
}

fn init() {
  println!("cargo::rustc-link-search={}","./libs/");
  println!("cargo::rerun-if-changed=src/bindings/wrapper/");

  // init env
  for key in ENV_VARS_REMOVE {
    env::remove_var(key);
  }

  for (key,val) in ENV_VARS_REASSIGN {
    env::set_var(key,val);
  }
}

fn generate_bindings()-> io::Result<()> {
  for entry in fs::read_dir("./src/bindings/wrapper")?.flatten() {
    let name=entry.file_name();
    let wrapper_path=entry.path();
    let metadata=entry.metadata()?;
    if !metadata.is_file() {
      continue;
    }

    let dest=Path::new("./src/bindings/")
    .join(name)
    .with_extension("rs");

    let bindings=bindgen::builder()
    .header(wrapper_path.into_os_string().into_string().unwrap())
    .use_core()
    .layout_tests(false)
    .wrap_unsafe_ops(true)
    .merge_extern_blocks(true)
    .rust_edition(RustEdition::Edition2021)
    .clang_args(CLANG_ARGS)
    .generate()
    .unwrap()
    .to_string()
    .replacen(r#"unsafe extern "C" {"#,r#"extern "C" {"#,usize::MAX);
    // idk why but appearently xtensa-rustc doesn't support `unsafe extern`-blocks.

    fs::write(dest,bindings)?;
  }


  Ok(())
}



