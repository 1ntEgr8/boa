//! This module implements the `Keyword` structure, which represents reserved words of the JavaScript language.
//!
//! More information:
//!  - [ECMAScript reference][spec]
//!  - [MDN documentation][mdn]
//!
//! [spec]: https://www.ecma-international.org/ecma-262/#sec-keywords
//! [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Lexical_grammar#Keywords

use std::{
    error,
    fmt::{Display, Error, Formatter},
    str::FromStr,
};

#[cfg(feature = "serde-ast")]
use serde::{Deserialize, Serialize};

/// Keywords are tokens that have special meaning in JavaScript.
///
/// In JavaScript you cannot use these reserved words as variables, labels, or function names.
///
/// More information:
///  - [ECMAScript reference][spec]
///  - [MDN documentation][mdn]
///
/// [spec]: https://www.ecma-international.org/ecma-262/#sec-keywords
/// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Lexical_grammar#Keywords
#[cfg_attr(feature = "serde-ast", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Keyword {
    /// The `await` keyword.
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#prod-AwaitExpression
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/await
    Await,

    /// The `break` keyword.
    ///
    /// More information:
    ///  - [break `Node` documentation][node]
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#prod-BreakStatement
    /// [node]: ../node/enum.Node.html#variant.Break
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/break
    Break,

    /// The `case` keyword.
    ///
    /// More information:
    ///  - [switch `Node` documentation][node]
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#prod-CaseClause
    /// [node]: ../node/enum.Node.html#variant.Switch
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/switch
    Case,

    /// The `catch` keyword.
    ///
    /// More information:
    ///  - [try `Node` documentation][node]
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#prod-Catch
    /// [node]: ../node/enum.Node.html#variant.Try
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/try...catch
    Catch,

    /// The `class` keyword.
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#prod-ClassDeclaration
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/class
    Class,

    /// The `continue` keyword.
    ///
    /// More information:
    ///  - [continue `Node` documentation][node]
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#prod-ContinueStatement
    /// [node]: ../node/enum.Node.html#variant.Continue
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/continue
    Continue,

    /// The `const` keyword.
    ///
    /// More information:
    ///  - [const `Node` documentation][node]
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-let-and-const-declarations
    /// [node]: ../node/enum.Node.html#variant.ConstDecl
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/const
    Const,

    /// The `debugger` keyword.
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-debugger-statement
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/debugger
    Debugger,

    /// The `default` keyword.
    ///
    /// More information:
    ///  - [switch `Node` documentation][node]
    ///  - [ECMAScript reference default clause][spec-clause]
    ///  - [ECMAScript reference default export][spec-export]
    ///  - [MDN documentation][mdn]
    ///
    /// [node]: ../node/enum.Node.html#variant.Switch
    /// [spec-clause]: https://tc39.es/ecma262/#prod-DefaultClause
    /// [spec-export]: https://tc39.es/ecma262/#prod-ImportedDefaultBinding
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/default
    Default,

    /// The `delete` keyword.
    ///
    /// More information:
    ///  - [delete `UnaryOp` documentation][unary]
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-delete-operator
    /// [unary]: ../op/enum.UnaryOp.html#variant.Delete
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/delete
    Delete,

    /// The `do` keyword.
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-do-while-statement
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/do...while
    Do,

    /// The `else` keyword.
    ///
    /// More information:
    ///  - [if `Node` documentation][node]
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [node]: ../node/enum.Node.html#variant.If
    /// [spec]: https://tc39.es/ecma262/#prod-IfStatement
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/if...else
    Else,

    /// The `enum` keyword.
    ///
    /// Future reserved keyword.
    Enum,

    /// The `export` keyword.
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-exports
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/export
    Export,

    /// The `extends` keyword.
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#prod-ClassHeritage
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Classes/extends
    Extends,

    /// The `finally` keyword.
    ///
    /// More information:
    ///  - [try `Node` documentation][node]
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [node]: ../node/enum.Node.html#variant.Try
    /// [spec]: https://tc39.es/ecma262/#prod-Finally
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/try...catch
    Finally,

    /// The `for` keyword.
    ///
    /// More information:
    ///  - [for loop `Node` documentation][node]
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [node]: ../node/enum.Node.html#variant.ForLoop
    /// [spec]: https://tc39.es/ecma262/#prod-ForDeclaration
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/for
    For,

    /// The `function` keyword.
    ///
    /// More information:
    ///  - [function `Node` documentation][node]
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [node]: ../node/enum.Node.html#variant.FunctionDecl
    /// [spec]: https://tc39.es/ecma262/#sec-terms-and-definitions-function
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/function
    Function,

    /// The `if` keyword.
    ///
    /// More information:
    ///  - [if `Node` documentation][node]
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [node]: ../node/enum.Node.html#variant.If
    /// [spec]: https://tc39.es/ecma262/#prod-IfStatement
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/if...else
    If,

    /// The `in` keyword.
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#prod-RelationalExpression
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/in
    In,

    /// The `instanceof` keyword.
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-instanceofoperator
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/instanceof
    InstanceOf,

    /// The `import` keyword.
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-imports
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/import
    Import,

    /// The `let` keyword.
    ///
    /// More information:
    ///  - [let `Node` documentation][node]
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [node]: ../node/enum.Node.html#variant.LetDecl
    /// [spec]: https://tc39.es/ecma262/#sec-let-and-const-declarations
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/let
    Let,

    /// The `new` keyword.
    ///
    /// More information:
    ///  - [new `Node` documentation][node]
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [node]: ../node/enum.Node.html#variant.New
    /// [spec]: https://tc39.es/ecma262/#prod-NewExpression
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/new
    New,

    /// The `return` keyword
    ///
    /// More information:
    ///  - [return `Node` documentation][node]
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [node]: ../node/enum.Node.html#variant.Return
    /// [spec]: https://tc39.es/ecma262/#prod-ReturnStatement
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/return
    Return,

    /// The `super` keyword
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-super-keyword
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/super
    Super,

    /// The `switch` keyword.
    ///
    /// More information:
    ///  - [switch `Node` documentation][node]
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [node]: ../node/enum.Node.html#variant.Switch
    /// [spec]: https://tc39.es/ecma262/#prod-SwitchStatement
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/switch
    Switch,

    /// The `this` keyword.
    ///
    /// More information:
    ///  - [this `Node` documentation][node]
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [node]: ../node/enum.Node.html#variant.This
    /// [spec]: https://tc39.es/ecma262/#sec-this-keyword
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/this
    This,

    /// The `throw` keyword.
    ///
    /// More information:
    ///  - [throw `Node` documentation][node]
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [node]: ../node/enum.Node.html#variant.Throw
    /// [spec]: https://tc39.es/ecma262/#prod-ArrowFunction
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Functions/Arrow_functions
    Throw,

    /// The `try` keyword.
    ///
    /// More information:
    ///  - [try `Node` documentation][node]
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [node]: ../node/enum.Node.html#variant.Try
    /// [spec]: https://tc39.es/ecma262/#prod-TryStatement
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/try...catch
    Try,

    /// The `typeof` keyword.
    ///
    /// More information:
    ///  - [typeof `UnaryOp` documentation][unary]
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [unary]: ../op/enum.UnaryOp.html#variant.TypeOf
    /// [spec]: https://tc39.es/ecma262/#sec-typeof-operator
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/typeof
    TypeOf,

    /// The `var` keyword.
    ///
    /// More information:
    ///  - [var `Node` documentation][node]
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [node]: ../node/enum.Node.html#variant.VarDecl
    /// [spec]: https://tc39.es/ecma262/#prod-VariableStatement
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/var
    Var,

    /// The `void` keyword.
    ///
    /// More information:
    ///  - [void `UnaryOp` documentation][unary]
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [unary]: ../op/enum.UnaryOp.html#variant.Void
    /// [spec]: https://tc39.es/ecma262/#sec-void-operator
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/void
    Void,

    /// The `while` keyword.
    ///
    /// More information:
    ///  - [while `Node` documentation][node]
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [node]: ../node/enum.Node.html#variant.While
    /// [spec]: https://tc39.es/ecma262/#prod-grammar-notation-WhileStatement
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/while
    While,

    /// The `with` keyword.
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#prod-WithStatement
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/with
    With,

    /// The 'yield' keyword.
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#prod-YieldExpression
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/yield
    Yield,
}

