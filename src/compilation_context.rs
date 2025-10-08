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

#[derive(Debug, Default)]
struct LexicalScopeRegistry {
    variables: Vec<Variable>,
    pub depth: usize,
}

impl LexicalScopeRegistry {
    fn new() -> Self {
        LexicalScopeRegistry {
            variables: Vec::new(),
            depth: 0,
        }
    }
}

#[derive(Debug)]
pub struct Upvalue {
    pub index: usize,
    pub is_local: bool,
}

#[derive(Debug, Default)]
pub struct CompilationContext {
    scope_registry: LexicalScopeRegistry,
    enclosing: Option<Box<CompilationContext>>,
    pub upvalues: Vec<Upvalue>,
}

impl CompilationContext {
    pub fn new(enclosing: Option<Box<CompilationContext>>) -> Self {
        CompilationContext {
            scope_registry: LexicalScopeRegistry::new(),
            enclosing,
            upvalues: Vec::new(),
        }
    }

    pub fn increment_depth(&mut self) {
        self.scope_registry.depth += 1;
    }

    pub fn decrement_depth(&mut self) {
        self.scope_registry.depth -= 1;
    }

    pub fn add_local(&mut self, name: String) -> Result<(), String> {
        self.scope_registry
            .variables
            .push(Variable { name, depth: None });
        Ok(())
    }

    pub fn mark_initialized(&mut self) -> Result<(), String> {
        logger::debug("Marking variable as initialized");

        if self.scope_registry.depth == 0 {
            return Ok(());
        }

        if let Some(last) = self.scope_registry.variables.last_mut() {
            last.depth = Some(self.scope_registry.depth);
        }
        Ok(())
    }

    pub fn resolve_local(&self, name: &str) -> Result<Option<usize>, String> {
        let length = self.scope_registry.variables.len();

        let result = self
            .scope_registry
            .variables
            .iter()
            .rev()
            .position(|variable| variable.name == name)
            .map(|position| length - position - 1);

        if let Some(position) = result {
            if self.scope_registry.variables[position].depth.is_none() {
                return Err(format!("Variable {} is not initialized", name));
            }
        }

        Ok(result)
    }

    pub fn resolve_upvalue(&mut self, name: &str) -> Result<Option<usize>, String> {
        match &mut self.enclosing {
            Some(enclosing) => {
                let result = enclosing.resolve_local(name)?;
                if let Some(local) = result {
                    self.add_upvalue(local, true)?;
                    return Ok(Some(self.upvalues.len() - 1));
                }

                let upvalue = enclosing.resolve_upvalue(name)?;
                if let Some(upvalue) = upvalue {
                    self.add_upvalue(upvalue, false)?;
                    return Ok(Some(self.upvalues.len() - 1));
                }

                return Ok(None);
            }
            _ => Ok(None),
        }
    }

    pub fn peek(&self) -> Option<&Variable> {
        self.scope_registry.variables.last()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Variable> {
        self.scope_registry.variables.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.scope_registry.variables.is_empty()
    }

    pub fn pop(&mut self) {
        self.scope_registry.variables.pop();
    }

    pub fn get_depth(&self) -> usize {
        self.scope_registry.depth
    }

    pub fn take_enclosing(&mut self) -> Option<Self> {
        self.enclosing.take().map(|boxed| *boxed)
    }

    fn add_upvalue(&mut self, index: usize, is_local: bool) -> Result<(), String> {
        // compatibility with clox
        if self.upvalues.len() > u8::MAX as usize {
            return Err("Too many closure variables in function.".to_string());
        }

        let existing_upvalue = self
            .upvalues
            .iter()
            .find(|upvalue| upvalue.index == index && upvalue.is_local == is_local);

        if existing_upvalue.is_none() {
            self.upvalues.push(Upvalue { index, is_local });
        }

        Ok(())
    }
}
