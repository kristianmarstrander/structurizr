//! DSL parser implementation

use crate::model::*;
use crate::parser::tokens::{Token, TokenType};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct DslParser {
    tokens: Vec<Token>,
    current: usize,
    element_counter: usize,
    relationship_counter: usize,
}

impl DslParser {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            current: 0,
            element_counter: 0,
            relationship_counter: 0,
        }
    }

    pub fn parse_file<P: AsRef<Path>>(&mut self, path: P) -> Result<Workspace> {
        let content = fs::read_to_string(path.as_ref())
            .with_context(|| format!("Failed to read file: {:?}", path.as_ref()))?;
        self.parse(&content)
    }

    pub fn parse(&mut self, content: &str) -> Result<Workspace> {
        // Tokenize
        self.tokenize(content)?;
        
        // Parse workspace
        self.parse_workspace()
    }

    fn tokenize(&mut self, content: &str) -> Result<()> {
        self.tokens.clear();
        
        let mut line = 1;
        let mut column = 1;
        let mut chars = content.chars().peekable();
        
        while let Some(ch) = chars.next() {
            match ch {
                '\n' => {
                    line += 1;
                    column = 1;
                }
                ' ' | '\t' | '\r' => {
                    column += 1;
                }
                '#' => {
                    // Comment - skip to end of line
                    while chars.peek() != Some(&'\n') && chars.peek().is_some() {
                        chars.next();
                        column += 1;
                    }
                }
                '{' => {
                    self.tokens.push(Token::new(TokenType::LeftBrace, line, column));
                    column += 1;
                }
                '}' => {
                    self.tokens.push(Token::new(TokenType::RightBrace, line, column));
                    column += 1;
                }
                '=' => {
                    self.tokens.push(Token::new(TokenType::Equals, line, column));
                    column += 1;
                }
                '*' => {
                    // Wildcard for include
                    self.tokens.push(Token::new(TokenType::Identifier("*".to_string()), line, column));
                    column += 1;
                }
                '"' => {
                    // String literal
                    let start_col = column;
                    column += 1;
                    let mut string = String::new();
                    
                    while let Some(&next_ch) = chars.peek() {
                        if next_ch == '"' {
                            chars.next();
                            column += 1;
                            break;
                        }
                        if next_ch == '\\' {
                            chars.next();
                            column += 1;
                            if let Some(&escaped) = chars.peek() {
                                chars.next();
                                column += 1;
                                string.push(escaped);
                            }
                        } else {
                            string.push(next_ch);
                            chars.next();
                            column += 1;
                        }
                    }
                    
                    self.tokens.push(Token::new(TokenType::String(string), line, start_col));
                }
                '-' => {
                    // Check for arrow ->
                    if chars.peek() == Some(&'>') {
                        chars.next();
                        self.tokens.push(Token::new(TokenType::Arrow, line, column));
                        column += 2;
                    } else {
                        // Part of identifier
                        let start_col = column;
                        let mut ident = String::from(ch);
                        column += 1;
                        
                        while let Some(&next_ch) = chars.peek() {
                            if next_ch.is_alphanumeric() || next_ch == '_' || next_ch == '-' {
                                ident.push(next_ch);
                                chars.next();
                                column += 1;
                            } else {
                                break;
                            }
                        }
                        
                        let token_type = self.keyword_or_identifier(&ident);
                        self.tokens.push(Token::new(token_type, line, start_col));
                    }
                }
                _ if ch.is_alphabetic() || ch == '_' || ch.is_numeric() => {
                    // Identifier or keyword
                    let start_col = column;
                    let mut ident = String::from(ch);
                    column += 1;
                    
                    while let Some(&next_ch) = chars.peek() {
                        if next_ch.is_alphanumeric() || next_ch == '_' || next_ch == '-' {
                            ident.push(next_ch);
                            chars.next();
                            column += 1;
                        } else {
                            break;
                        }
                    }
                    
                    let token_type = self.keyword_or_identifier(&ident);
                    self.tokens.push(Token::new(token_type, line, start_col));
                }
                _ => {
                    column += 1;
                }
            }
        }
        
        self.tokens.push(Token::new(TokenType::Eof, line, column));
        Ok(())
    }

    fn keyword_or_identifier(&self, s: &str) -> TokenType {
        match s.to_lowercase().as_str() {
            "workspace" => TokenType::Workspace,
            "model" => TokenType::Model,
            "views" => TokenType::Views,
            "styles" => TokenType::Styles,
            "element" => TokenType::Element,
            "relationship" => TokenType::Relationship,
            "person" => TokenType::Person,
            "softwaresystem" => TokenType::SoftwareSystem,
            "container" => TokenType::Container,
            "component" => TokenType::Component,
            "deploymentenvironment" => TokenType::DeploymentEnvironment,
            "deploymentnode" => TokenType::DeploymentNode,
            "systemlandscape" => TokenType::SystemLandscape,
            "systemcontext" => TokenType::SystemContext,
            "include" => TokenType::Include,
            "exclude" => TokenType::Exclude,
            "autolayout" => TokenType::AutoLayout,
            "enterprise" => TokenType::Enterprise,
            "tags" => TokenType::Tags,
            "shape" => TokenType::Shape,
            "background" => TokenType::Background,
            "color" => TokenType::Color,
            "colour" => TokenType::Color,
            "stroke" => TokenType::Stroke,
            "strokewidth" => TokenType::StrokeWidth,
            "fontsize" => TokenType::FontSize,
            "width" => TokenType::Width,
            "height" => TokenType::Height,
            "border" => TokenType::Border,
            "opacity" => TokenType::Opacity,
            "metadata" => TokenType::Metadata,
            "description" => TokenType::Description,
            "thickness" => TokenType::Thickness,
            "style" => TokenType::Style,
            "routing" => TokenType::Routing,
            "position" => TokenType::Position,
            _ => TokenType::Identifier(s.to_string()),
        }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or(&self.tokens[self.tokens.len() - 1])
    }

    fn advance(&mut self) -> &Token {
        if self.current < self.tokens.len() - 1 {
            self.current += 1;
        }
        &self.tokens[self.current - 1]
    }

    fn expect(&mut self, expected: TokenType) -> Result<()> {
        let token = self.peek();
        if std::mem::discriminant(&token.token_type) == std::mem::discriminant(&expected) {
            self.advance();
            Ok(())
        } else {
            anyhow::bail!(
                "Expected {:?}, found {} at {}:{}",
                expected,
                token,
                token.line,
                token.column
            )
        }
    }

    fn expect_string(&mut self) -> Result<String> {
        let token = self.advance();
        match &token.token_type {
            TokenType::String(s) => Ok(s.clone()),
            _ => anyhow::bail!("Expected string, found {} at {}:{}", token, token.line, token.column),
        }
    }

    fn expect_identifier(&mut self) -> Result<String> {
        let token = self.advance();
        match &token.token_type {
            TokenType::Identifier(s) => Ok(s.clone()),
            TokenType::String(s) => Ok(s.clone()),
            _ => anyhow::bail!("Expected identifier, found {} at {}:{}", token, token.line, token.column),
        }
    }

    fn expect_string_or_any_token(&mut self) -> Result<String> {
        let token = self.advance();
        match &token.token_type {
            TokenType::String(s) => Ok(s.clone()),
            TokenType::Identifier(s) => Ok(s.clone()),
            // Allow any keyword token as a string (for shape names, etc.)
            _ => Ok(format!("{:?}", token.token_type).to_lowercase()),
        }
    }

    fn parse_workspace(&mut self) -> Result<Workspace> {
        self.expect(TokenType::Workspace)?;
        
        let name = self.expect_string()?;
        let description = self.expect_string().unwrap_or_default();
        
        let mut workspace = Workspace::new(name, description);
        
        self.expect(TokenType::LeftBrace)?;
        
        while !matches!(self.peek().token_type, TokenType::RightBrace | TokenType::Eof) {
            match &self.peek().token_type {
                TokenType::Model => {
                    self.advance();
                    self.parse_model(&mut workspace)?;
                }
                TokenType::Views => {
                    self.advance();
                    self.parse_views(&mut workspace)?;
                }
                _ => {
                    self.advance();
                }
            }
        }
        
        self.expect(TokenType::RightBrace)?;
        
        Ok(workspace)
    }

    fn parse_model(&mut self, workspace: &mut Workspace) -> Result<()> {
        self.expect(TokenType::LeftBrace)?;
        
        let mut element_vars: HashMap<String, String> = HashMap::new();
        
        while !matches!(self.peek().token_type, TokenType::RightBrace | TokenType::Eof) {
            match &self.peek().token_type {
                TokenType::Person | TokenType::SoftwareSystem => {
                    let (var_name, element) = self.parse_element(None)?;
                    if let Some(var) = var_name {
                        element_vars.insert(var, element.id.clone());
                    }
                    workspace.add_element(element);
                }
                TokenType::Enterprise => {
                    self.advance();
                    self.expect_string()?; // enterprise name
                    self.expect(TokenType::LeftBrace)?;
                    
                    while !matches!(self.peek().token_type, TokenType::RightBrace | TokenType::Eof) {
                        match &self.peek().token_type {
                            TokenType::Person | TokenType::SoftwareSystem => {
                                let (var_name, element) = self.parse_element(None)?;
                                if let Some(var) = var_name {
                                    element_vars.insert(var, element.id.clone());
                                }
                                workspace.add_element(element);
                            }
                            _ => {
                                self.advance();
                            }
                        }
                    }
                    
                    self.expect(TokenType::RightBrace)?;
                }
                TokenType::Identifier(var) => {
                    // Relationship: source -> destination "description" "technology"
                    let source_var = var.clone();
                    self.advance();
                    
                    if matches!(self.peek().token_type, TokenType::Arrow) {
                        self.advance();
                        let dest_var = self.expect_identifier()?;
                        let description = self.expect_string().unwrap_or_default();
                        let technology = if matches!(self.peek().token_type, TokenType::String(_)) {
                            Some(self.expect_string()?)
                        } else {
                            None
                        };
                        
                        if let (Some(source_id), Some(dest_id)) = (
                            element_vars.get(&source_var),
                            element_vars.get(&dest_var),
                        ) {
                            let rel_id = format!("rel_{}", self.relationship_counter);
                            self.relationship_counter += 1;
                            
                            let mut rel = Relationship::new(
                                rel_id,
                                source_id.clone(),
                                dest_id.clone(),
                                description,
                            );
                            rel.technology = technology;
                            workspace.add_relationship(rel);
                        }
                    }
                }
                _ => {
                    self.advance();
                }
            }
        }
        
        self.expect(TokenType::RightBrace)?;
        Ok(())
    }

    fn parse_element(&mut self, parent_id: Option<String>) -> Result<(Option<String>, Element)> {
        // Optional variable assignment
        let mut var_name = None;
        let _start_token = self.peek().clone();
        
        // Check for variable = 
        if matches!(self.peek().token_type, TokenType::Identifier(_)) {
            let potential_var = self.expect_identifier()?;
            if matches!(self.peek().token_type, TokenType::Equals) {
                self.advance();
                var_name = Some(potential_var);
            } else {
                // Backtrack - it wasn't a variable assignment
                // We need to handle this case... for simplicity, let's just continue
            }
        }
        
        let element_type_token = self.advance().clone();
        let element_type = match element_type_token.token_type {
            TokenType::Person => ElementType::Person,
            TokenType::SoftwareSystem => ElementType::SoftwareSystem,
            TokenType::Container => ElementType::Container,
            TokenType::Component => ElementType::Component,
            _ => anyhow::bail!("Unexpected element type"),
        };
        
        let name = self.expect_string()?;
        let description = if matches!(self.peek().token_type, TokenType::String(_)) {
            Some(self.expect_string()?)
        } else {
            None
        };
        
        // Optional technology
        let technology = if matches!(self.peek().token_type, TokenType::String(_)) {
            Some(self.expect_string()?)
        } else {
            None
        };
        
        let element_id = format!("element_{}", self.element_counter);
        self.element_counter += 1;
        
        let mut element = Element::new(element_id.clone(), name, description, element_type);
        element.technology = technology;
        element.parent_id = parent_id;
        
        // Check for body
        if matches!(self.peek().token_type, TokenType::LeftBrace) {
            self.advance();
            
            while !matches!(self.peek().token_type, TokenType::RightBrace | TokenType::Eof) {
                match &self.peek().token_type {
                    TokenType::Tags => {
                        self.advance();
                        let tags_str = self.expect_string()?;
                        for tag in tags_str.split(',') {
                            element.add_tag(tag.trim().to_string());
                        }
                    }
                    TokenType::Container | TokenType::Component => {
                        let (_, child) = self.parse_element(Some(element_id.clone()))?;
                        element.add_child(child.id.clone());
                        // Note: We need to add child to workspace too, but we can't do that here
                        // This is a simplification - in a real implementation, we'd need to handle this differently
                    }
                    _ => {
                        self.advance();
                    }
                }
            }
            
            self.expect(TokenType::RightBrace)?;
        }
        
        Ok((var_name, element))
    }

    fn parse_views(&mut self, workspace: &mut Workspace) -> Result<()> {
        self.expect(TokenType::LeftBrace)?;
        
        while !matches!(self.peek().token_type, TokenType::RightBrace | TokenType::Eof) {
            match &self.peek().token_type {
                TokenType::SystemLandscape => {
                    self.advance();
                    let view = self.parse_view(ViewType::SystemLandscape, workspace)?;
                    workspace.add_view(view);
                }
                TokenType::SystemContext => {
                    self.advance();
                    let _scope_element = self.expect_identifier()?; // scope element
                    let view = self.parse_view(ViewType::SystemContext, workspace)?;
                    workspace.add_view(view);
                }
                TokenType::Container => {
                    self.advance();
                    let _scope_element = self.expect_identifier()?;
                    let view = self.parse_view(ViewType::Container, workspace)?;
                    workspace.add_view(view);
                }
                TokenType::Component => {
                    self.advance();
                    let _scope_element = self.expect_identifier()?;
                    let view = self.parse_view(ViewType::Component, workspace)?;
                    workspace.add_view(view);
                }
                TokenType::Styles => {
                    self.advance();
                    self.parse_styles(workspace)?;
                }
                _ => {
                    self.advance();
                }
            }
        }
        
        self.expect(TokenType::RightBrace)?;
        Ok(())
    }

    fn parse_view(&mut self, view_type: ViewType, workspace: &Workspace) -> Result<View> {
        let key = self.expect_string()?;
        let title = if matches!(self.peek().token_type, TokenType::String(_)) {
            Some(self.expect_string()?)
        } else {
            None
        };
        
        let mut view = View::new(key, view_type);
        view.title = title;
        
        self.expect(TokenType::LeftBrace)?;
        
        while !matches!(self.peek().token_type, TokenType::RightBrace | TokenType::Eof) {
            match &self.peek().token_type {
                TokenType::Include => {
                    self.advance();
                    // include can be followed by * or element identifier
                    let include_spec = match &self.peek().token_type {
                        TokenType::String(_) => self.expect_string()?,
                        TokenType::Identifier(_) => self.expect_identifier()?,
                        _ => {
                            let token = self.peek();
                            anyhow::bail!("Expected * or identifier after include, found {} at {}:{}", token, token.line, token.column);
                        }
                    };
                    
                    if include_spec == "*" {
                        // Include all elements
                        for element_id in workspace.elements.keys() {
                            view.add_element(element_id.clone());
                        }
                        for (idx, _) in workspace.relationships.iter().enumerate() {
                            view.add_relationship(format!("rel_{}", idx));
                        }
                    }
                }
                TokenType::Exclude => {
                    self.advance();
                    self.expect_identifier()?;
                }
                TokenType::AutoLayout => {
                    self.advance();
                    // autolayout can be followed by an identifier (tb, lr) or string
                    let direction_str = match &self.peek().token_type {
                        TokenType::String(_) => self.expect_string()?,
                        TokenType::Identifier(_) => self.expect_identifier()?,
                        _ => {
                            let token = self.peek();
                            anyhow::bail!("Expected direction after autolayout, found {} at {}:{}", token, token.line, token.column);
                        }
                    };
                    if let Some(direction) = Direction::from_str(&direction_str) {
                        view.auto_layout = Some(AutoLayout::new(direction));
                    }
                }
                _ => {
                    self.advance();
                }
            }
        }
        
        self.expect(TokenType::RightBrace)?;
        Ok(view)
    }

    fn parse_styles(&mut self, workspace: &mut Workspace) -> Result<()> {
        self.expect(TokenType::LeftBrace)?;
        
        while !matches!(self.peek().token_type, TokenType::RightBrace | TokenType::Eof) {
            match &self.peek().token_type {
                TokenType::Element => {
                    self.advance();
                    let tag = self.expect_string()?;
                    let style = self.parse_element_style(tag)?;
                    workspace.styles.add_element_style(style);
                }
                TokenType::Relationship => {
                    self.advance();
                    let tag = self.expect_string()?;
                    let style = self.parse_relationship_style(tag)?;
                    workspace.styles.add_relationship_style(style);
                }
                _ => {
                    self.advance();
                }
            }
        }
        
        self.expect(TokenType::RightBrace)?;
        Ok(())
    }

    fn parse_element_style(&mut self, tag: String) -> Result<ElementStyle> {
        let mut style = ElementStyle::new(tag);
        
        self.expect(TokenType::LeftBrace)?;
        
        while !matches!(self.peek().token_type, TokenType::RightBrace | TokenType::Eof) {
            match &self.peek().token_type {
                TokenType::Shape => {
                    self.advance();
                    let shape_str = self.expect_string_or_any_token()?;
                    style.shape = Shape::from_str(&shape_str);
                }
                TokenType::Background => {
                    self.advance();
                    style.background = Some(self.expect_string()?);
                }
                TokenType::Color => {
                    self.advance();
                    style.color = Some(self.expect_string()?);
                }
                TokenType::Stroke => {
                    self.advance();
                    style.stroke = Some(self.expect_string()?);
                }
                TokenType::StrokeWidth => {
                    self.advance();
                    let width_str = self.expect_identifier()?;
                    style.stroke_width = width_str.parse().ok();
                }
                TokenType::FontSize => {
                    self.advance();
                    let size_str = self.expect_identifier()?;
                    style.font_size = size_str.parse().ok();
                }
                TokenType::Width => {
                    self.advance();
                    let width_str = self.expect_identifier()?;
                    style.width = width_str.parse().ok();
                }
                TokenType::Height => {
                    self.advance();
                    let height_str = self.expect_identifier()?;
                    style.height = height_str.parse().ok();
                }
                TokenType::Opacity => {
                    self.advance();
                    let opacity_str = self.expect_identifier()?;
                    style.opacity = opacity_str.parse().ok();
                }
                TokenType::Border => {
                    self.advance();
                    let border_str = self.expect_identifier()?;
                    style.border = Border::from_str(&border_str);
                }
                _ => {
                    self.advance();
                }
            }
        }
        
        self.expect(TokenType::RightBrace)?;
        Ok(style)
    }

    fn parse_relationship_style(&mut self, tag: String) -> Result<RelationshipStyle> {
        let mut style = RelationshipStyle::new(tag);
        
        self.expect(TokenType::LeftBrace)?;
        
        while !matches!(self.peek().token_type, TokenType::RightBrace | TokenType::Eof) {
            match &self.peek().token_type {
                TokenType::Color => {
                    self.advance();
                    style.color = Some(self.expect_string()?);
                }
                TokenType::Thickness => {
                    self.advance();
                    let thickness_str = self.expect_identifier()?;
                    style.thickness = thickness_str.parse().ok();
                }
                TokenType::Style => {
                    self.advance();
                    let style_str = self.expect_identifier()?;
                    style.style = Border::from_str(&style_str);
                }
                _ => {
                    self.advance();
                }
            }
        }
        
        self.expect(TokenType::RightBrace)?;
        Ok(style)
    }
}

impl Default for DslParser {
    fn default() -> Self {
        Self::new()
    }
}
