use {
    kiro_editor::{self as kiro, Editor, StdinRawMode, HELP},
    std::{
        env, 
        io, 
        process::exit,
    },
};

// This function directly opens the Kiro Editor
pub fn spawn_editor(files: Vec<String>) -> kiro::Result<()> {
    let input = StdinRawMode::new()?.input_keys();
    let output = io::stdout();
    let window_size: Option<(usize, usize)> = {
        use termsize;
        let termsize::Size { rows, cols } = termsize::get();
        Some((rows, cols))
    };
    
    Editor::open(
        input,
        output,
        window_size,
        &files
    )?.edit()
}

