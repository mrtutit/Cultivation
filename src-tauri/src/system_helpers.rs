use duct::cmd;
use ini::Ini;
use std::path::PathBuf;

#[tauri::command]
pub fn run_program(path: String, args: Option<String>) {
  // Without unwrap_or, this can crash when UAC prompt is denied
  open::that(format!("{} {}", &path, &args.unwrap_or("".into()))).unwrap_or(());
}

#[tauri::command]
pub fn run_program_relative(path: String, args: Option<String>) {
  // Save the current working directory
  let cwd = std::env::current_dir().unwrap();

  // Set the new working directory to the path before the executable
  let mut path_buf = std::path::PathBuf::from(&path);
  path_buf.pop();

  // Set new working directory
  std::env::set_current_dir(&path_buf).unwrap();

  // Without unwrap_or, this can crash when UAC prompt is denied
  open::that(format!("{} {}", &path, args.unwrap_or("".into()))).unwrap_or(());

  // Restore the original working directory
  std::env::set_current_dir(&cwd).unwrap();
}

#[tauri::command]
pub fn run_command(program: &str, args: Vec<&str>, relative: Option<bool>) {
  let prog = program.to_string();
  let args = args.iter().map(|s| s.to_string()).collect::<Vec<String>>();

  // Commands should not block (this is for the reshade injector mostly)
  std::thread::spawn(move || {
    // Save the current working directory
    let cwd = std::env::current_dir().unwrap();

    if relative.unwrap_or(false) {
      // Set the new working directory to the path before the executable
      let mut path_buf = std::path::PathBuf::from(&prog);
      path_buf.pop();

      // Set new working directory
      std::env::set_current_dir(&path_buf).unwrap();
    }

    cmd(prog, args).run().unwrap();

    // Restore the original working directory
    std::env::set_current_dir(&cwd).unwrap();
  });
}

#[tauri::command]
pub fn run_jar(path: String, execute_in: String, java_path: String) {
  let command = if java_path.is_empty() {
    format!("java -jar \"{}\"", path)
  } else {
    format!("\"{}\" -jar \"{}\"", java_path, path)
  };

  // Open the program from the specified path.
  match open::with(
    format!("/k cd /D \"{}\" & {}", &execute_in, &command),
    "C:\\Windows\\System32\\cmd.exe",
  ) {
    Ok(_) => (),
    Err(e) => println!("Failed to open jar ({} from {}): {}", &path, &execute_in, e),
  };
}

#[tauri::command]
pub fn open_in_browser(url: String) {
  // Open the URL in the default browser.
  match open::that(url) {
    Ok(_) => (),
    Err(e) => println!("Failed to open URL: {}", e),
  };
}

#[tauri::command]
pub fn install_location() -> String {
  let mut exe_path = std::env::current_exe().unwrap();

  // Get the path to the executable.
  exe_path.pop();

  return exe_path.to_str().unwrap().to_string();
}

#[tauri::command]
pub fn set_migoto_target(path: String, migoto_path: String) -> bool {
  let pathbuf = PathBuf::from(path);
  let mut migoto_pathbuf = PathBuf::from(migoto_path);

  migoto_pathbuf.pop();
  migoto_pathbuf.push("d3dx.ini");

  let mut conf = match Ini::load_from_file(&migoto_pathbuf) {
    Ok(c) => {
      println!("Loaded migoto ini");
      c
    }
    Err(e) => {
      println!("Error loading migoto config: {}", e);
      return false;
    }
  };

  // Set options
  conf
    .with_section(Some("Loader"))
    .set("target", pathbuf.to_str().unwrap());

  // Write file
  match conf.write_to_file(&migoto_pathbuf) {
    Ok(_) => {
      println!("Wrote config!");
      true
    }
    Err(e) => {
      println!("Error writing config: {}", e);
      false
    }
  }
}

#[cfg(windows)]
#[tauri::command]
pub fn is_elevated() -> bool {
  is_elevated::is_elevated()
}

#[cfg(unix)]
#[tauri::command]
pub fn is_elevated() -> bool {
  sudo::check() == sudo::RunningAs::Root
}
