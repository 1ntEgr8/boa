//! Object initializer parsing.
//!
//! More information:
//!  - [MDN documentation][mdn]
//!  - [ECMAScript specification][spec]
//!
//! [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Object_initializer
//! [spec]: https://tc39.es/ecma262/#sec-object-initializer

#[cfg(test)]
mod tests;

use crate::{
    syntax::{
        ast::{
            node::{self, MethodDefinitionKind, Node},
            punc::Punctuator,
            token::TokenKind,
        },
        parser::{
            expression::AssignmentExpression,
            function::{FormalParameters, FunctionBody},
            AllowAwait, AllowIn, AllowYield, Cursor, ParseError, ParseResult, TokenParser,
        },
    },
    Interner, Sym,
};

/// Parses an object literal.
///
/// More information:
///  - [MDN documentation][mdn]
///  - [ECMAScript specification][spec]
///
/// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Object_initializer
/// [spec]: https://tc39.es/ecma262/#prod-ObjectLiteral
#[derive(Debug, Clone, Copy)]
pub(super) struct ObjectLiteral {
    allow_yield: AllowYield,
    allow_await: AllowAwait,
}

impl ObjectLiteral {
    /// Creates a new `ObjectLiteral` parser.
    pub(super) fn new<Y, A>(allow_yield: Y, allow_await: A) -> Self
    where
        Y: Into<AllowYield>,
        A: Into<AllowAwait>,
    {
        Self {
            allow_yield: allow_yield.into(),
            allow_await: allow_await.into(),
        }
    }
}

impl TokenParser for ObjectLiteral {
    type Output = Node;

    fn parse(self, cursor: &mut Cursor<'_>, interner: &mut Interner) -> ParseResult {
        let mut elements = Vec::new();

        loop {
            if cursor.next_if(Punctuator::CloseBlock).is_some() {
                break;
            }

            elements.push(
                PropertyDefinition::new(self.allow_yield, self.allow_await)
                    .parse(cursor, interner)?,
            );

            if cursor.next_if(Punctuator::CloseBlock).is_some() {
                break;
            }

            if cursor.next_if(Punctuator::Comma).is_none() {
                let next_token = cursor.next().ok_or(ParseError::AbruptEnd)?;
                return Err(ParseError::expected(
                    vec![
                        Punctuator::Comma.to_string(),
                        Punctuator::CloseBlock.to_string(),
                    ],
                    next_token.display(interner).to_string(),
                    next_token.pos,
                    "object literal",
                ));
            }
        }

        Ok(Node::Object(elements))
    }
}

/// Parses a property definition.
///
/// More information:
///  - [ECMAScript specification][spec]
///
/// [spec]: https://tc39.es/ecma262/#prod-PropertyDefinition
#[derive(Debug, Clone, Copy)]
struct PropertyDefinition {
    allow_yield: AllowYield,
    allow_await: AllowAwait,
}

impl PropertyDefinition {
    /// Creates a new `PropertyDefinition` parser.
    fn new<Y, A>(allow_yield: Y, allow_await: A) -> Self
    where
        Y: Into<AllowYield>,
        A: Into<AllowAwait>,
    {
        Self {
            allow_yield: allow_yield.into(),
            allow_await: allow_await.into(),
        }
    }
}

impl TokenParser for PropertyDefinition {
    type Output = node::PropertyDefinition;

    fn parse(
        self,
        cursor: &mut Cursor<'_>,
        interner: &mut Interner,
    ) -> Result<Self::Output, ParseError> {
        if cursor.next_if(Punctuator::Spread).is_some() {
            let node = AssignmentExpression::new(true, self.allow_yield, self.allow_await)
                .parse(cursor, interner)?;
            return Ok(node::PropertyDefinition::SpreadObject(node));
        }

        let prop_name = cursor
            .next()
            .map(|tk| tk.to_string_sym(interner))
            .ok_or(ParseError::AbruptEnd)?;
        if cursor.next_if(Punctuator::Colon).is_some() {
            let val = AssignmentExpression::new(true, self.allow_yield, self.allow_await)
                .parse(cursor, interner)?;
            return Ok(node::PropertyDefinition::Property(prop_name, val));
        }

        // TODO: optimise comparison by using some pre-generated symbols.
        if cursor
            .next_if(TokenKind::Punctuator(Punctuator::OpenParen))
            .is_some()
            || prop_name == interner.get_or_intern("get")
            || prop_name == interner.get_or_intern("set")
        {
            return MethodDefinition::new(self.allow_yield, self.allow_await, prop_name)
                .parse(cursor, interner);
        }

        let pos = cursor
            .peek(0)
            .map(|tok| tok.pos)
            .ok_or(ParseError::AbruptEnd)?;
        Err(ParseError::general("expected property definition", pos))
    }
}