#[derive(Debug, Clone, Copy)]
pub struct KeywordError;
impl Display for KeywordError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "invalid token")
    }
}

// This is important for other errors to wrap this one.
impl error::Error for KeywordError {
    fn description(&self) -> &str {
        "invalid token"
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
impl FromStr for Keyword {
    type Err = KeywordError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "await" => Ok(Keyword::Await),
            "break" => Ok(Keyword::Break),
            "case" => Ok(Keyword::Case),
            "catch" => Ok(Keyword::Catch),
            "class" => Ok(Keyword::Class),
            "continue" => Ok(Keyword::Continue),
            "const" => Ok(Keyword::Const),
            "debugger" => Ok(Keyword::Debugger),
            "default" => Ok(Keyword::Default),
            "delete" => Ok(Keyword::Delete),
            "do" => Ok(Keyword::Do),
            "else" => Ok(Keyword::Else),
            "enum" => Ok(Keyword::Enum),
            "extends" => Ok(Keyword::Extends),
            "export" => Ok(Keyword::Export),
            "finally" => Ok(Keyword::Finally),
            "for" => Ok(Keyword::For),
            "function" => Ok(Keyword::Function),
            "if" => Ok(Keyword::If),
            "in" => Ok(Keyword::In),
            "instanceof" => Ok(Keyword::InstanceOf),
            "import" => Ok(Keyword::Import),
            "let" => Ok(Keyword::Let),
            "new" => Ok(Keyword::New),
            "return" => Ok(Keyword::Return),
            "super" => Ok(Keyword::Super),
            "switch" => Ok(Keyword::Switch),
            "this" => Ok(Keyword::This),
            "throw" => Ok(Keyword::Throw),
            "try" => Ok(Keyword::Try),
            "typeof" => Ok(Keyword::TypeOf),
            "var" => Ok(Keyword::Var),
            "void" => Ok(Keyword::Void),
            "while" => Ok(Keyword::While),
            "with" => Ok(Keyword::With),
            "yield" => Ok(Keyword::Yield),
            _ => Err(KeywordError),
        }
    }
}
impl Display for Keyword {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match *self {
                Keyword::Await => "await",
                Keyword::Break => "break",
                Keyword::Case => "case",
                Keyword::Catch => "catch",
                Keyword::Class => "class",
                Keyword::Continue => "continue",
                Keyword::Const => "const",
                Keyword::Debugger => "debugger",
                Keyword::Default => "default",
                Keyword::Delete => "delete",
                Keyword::Do => "do",
                Keyword::Else => "else",
                Keyword::Enum => "enum",
                Keyword::Extends => "extends",
                Keyword::Export => "export",
                Keyword::Finally => "finally",
                Keyword::For => "for",
                Keyword::Function => "function",
                Keyword::If => "if",
                Keyword::In => "in",
                Keyword::InstanceOf => "instanceof",
                Keyword::Import => "import",
                Keyword::Let => "let",
                Keyword::New => "new",
                Keyword::Return => "return",
                Keyword::Super => "super",
                Keyword::Switch => "switch",
                Keyword::This => "this",
                Keyword::Throw => "throw",
                Keyword::Try => "try",
                Keyword::TypeOf => "typeof",
                Keyword::Var => "var",
                Keyword::Void => "void",
                Keyword::While => "while",
                Keyword::With => "with",
                Keyword::Yield => "yield",
            }
        )
    }
}