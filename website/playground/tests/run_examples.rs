use anyhow::{bail, Context, Result};
use glob::glob;
use koto::{prelude::*, Ptr, PtrMut};
use std::fs;

#[test]
fn run_playground_examples() -> Result<()> {
    let mut found_script = false;

    let output = PtrMut::from(String::new());

    let mut koto = Koto::with_settings(KotoSettings {
        stdout: make_ptr!(OutputCapture {
            output: output.clone(),
        }),
        stderr: make_ptr!(OutputCapture { output }),
        run_tests: true,
        run_import_tests: true,
        ..Default::default()
    });

    for path in glob("**/*.koto").context("failed to scan for scripts")? {
        let path = path.context("failed to read path")?;
        let script = fs::read_to_string(&path)
            .with_context(|| format!("failed to read from path '{path:?}'"))?;

        if let Err(e) = koto.compile_and_run(&script) {
            bail!("error while running script '{path:?}'\n  -> {}", e.to_string());
        }

        found_script = true;
    }

    assert!(found_script);

    Ok(())
}

#[derive(Debug)]
struct OutputCapture {
    output: PtrMut<String>,
}

impl KotoFile for OutputCapture {
    fn id(&self) -> KString {
        "_stdout_".into()
    }
}

impl KotoRead for OutputCapture {}
impl KotoWrite for OutputCapture {
    fn write(&self, bytes: &[u8]) -> koto::runtime::Result<()> {
        let bytes_str = match std::str::from_utf8(bytes) {
            Ok(s) => s,
            Err(e) => return Err(e.to_string().into()),
        };
        self.output.borrow_mut().push_str(bytes_str);
        Ok(())
    }

    fn write_line(&self, output: &str) -> koto::runtime::Result<()> {
        let mut unlocked = self.output.borrow_mut();
        unlocked.push_str(output);
        unlocked.push('\n');
        Ok(())
    }

    fn flush(&self) -> koto::runtime::Result<()> {
        Ok(())
    }
}
