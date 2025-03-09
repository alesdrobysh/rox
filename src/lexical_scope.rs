use crate::logger;

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub depth: Option<usize>,
}

impl Clone for Variable {
    fn clone(&self) -> Self {
        Variable {
            name: self.name.clone(),
            depth: self.depth,
        }
    }
}

#[derive(Debug)]
pub struct LexicalScopeRegistry {
    variables: Vec<Variable>,
    pub depth: usize,
}

impl LexicalScopeRegistry {
    pub fn new() -> Self {
        LexicalScopeRegistry {
            variables: Vec::new(),
            depth: 0,
        }
    }

    pub fn increment_depth(&mut self) {
        self.depth += 1;
    }

    pub fn decrement_depth(&mut self) {
        self.depth -= 1;
    }

    pub fn add_local(&mut self, name: String) -> Result<(), String> {
        self.variables.push(Variable { name, depth: None });
        Ok(())
    }

    pub fn mark_initialized(&mut self) -> Result<(), String> {
        logger::debug("Marking variable as initialized");

        if let Some(last) = self.variables.last_mut() {
            last.depth = Some(self.depth);
        }
        Ok(())
    }

    pub fn resolve_local(&self, name: &str) -> Result<Option<usize>, String> {
        let length = self.variables.len();

        let result = self
            .variables
            .iter()
            .rev()
            .position(|variable| variable.name == name)
            .map(|position| length - position - 1);

        if let Some(position) = result {
            if self.variables[position].depth.is_none() {
                return Err(format!("Variable {} is not initialized", name));
            }
        }

        Ok(result)
    }

    pub fn peek(&self) -> Option<&Variable> {
        self.variables.last()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Variable> {
        self.variables.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.variables.is_empty()
    }

    pub fn pop(&mut self) {
        self.variables.pop();
    }
}