/// Parses a method definition.
///
/// More information:
///  - [ECMAScript specification][spec]
///
/// [spec]: https://tc39.es/ecma262/#prod-MethodDefinition
#[derive(Debug, Clone)]
struct MethodDefinition {
    allow_yield: AllowYield,
    allow_await: AllowAwait,
    identifier: Sym,
}

impl MethodDefinition {
    /// Creates a new `MethodDefinition` parser.
    fn new<Y, A>(allow_yield: Y, allow_await: A, identifier: Sym) -> Self
    where
        Y: Into<AllowYield>,
        A: Into<AllowAwait>,
    {
        Self {
            allow_yield: allow_yield.into(),
            allow_await: allow_await.into(),
            identifier,
        }
    }
}

impl TokenParser for MethodDefinition {
    type Output = node::PropertyDefinition;

    fn parse(
        self,
        cursor: &mut Cursor<'_>,
        interner: &mut Interner,
    ) -> Result<Self::Output, ParseError> {
        // TODO: optimise by avoiding string comparisons and using only symbols
        let (methodkind, prop_name, params) = match interner
            .resolve(self.identifier)
            .expect("identifier string disappeared")
        {
            idn @ "get" | idn @ "set" => {
                let prop_name = cursor
                    .next()
                    .map(|tk| tk.to_string_sym(interner))
                    .ok_or(ParseError::AbruptEnd)?;
                cursor.expect(
                    Punctuator::OpenParen,
                    "property method definition",
                    interner,
                )?;
                let first_param = cursor.peek(0).expect("current token disappeared").clone();
                let params = FormalParameters::new(false, false).parse(cursor, interner)?;
                cursor.expect(Punctuator::CloseParen, "method definition", interner)?;
                if idn == "get" {
                    if !params.is_empty() {
                        return Err(ParseError::unexpected(
                            first_param.display(interner).to_string(),
                            first_param.pos,
                            Some("getter functions must have no arguments"),
                        ));
                    }
                    (MethodDefinitionKind::Get, prop_name, params)
                } else {
                    if params.len() != 1 {
                        return Err(ParseError::unexpected(
                            first_param.display(interner).to_string(),
                            first_param.pos,
                            Some("setter functions must have one argument"),
                        ));
                    }
                    (MethodDefinitionKind::Set, prop_name, params)
                }
            }
            _ => {
                let params = FormalParameters::new(false, false).parse(cursor, interner)?;
                cursor.expect(Punctuator::CloseParen, "method definition", interner)?;
                (MethodDefinitionKind::Ordinary, self.identifier, params)
            }
        };

        cursor.expect(
            Punctuator::OpenBlock,
            "property method definition",
            interner,
        )?;
        let body = FunctionBody::new(false, false)
            .parse(cursor, interner)
            .map(Node::StatementList)?;
        cursor.expect(
            Punctuator::CloseBlock,
            "property method definition",
            interner,
        )?;

        Ok(node::PropertyDefinition::MethodDefinition(
            methodkind,
            prop_name,
            Node::FunctionDecl(None, params, Box::new(body)),
        ))
    }
}

/// Initializer parsing.
///
/// More information:
///  - [ECMAScript specification][spec]
///
/// [spec]: https://tc39.es/ecma262/#prod-Initializer
#[derive(Debug, Clone, Copy)]
pub(in crate::syntax::parser) struct Initializer {
    allow_in: AllowIn,
    allow_yield: AllowYield,
    allow_await: AllowAwait,
}

impl Initializer {
    /// Creates a new `Initializer` parser.
    pub(in crate::syntax::parser) fn new<I, Y, A>(
        allow_in: I,
        allow_yield: Y,
        allow_await: A,
    ) -> Self
    where
        I: Into<AllowIn>,
        Y: Into<AllowYield>,
        A: Into<AllowAwait>,
    {
        Self {
            allow_in: allow_in.into(),
            allow_yield: allow_yield.into(),
            allow_await: allow_await.into(),
        }
    }
}

impl TokenParser for Initializer {
    type Output = Node;

    fn parse(self, cursor: &mut Cursor<'_>, interner: &mut Interner) -> ParseResult {
        cursor.expect(Punctuator::Assign, "initializer", interner)?;
        AssignmentExpression::new(self.allow_in, self.allow_yield, self.allow_await)
            .parse(cursor, interner)
    }
}
