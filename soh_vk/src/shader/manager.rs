//-----------------------------------------------------------------------------
use anyhow::Result;
use std::path::Path;
//-----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Precompile all shaders in directories
    Precompile,
    /// Compile (and save) shaders on demand
    CompileOnDemand,
}

//-----------------------------------------------------------------------------
// Builder
pub struct ManagerBuilder {
    mode: Mode,
    recompile: bool,
    directory: String,
}

impl ManagerBuilder {
    pub fn new() -> Self {
        return ManagerBuilder {
            mode: Mode::Precompile,
            recompile: false,
            directory: "shaders".to_owned(),
        };
    }

    pub fn mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        return self;
    }

    pub fn recompile(mut self, recompile: bool) -> Self {
        self.recompile = recompile;
        return self;
    }

    pub fn directory(mut self, directory: &str) -> Self {
        self.directory = directory.to_owned();
        return self;
    }

    pub fn build(self) -> Result<Manager> {
        return Manager::new(self.mode, self.recompile, self.directory);
    }
}

impl Default for ManagerBuilder {
    fn default() -> Self {
        return Self::new();
    }
}

//-----------------------------------------------------------------------------

pub struct Manager {
    compiler: shaderc::Compiler,
    options: shaderc::CompileOptions<'static>,

    mode: Mode,
    recompile: bool,
    directory: String,
}

//-----------------------------------------------------------------------------
// Constructor
impl Manager {
    pub fn new(mode: Mode, recompile: bool, directory: String) -> Result<Manager> {
        // Create compiler
        let compiler = shaderc::Compiler::new()?;

        // Create options
        let mut options = shaderc::CompileOptions::new()?;

        options.set_source_language(shaderc::SourceLanguage::GLSL);
        options.set_optimization_level(shaderc::OptimizationLevel::Performance);

        let manager = Manager {
            compiler,
            options,

            mode,
            recompile,
            directory,
        };

        std::fs::create_dir_all(format!("{}/compiled", manager.directory))?;

        if manager.mode == Mode::Precompile {
            manager.precompile()?;
        }

        return Ok(manager);
    }

    pub fn get_shader(&self, shader_filename: &str) -> Result<Vec<u32>> {
        let shader_filename = format!("{}/{}", self.directory, shader_filename);
        let binary_filename = Self::get_binary_filename(&shader_filename)?;

        if Self::binary_file_exists(&shader_filename) && !self.recompile {
            return self.load_from_file(&binary_filename);
        }

        let artifact = self.compile_shader(&shader_filename)?;
        return Ok(artifact.as_binary().to_owned());
    }

    // Loop over all shaders in `dir` and compile them
    fn precompile(&self) -> Result<()> {
        let dir_iterator = std::fs::read_dir(&self.directory)?.filter_map(Result::ok);

        for entry in dir_iterator {
            let path = entry.path();

            if path.is_dir() {
                continue;
            }

            if Self::binary_file_exists(&path) && !self.recompile {
                continue;
            }

            #[allow(unused)]
            let _ = self.compile_shader(&path).inspect_err(|err| {
                soh_log::log_warning!("Failed to precompile shaders: {}", err);
            });
        }

        return Ok(());
    }

    fn compile_shader<T: AsRef<Path>>(&self, path: T) -> Result<shaderc::CompilationArtifact> {
        // Check the filename and deduce the shader kind
        fn deduce_shader_kind(path: &Path) -> shaderc::ShaderKind {
            let Some(ext) = path.extension() else {
                soh_log::log_warning!("Couldn't deduce shader type for file \"{}\". Defaulting to \"shaderc::ShaderKind::InferFromSource\"", path.display());
                return shaderc::ShaderKind::InferFromSource;
            };

            if ext == "vert" {
                return shaderc::ShaderKind::Vertex;
            } else if ext == "frag" {
                return shaderc::ShaderKind::Fragment;
            } else {
                soh_log::log_warning!("Couldn't deduce shader type for file \"{}\". Defaulting to \"shaderc::ShaderKind::InferFromSource\"", path.display());
                return shaderc::ShaderKind::InferFromSource;
            }
        }

        // Save the compiled shader to a *.spv file
        fn save_compiled_shader(
            path: &Path,
            artifact: &shaderc::CompilationArtifact,
        ) -> std::io::Result<()> {
            let bin_file_path = Manager::get_binary_filename(path).unwrap();
            let data = artifact.as_binary_u8();

            soh_log::log_info!("Saving shader {:?}", bin_file_path);

            // let file = std::fs::OpenOptions::new().read(true).

            return std::fs::write(&bin_file_path, data);
        }

        let path = path.as_ref();
        if path.is_dir() {
            panic!(
                "Trying to compile shader \"{}\" which is a directory",
                path.display()
            );
        }

        soh_log::log_info!("Compiling shader \"{}\"", path.display());

        let shader_kind = deduce_shader_kind(path);
        let path_str = path.as_os_str().to_str().unwrap_or("");
        let source_text = std::fs::read_to_string(path)?;

        let artifact = self.compiler.compile_into_spirv(
            &source_text,
            shader_kind,
            path_str,
            "main",
            Some(&self.options),
        )?;

        save_compiled_shader(path, &artifact)?;

        return Ok(artifact);
    }

    fn binary_file_exists<T: AsRef<Path>>(path: T) -> bool {
        let path = Self::get_binary_filename(path).unwrap();

        return Path::new(&path).exists();
    }

    fn load_from_file<T: AsRef<Path>>(&self, path: T) -> Result<Vec<u32>> {
        soh_log::log_info!(
            "Loading precompiled shader: \"{}\"",
            path.as_ref().display()
        );

        let u8_data = std::fs::read(path)?;
        let u32_data: Vec<u32> = unsafe {
            std::slice::from_raw_parts::<'_, u32>(u8_data.as_ptr().cast(), u8_data.len() / 4)
        }
        .into();

        if u32_data[0] != 0x07230203 {
            soh_log::log_error!(
                "First byte isn't `0x07230203`, it is `{:#x}` instead",
                u32_data[0]
            );
        }

        return Ok(u32_data);
    }

    #[inline(always)]
    fn get_binary_filename<T: AsRef<Path>>(path: T) -> Result<String> {
        // This function looks very ugly
        let path = path.as_ref();

        anyhow::ensure!(
            path.is_file(),
            "Trying to get binary filename for a directory"
        );

        let dir = path.parent();

        let dir_str = if let Some(dir) = dir {
            dir.to_str().unwrap()
        } else {
            "."
        };

        let filename = path.file_name().unwrap().to_str().unwrap();

        return Ok(format!("{}/compiled/{}.spv", dir_str, filename));
    }
}

//-----------------------------------------------------------------------------
