use token::Token;

pub trait Node {
    fn start(&self) -> usize;
    // fn end(&self) -> usize;
    fn string(&self) -> String;
}

pub trait Expr: Node {}

pub trait Stmt: Node {}

pub trait Decl: Node {}

pub struct SemiColonStmt {
    pub pos: usize,
}
pub struct ExprStmt {
    pub x: Box<dyn Expr>,
}

pub struct IncDecStmt {
    pub pos: usize,
    pub op: Token,
    pub id: Ident,
}

pub struct AssignStmt {
    pub lhs: Ident,
    pub pos: usize,
    pub op: Token,
    pub rhs: Ident,
}

pub struct BlockStmt {
    pub lbrace: usize,
    pub stmts: Vec<Box<dyn Stmt>>,
    pub rbrace: usize,
}

pub struct IfStmt {
    pub if_pos: usize,
    pub lparen_pos: usize,
    pub cond: Box<dyn Expr>,
    pub rparen_pos: usize,
    pub init: Box<dyn Stmt>,
    pub elifs: Vec<ElseIf>,
    pub _else: Option<Else>,
}

pub struct ElseIf {
    pub else_pos: usize,
    pub if_pos: usize,
    pub lparen_pos: usize,
    pub cond: Box<dyn Expr>,
    pub rparen_pos: usize,
    pub init: Box<dyn Stmt>,
}
pub struct Else {
    pub pos: usize,
    pub init: Box<dyn Stmt>,
}

pub struct WhileStmt {
    pub pos: usize,
    pub lparen_pos: usize,
    pub cond: Box<dyn Expr>,
    pub rparen_pos: usize,
    pub init: Box<dyn Stmt>,
}

pub struct DowhileStmt {
    pub do_pos: usize,
    pub init: Box<dyn Stmt>,
    pub while_pos: usize,
    pub lbrace_pos: usize,
    pub cond: Box<dyn Expr>,
    pub rbrace_pos: usize,
}
pub struct ReturnStmt {
    pub pos: usize,                   // position of the 'return' keyword
    pub value: Option<Box<dyn Expr>>, // the return value
    pub semi: usize,
}

pub struct BreakStmt {
    pub pos: usize,  // position of the 'break' keyword
    pub semi: usize, // position of the ';'
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

impl Node for IfStmt {
    fn start(&self) -> usize {
        self.if_pos
    }

    fn string(&self) -> String {
        let mut s = format!("if ({}) {}", self.cond.string(), self.init.string());

        for elif in &self.elifs {
            s.push_str(format!("else if ({}) {}", elif.cond.string(), elif.init.string()).as_str());
        }

        if let Some(_else) = &self._else {
            s.push_str(format!("else {}", _else.init.string()).as_str());
        }

        s
    }
}

impl Node for BlockStmt {
    fn start(&self) -> usize {
        self.lbrace
    }

    fn string(&self) -> String {
        let mut s = String::from("{\n");

        for stmt in self.stmts.iter() {
            s.push_str(format!("\t{}\n", stmt.string()).as_str());
        }

        s.push_str("}");

        s
    }
}

impl Node for WhileStmt {
    fn start(&self) -> usize {
        self.pos
    }
    fn string(&self) -> String {
        format!("while ({}) {}", self.cond.string(), self.init.string())
    }
}

// mark all the statement nodes

impl Stmt for ReturnStmt {}
impl Stmt for BreakStmt {}
impl Stmt for ContinueStmt {}
impl Stmt for IfStmt {}
impl Stmt for BlockStmt {}
impl Stmt for WhileStmt {}

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

pub struct InfixExpr {
    pub x: Box<dyn Expr>,
    pub op_pos: usize,
    pub op: Token, // +, -, *, /
    pub y: Box<dyn Expr>,
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

impl Node for InfixExpr {
    fn start(&self) -> usize {
        self.x.start()
    }

    fn string(&self) -> String {
        format!(
            "({} {} {})",
            self.x.string(),
            self.op.to_str(),
            self.y.string()
        )
    }
}

impl Expr for BasicLit {}
impl Expr for UnaryExpr {}
impl Expr for Ident {}
impl Expr for StarExpr {}
impl Expr for InfixExpr {}
