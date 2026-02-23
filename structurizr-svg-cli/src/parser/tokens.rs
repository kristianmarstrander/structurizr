//! Token definitions for the DSL parser

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    // Keywords
    Workspace,
    Model,
    Views,
    Styles,
    Element,
    Relationship,
    Person,
    SoftwareSystem,
    Container,
    Component,
    DeploymentEnvironment,
    DeploymentNode,
    SystemLandscape,
    SystemContext,
    Include,
    Exclude,
    AutoLayout,
    Enterprise,
    Tags,
    
    // Style keywords
    Shape,
    Background,
    Color,
    Stroke,
    StrokeWidth,
    FontSize,
    Width,
    Height,
    Border,
    Opacity,
    Metadata,
    Description,
    Thickness,
    Style,
    Routing,
    Position,
    
    // Literals
    String(String),
    Identifier(String),
    
    // Operators
    Arrow,      // ->
    Equals,     // =
    
    // Delimiters
    LeftBrace,  // {
    RightBrace, // }
    
    // Special
    Comment,
    Newline,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, column: usize) -> Self {
        Self {
            token_type,
            line,
            column,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} at {}:{}", self.token_type, self.line, self.column)
    }
}
