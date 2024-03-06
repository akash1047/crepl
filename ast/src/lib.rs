use token::Token;

pub trait Node {
    fn start(&self) -> usize;
    // fn end(&self) -> usize;
    fn string(&self) -> String;
}

pub trait Expr: Node {}

pub trait Stmt: Node {}

pub trait Decl: Node {}

pub struct ReturnStmt {
    pub pos: usize,                   // position of the 'return' keyword
    pub value: Option<Box<dyn Expr>>, // the return value
}

pub struct BreakStmt {
    pub pos: usize, // position of the 'break' keyword
}

pub struct ContinueStmt {
    pub pos: usize, // position of the 'continue' keyword
}

impl Node for ReturnStmt {
    fn start(&self) -> usize {
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
    fn start(&self) -> usize {
        self.pos
    }

    fn string(&self) -> String {
        "break;".to_string()
    }
}

impl Node for ContinueStmt {
    fn start(&self) -> usize {
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

pub struct BasicLit {
    pub pos: usize,
    pub tok: Token,
    pub lit: String,
}

pub struct UnaryExpr {
    pub op_pos: usize,
    pub op: Token,
    pub x: Box<dyn Expr>,
}

pub struct Ident {
    pub pos: usize,
    pub name: String,
}

pub struct StarExpr {
    pub pos: usize,
    pub x: Box<dyn Expr>,
}

impl Node for BasicLit {
    fn start(&self) -> usize {
        self.pos
    }

    fn string(&self) -> String {
        self.lit.clone()
    }
}

impl Node for UnaryExpr {
    fn start(&self) -> usize {
        self.op_pos
    }

    fn string(&self) -> String {
        format!("({}{})", self.op.to_str(), self.x.string())
    }
}

impl Node for Ident {
    fn start(&self) -> usize {
        self.pos
    }

    fn string(&self) -> String {
        self.name.clone()
    }
}

impl Node for StarExpr {
    fn start(&self) -> usize {
        self.pos
    }

    fn string(&self) -> String {
        format!("(*{})", self.x.string())
    }
}

impl Expr for BasicLit {}
impl Expr for UnaryExpr {}
impl Expr for Ident {}
impl Expr for StarExpr {}
