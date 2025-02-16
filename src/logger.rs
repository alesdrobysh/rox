enum DebugLevel {
    None,
    Info,
    Debug,
}

impl DebugLevel {
    pub fn from_str(level: &str) -> DebugLevel {
        match level {
            "info" => DebugLevel::Info,
            "debug" => DebugLevel::Debug,
            _ => DebugLevel::None,
        }
    }

    pub fn is_debug(&self) -> bool {
        match self {
            DebugLevel::Debug => true,
            _ => false,
        }
    }

    pub fn is_info(&self) -> bool {
        match self {
            DebugLevel::Debug | DebugLevel::Info => true,
            _ => false,
        }
    }
}

pub fn debug(message: &str) {
    match std::env::var("DEBUG").map(|s| DebugLevel::from_str(s.as_str())) {
        Ok(level) if level.is_debug() => {
            eprintln!("{}", message)
        }
        _ => {}
    }
}

pub fn info(message: &str) {
    match std::env::var("DEBUG").map(|s| DebugLevel::from_str(s.as_str())) {
        Ok(level) if level.is_info() => {
            eprintln!("{}", message)
        }
        _ => {}
    }
}
