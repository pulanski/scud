///////////////////////
// Dry-run Executors //
///////////////////////


////////////////////////
// Standard Executors //
////////////////////////
pub fn execute_push_git() {
    match Command::new("git")
        .arg("push")
        .status() {
        Ok(status) => {
            if !status.success() {
                panic!("Failed to push files");
            }
        }
        Err(error) => {
            panic!("Failed to push files: {}", error);
        }
    }
}
