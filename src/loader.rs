use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

use crate::ast::{Program, TopLevelDecl};
use crate::lexer::{LexError, lex_file};
use crate::parser::{ParseError, Parser};

#[derive(Debug)]
pub enum LoadError {
    InvalidImportTarget { importer: PathBuf, import_path: String },
    Io { path: PathBuf, message: String },
    Cycle { chain: Vec<PathBuf> },
    Lex { path: PathBuf, src: String, error: LexError },
    Parse { path: PathBuf, src: String, error: ParseError },
}

impl Display for LoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::InvalidImportTarget { importer, import_path } => write!(
                f,
                "invalid import `{}` in {}: imports must target a .eres file",
                import_path,
                importer.display()
            ),
            LoadError::Io { path, message } => write!(f, "failed to read {}: {}", path.display(), message),
            LoadError::Cycle { chain } => write!(
                f,
                "module import cycle detected: {}",
                chain
                    .iter()
                    .map(|path| path.display().to_string())
                    .collect::<Vec<_>>()
                    .join(" -> ")
            ),
            LoadError::Lex { path, error, .. } => write!(f, "lex error in {}: {}", path.display(), error),
            LoadError::Parse { path, error, .. } => write!(f, "parse error in {}: {}", path.display(), error),
        }
    }
}

impl std::error::Error for LoadError {}

#[derive(Debug)]
pub struct LoadedProgram {
    pub program: Program,
    pub loaded_files: Vec<PathBuf>,
}

pub fn load_program_from_entry(path: impl AsRef<Path>) -> Result<LoadedProgram, LoadError> {
    let mut loader = ModuleLoader {
        programs: HashMap::new(),
        order: Vec::new(),
        in_progress: HashSet::new(),
        stack: Vec::new(),
    };
    let entry = canonicalize_existing(path.as_ref())?;
    loader.load_file(&entry)?;

    let mut items = Vec::new();
    for path in &loader.order {
        let program = loader.programs.get(path).expect("loaded program missing");
        for item in &program.items {
            if !matches!(item, TopLevelDecl::Use(_)) {
                items.push(item.clone());
            }
        }
    }

    Ok(LoadedProgram {
        program: Program { items },
        loaded_files: loader.order,
    })
}

struct ModuleLoader {
    programs: HashMap<PathBuf, Program>,
    order: Vec<PathBuf>,
    in_progress: HashSet<PathBuf>,
    stack: Vec<PathBuf>,
}

impl ModuleLoader {
    fn load_file(&mut self, path: &Path) -> Result<(), LoadError> {
        let canonical = canonicalize_existing(path)?;
        if self.programs.contains_key(&canonical) {
            return Ok(());
        }
        if self.in_progress.contains(&canonical) {
            let mut chain = self.stack.clone();
            chain.push(canonical);
            return Err(LoadError::Cycle { chain });
        }

        self.in_progress.insert(canonical.clone());
        self.stack.push(canonical.clone());

        let src = std::fs::read_to_string(&canonical).map_err(|error| LoadError::Io {
            path: canonical.clone(),
            message: error.to_string(),
        })?;
        let tokens = lex_file(&src).map_err(|error| LoadError::Lex {
            path: canonical.clone(),
            src: src.clone(),
            error,
        })?;
        let mut parser = Parser::new(&tokens);
        let program = parser.parse_program().map_err(|error| LoadError::Parse {
            path: canonical.clone(),
            src: src.clone(),
            error,
        })?;

        for item in &program.items {
            if let TopLevelDecl::Use(import_path) = item {
                let next = resolve_import_path(&canonical, import_path)?;
                self.load_file(&next)?;
            }
        }

        self.stack.pop();
        self.in_progress.remove(&canonical);
        self.order.push(canonical.clone());
        self.programs.insert(canonical, program);
        Ok(())
    }
}

fn resolve_import_path(importer: &Path, import_path: &str) -> Result<PathBuf, LoadError> {
    if !import_path.ends_with(".eres") {
        return Err(LoadError::InvalidImportTarget {
            importer: importer.to_path_buf(),
            import_path: import_path.to_string(),
        });
    }

    let base = importer.parent().unwrap_or_else(|| Path::new("."));
    canonicalize_existing(&base.join(import_path))
}

fn canonicalize_existing(path: &Path) -> Result<PathBuf, LoadError> {
    std::fs::canonicalize(path).map_err(|error| LoadError::Io {
        path: path.to_path_buf(),
        message: error.to_string(),
    })
}
