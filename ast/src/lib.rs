use token::Position;

pub trait Node {
    fn start(&self) -> Position;
    // fn end(&self) -> Position;
    fn string(&self) -> String;
}

pub trait Expr: Node {}

pub trait Stmt: Node {}

pub trait Decl: Node {}

pub struct ReturnStmt {
    pub pos: Position,                // position of the 'return' keyword
    pub value: Option<Box<dyn Expr>>, // the return value
}

pub struct BreakStmt {
    pub pos: Position, // position of the 'break' keyword
}

pub struct ContinueStmt {
    pub pos: Position, // position of the 'continue' keyword
}

impl Node for ReturnStmt {
    fn start(&self) -> Position {
        self.pos
    }

    fn string(&self) -> String {
        match &self.value {
            Some(return_value) => {
                format!("return {};", return_value.string())
            }

            None => "return;".to_string(),
        }
    }
}

impl Node for BreakStmt {
    fn start(&self) -> Position {
        self.pos
    }

    fn string(&self) -> String {
        "break;".to_string()
    }
}

impl Node for ContinueStmt {
    fn start(&self) -> Position {
        self.pos
    }

    fn string(&self) -> String {
        "continue;".to_string()
    }
}

// mark all the statement nodes

impl Stmt for ReturnStmt {}
impl Stmt for BreakStmt {}
impl Stmt for ContinueStmt {}
