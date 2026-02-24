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
        let path = path.as_ref();
        let base_dir = path.parent().unwrap_or(Path::new("."));
        let content = Self::preprocess_file(path, base_dir)?;
        self.parse(&content)
    }

    /// Recursively resolve `!include <file>` directives relative to `base_dir`.
    fn preprocess_file(path: &Path, base_dir: &Path) -> Result<String> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {:?}", path))?;
        Self::preprocess_includes(&content, base_dir)
    }

    fn preprocess_includes(content: &str, base_dir: &Path) -> Result<String> {
        let mut result = String::new();
        for line in content.lines() {
            if let Some(rest) = line.trim().strip_prefix("!include ") {
                let include_path = base_dir.join(rest.trim());
                let included = Self::preprocess_file(&include_path, base_dir)?;
                result.push_str(&included);
            } else {
                result.push_str(line);
            }
            result.push('\n');
        }
        Ok(result)
    }

    pub fn parse(&mut self, content: &str) -> Result<Workspace> {
        self.tokenize(content)?;
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
                // `#rrggbb` hex color or `# comment` — distinguish by next char
                '#' => {
                    if chars.peek().map(|c| c.is_ascii_hexdigit()).unwrap_or(false) {
                        // Hex color literal — collect as a String token
                        let start_col = column;
                        let mut color = String::from('#');
                        column += 1;
                        while let Some(&next_ch) = chars.peek() {
                            if next_ch.is_ascii_hexdigit() {
                                color.push(next_ch);
                                chars.next();
                                column += 1;
                            } else {
                                break;
                            }
                        }
                        self.tokens
                            .push(Token::new(TokenType::String(color), line, start_col));
                    } else {
                        // Hash comment — skip to end of line
                        while chars.peek() != Some(&'\n') && chars.peek().is_some() {
                            chars.next();
                            column += 1;
                        }
                    }
                }
                // Line comment `//` or lone `/`
                '/' => {
                    if chars.peek() == Some(&'/') {
                        while chars.peek() != Some(&'\n') && chars.peek().is_some() {
                            chars.next();
                            column += 1;
                        }
                    } else {
                        column += 1;
                    }
                }
                // `!include` is handled by preprocessing; other `!` directives are skipped
                '!' => {
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
                    self.tokens.push(Token::new(
                        TokenType::Identifier("*".to_string()),
                        line,
                        column,
                    ));
                    column += 1;
                }
                '"' => {
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

                    self.tokens
                        .push(Token::new(TokenType::String(string), line, start_col));
                }
                '-' => {
                    if chars.peek() == Some(&'>') {
                        chars.next();
                        self.tokens
                            .push(Token::new(TokenType::Arrow, line, column));
                        column += 2;
                    } else {
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
            "group" => TokenType::Group,
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
        self.tokens
            .get(self.current)
            .unwrap_or(&self.tokens[self.tokens.len() - 1])
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
            _ => anyhow::bail!(
                "Expected string, found {} at {}:{}",
                token,
                token.line,
                token.column
            ),
        }
    }

    fn expect_identifier(&mut self) -> Result<String> {
        let token = self.advance();
        match &token.token_type {
            TokenType::Identifier(s) => Ok(s.clone()),
            TokenType::String(s) => Ok(s.clone()),
            _ => anyhow::bail!(
                "Expected identifier, found {} at {}:{}",
                token,
                token.line,
                token.column
            ),
        }
    }

    fn expect_string_or_any_token(&mut self) -> Result<String> {
        let token = self.advance();
        match &token.token_type {
            TokenType::String(s) => Ok(s.clone()),
            TokenType::Identifier(s) => Ok(s.clone()),
            _ => Ok(format!("{:?}", token.token_type).to_lowercase()),
        }
    }

    /// Skip from `{` to the matching `}`, consuming both.
    fn skip_brace_block(&mut self) -> Result<()> {
        self.expect(TokenType::LeftBrace)?;
        let mut depth = 1usize;
        while depth > 0 {
            match self.peek().token_type {
                TokenType::LeftBrace => {
                    depth += 1;
                    self.advance();
                }
                TokenType::RightBrace => {
                    depth -= 1;
                    self.advance();
                }
                TokenType::Eof => break,
                _ => {
                    self.advance();
                }
            }
        }
        Ok(())
    }

    fn parse_workspace(&mut self) -> Result<Workspace> {
        self.expect(TokenType::Workspace)?;

        let name = self.expect_string()?;
        let description = self.expect_string().unwrap_or_default();

        let mut workspace = Workspace::new(name, description);

        self.expect(TokenType::LeftBrace)?;

        while !matches!(
            self.peek().token_type,
            TokenType::RightBrace | TokenType::Eof
        ) {
            match self.peek().token_type.clone() {
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
        self.parse_model_body(workspace, &mut element_vars)?;
        self.expect(TokenType::RightBrace)?;
        Ok(())
    }

    /// Parse the interior of a model/group/enterprise block, populating `workspace`
    /// and `element_vars` (variable-name → element-id map).
    fn parse_model_body(
        &mut self,
        workspace: &mut Workspace,
        element_vars: &mut HashMap<String, String>,
    ) -> Result<()> {
        while !matches!(
            self.peek().token_type,
            TokenType::RightBrace | TokenType::Eof
        ) {
            match self.peek().token_type.clone() {
                // Direct element declarations (no variable assignment)
                TokenType::Person | TokenType::SoftwareSystem => {
                    let (var_name, element) =
                        self.parse_element(None, workspace, element_vars)?;
                    if let Some(var) = var_name {
                        element_vars.insert(var, element.id.clone());
                    }
                    workspace.add_element(element);
                }
                // group/enterprise — transparent wrappers; parse their body as model content
                TokenType::Group | TokenType::Enterprise => {
                    self.advance();
                    // Optional name string
                    if matches!(
                        self.peek().token_type,
                        TokenType::String(_) | TokenType::Identifier(_)
                    ) {
                        self.advance();
                    }
                    self.expect(TokenType::LeftBrace)?;
                    self.parse_model_body(workspace, element_vars)?;
                    self.expect(TokenType::RightBrace)?;
                }
                TokenType::Identifier(_) => {
                    // Peek one ahead: `name = type ...` vs `name -> dest ...`
                    let next_is_equals = self.current + 1 < self.tokens.len()
                        && matches!(self.tokens[self.current + 1].token_type, TokenType::Equals);

                    if next_is_equals {
                        // Variable assignment: delegate to parse_element which handles `name =`
                        let (var_name, element) =
                            self.parse_element(None, workspace, element_vars)?;
                        if let Some(var) = var_name {
                            element_vars.insert(var, element.id.clone());
                        }
                        workspace.add_element(element);
                    } else {
                        // Relationship: source -> destination "description" "technology"
                        let source_var = match self.peek().token_type.clone() {
                            TokenType::Identifier(s) => s,
                            _ => unreachable!(),
                        };
                        self.advance();

                        if matches!(self.peek().token_type, TokenType::Arrow) {
                            self.advance();
                            let dest_var = self.expect_identifier()?;
                            let description = self.expect_string().unwrap_or_default();
                            let technology =
                                if matches!(self.peek().token_type, TokenType::String(_)) {
                                    Some(self.expect_string()?)
                                } else {
                                    None
                                };

                            if let (Some(source_id), Some(dest_id)) = (
                                element_vars.get(&source_var).cloned(),
                                element_vars.get(&dest_var).cloned(),
                            ) {
                                let rel_id = format!("rel_{}", self.relationship_counter);
                                self.relationship_counter += 1;

                                let mut rel = Relationship::new(
                                    rel_id,
                                    source_id,
                                    dest_id,
                                    description,
                                );
                                rel.technology = technology;
                                workspace.add_relationship(rel);
                            }
                        }
                        // else: orphan identifier — skip
                    }
                }
                _ => {
                    self.advance();
                }
            }
        }
        Ok(())
    }

    fn parse_element(
        &mut self,
        parent_id: Option<String>,
        workspace: &mut Workspace,
        element_vars: &mut HashMap<String, String>,
    ) -> Result<(Option<String>, Element)> {
        let mut var_name = None;

        // Handle optional variable assignment: `varName = type "name" ...`
        if matches!(self.peek().token_type, TokenType::Identifier(_)) {
            let potential_var = self.expect_identifier()?;
            if matches!(self.peek().token_type, TokenType::Equals) {
                self.advance(); // consume `=`
                var_name = Some(potential_var);
            }
            // If no `=`, the identifier was unexpected here; continue to read element type
        }

        let element_type_token = self.advance().clone();
        let element_type = match element_type_token.token_type {
            TokenType::Person => ElementType::Person,
            TokenType::SoftwareSystem => ElementType::SoftwareSystem,
            TokenType::Container => ElementType::Container,
            TokenType::Component => ElementType::Component,
            _ => anyhow::bail!(
                "Unexpected element type: {:?} at {}:{}",
                element_type_token.token_type,
                element_type_token.line,
                element_type_token.column
            ),
        };

        let name = self.expect_string()?;
        let description = if matches!(self.peek().token_type, TokenType::String(_)) {
            Some(self.expect_string()?)
        } else {
            None
        };

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

        if matches!(self.peek().token_type, TokenType::LeftBrace) {
            self.advance();

            while !matches!(
                self.peek().token_type,
                TokenType::RightBrace | TokenType::Eof
            ) {
                match self.peek().token_type.clone() {
                    TokenType::Tags => {
                        self.advance();
                        let tags_str = self.expect_string()?;
                        for tag in tags_str.split(',') {
                            element.add_tag(tag.trim().to_string());
                        }
                    }
                    // Direct child declaration inside a SoftwareSystem or Container
                    TokenType::Container | TokenType::Component => {
                        let (child_var, child) = self.parse_element(
                            Some(element_id.clone()),
                            workspace,
                            element_vars,
                        )?;
                        element.add_child(child.id.clone());
                        if let Some(var) = child_var {
                            element_vars.insert(var, child.id.clone());
                        }
                        workspace.add_element(child);
                    }
                    // Variable-assigned child: `varName = container/component ...`
                    TokenType::Identifier(_) => {
                        let next_is_equals = self.current + 1 < self.tokens.len()
                            && matches!(
                                self.tokens[self.current + 1].token_type,
                                TokenType::Equals
                            );
                        let type_after_eq = self.current + 2 < self.tokens.len()
                            && matches!(
                                self.tokens[self.current + 2].token_type,
                                TokenType::Container | TokenType::Component
                            );

                        if next_is_equals && type_after_eq {
                            let (child_var, child) = self.parse_element(
                                Some(element_id.clone()),
                                workspace,
                                element_vars,
                            )?;
                            element.add_child(child.id.clone());
                            if let Some(var) = child_var {
                                element_vars.insert(var, child.id.clone());
                            }
                            workspace.add_element(child);
                        } else {
                            self.advance();
                        }
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

        while !matches!(
            self.peek().token_type,
            TokenType::RightBrace | TokenType::Eof
        ) {
            match self.peek().token_type.clone() {
                TokenType::SystemLandscape => {
                    self.advance();
                    let view = self.parse_view(ViewType::SystemLandscape, workspace)?;
                    workspace.add_view(view);
                }
                TokenType::SystemContext => {
                    self.advance();
                    let _scope_element = self.expect_identifier()?;
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
                // Unknown block (e.g. `dynamic`, `filtered`) — skip balanced braces
                TokenType::LeftBrace => {
                    self.skip_brace_block()?;
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

        while !matches!(
            self.peek().token_type,
            TokenType::RightBrace | TokenType::Eof
        ) {
            match self.peek().token_type.clone() {
                TokenType::Include => {
                    self.advance();
                    let include_spec = match &self.peek().token_type {
                        TokenType::String(_) => self.expect_string()?,
                        TokenType::Identifier(_) => self.expect_identifier()?,
                        _ => {
                            self.advance();
                            continue;
                        }
                    };

                    if include_spec == "*" {
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
                    // consume the element identifier being excluded
                    match &self.peek().token_type {
                        TokenType::Identifier(_) | TokenType::String(_) => {
                            self.advance();
                        }
                        _ => {}
                    }
                }
                TokenType::AutoLayout => {
                    self.advance();
                    // Direction is optional; default to TopBottom
                    let direction = match &self.peek().token_type {
                        TokenType::String(_) => {
                            let s = self.expect_string()?;
                            Direction::from_str(&s).unwrap_or(Direction::TopBottom)
                        }
                        TokenType::Identifier(_) => {
                            let s = self.expect_identifier()?;
                            Direction::from_str(&s).unwrap_or(Direction::TopBottom)
                        }
                        _ => Direction::TopBottom,
                    };
                    view.auto_layout = Some(AutoLayout::new(direction));
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

        while !matches!(
            self.peek().token_type,
            TokenType::RightBrace | TokenType::Eof
        ) {
            match self.peek().token_type.clone() {
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

        while !matches!(
            self.peek().token_type,
            TokenType::RightBrace | TokenType::Eof
        ) {
            match self.peek().token_type.clone() {
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

        while !matches!(
            self.peek().token_type,
            TokenType::RightBrace | TokenType::Eof
        ) {
            match self.peek().token_type.clone() {
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

