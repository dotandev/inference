#![warn(clippy::pedantic)]
#![allow(dead_code)]

use core::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, PartialEq, Eq, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Location {
    pub offset_start: u32,
    pub offset_end: u32,
    pub start_line: u32,
    pub start_column: u32,
    pub end_line: u32,
    pub end_column: u32,
    pub source: String,
}

impl Location {
    #[must_use]
    pub fn new(
        offset_start: u32,
        offset_end: u32,
        start_line: u32,
        start_column: u32,
        end_line: u32,
        end_column: u32,
        source: String,
    ) -> Self {
        Self {
            offset_start,
            offset_end,
            start_line,
            start_column,
            end_line,
            end_column,
            source,
        }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Location {{ offset_start: {}, offset_end: {}, start_line: {}, start_column: {}, end_line: {}, end_column: {}, source: {} }}",
            self.offset_start, self.offset_end, self.start_line, self.start_column, self.end_line, self.end_column, self.source
        )
    }
}

#[macro_export]
macro_rules! ast_node {
    (
        $(#[$outer:meta])*
        $struct_vis:vis struct $name:ident {
            $(
                $(#[$field_attr:meta])*
                $field_vis:vis $field_name:ident : $field_ty:ty
            ),* $(,)?
        }
    ) => {
        $(#[$outer])*
        #[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
        $struct_vis struct $name {
            pub id: u32,
            pub location: $crate::types::Location,
            $(
                $(#[$field_attr])*
                $field_vis $field_name : $field_ty,
            )*
        }
    };
}

macro_rules! ast_nodes {
    (
        $(
            $(#[$outer:meta])*
            $struct_vis:vis struct $name:ident { $($fields:tt)* }
        )+
    ) => {
        $(
            ast_node! {
                $(#[$outer])*
                $struct_vis struct $name { $($fields)* }
            }
        )+
    };
}

macro_rules! ast_enum {
    (
        $(#[$outer:meta])*
        $enum_vis:vis enum $name:ident {
            $(
                $(#[$arm_attr:meta])*
                $(@$conv:ident)? $arm:ident $( ( $($tuple:tt)* ) )? $( { $($struct:tt)* } )? ,
            )*
        }
    ) => {
        $(#[$outer])*
        #[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
        $enum_vis enum $name {
            $(
                $(#[$arm_attr])*
                $arm $( ( $($tuple)* ) )? $( { $($struct)* } )? ,
            )*
        }
    }
}

macro_rules! ast_enums {
    (
        $(
            $(#[$outer:meta])*
            $enum_vis:vis enum $name:ident { $($arms:tt)* }
        )+
    ) => {
        $(
            ast_enum! {
                $(#[$outer])*
                $enum_vis enum $name { $($arms)* }
            }
        )+
    };
}

ast_enums! {

    pub enum Definition {
        Spec(SpecDefinition),
        Struct(StructDefinition),
        Enum(EnumDefinition),
        Constant(ConstantDefinition),
        Function(FunctionDefinition),
        ExternalFunction(ExternalFunctionDefinition),
        Type(TypeDefinition),
    }

    pub enum BlockType {
        Block(Block),
        Assume(Block),
        Forall(Block),
        Exists(Block),
        Unique(Block),
    }

    pub enum Statement {
        Block(BlockType),
        Expression(ExpressionStatement),
        Return(ReturnStatement),
        Loop(LoopStatement),
        Break(BreakStatement),
        If(IfStatement),
        VariableDefinition(VariableDefinitionStatement),
        TypeDefinition(TypeDefinitionStatement),
        Assert(AssertStatement),
        ConstantDefinition(ConstantDefinition),
    }

    pub enum Expression {
        Assign(Box<AssignExpression>),
        ArrayIndexAccess(Box<ArrayIndexAccessExpression>),
        MemberAccess(Box<MemberAccessExpression>),
        FunctionCall(Box<FunctionCallExpression>),
        PrefixUnary(Box<PrefixUnaryExpression>),
        Parenthesized(Box<ParenthesizedExpression>),
        Binary(Box<BinaryExpression>),
        Literal(Literal),
        Identifier(Identifier),
        Type(Box<Type>),
        Uzumaki(UzumakiExpression),
    }

    pub enum UnaryOperatorKind {
        Neg,
    }

    pub enum OperatorKind {
        Pow,
        Add,
        Sub,
        Mul,
        Div,
        Mod,
        And,
        Or,
        Eq,
        Ne,
        Lt,
        Le,
        Gt,
        Ge,
        BitAnd,
        BitOr,
        BitXor,
        BitNot,
        Shl,
        Shr,
    }

    pub enum Literal {
        Array(ArrayLiteral),
        Bool(BoolLiteral),
        String(StringLiteral),
        Number(NumberLiteral),
        Unit(UnitLiteral),
    }

    pub enum Type {
        Array(Box<TypeArray>),
        Simple(SimpleType),
        Generic(GenericType),
        Function(FunctionType),
        QualifiedName(QualifiedName),
        Qualified(TypeQualifiedName),
        Identifier(Identifier),
    }
}

ast_nodes! {

    pub struct Position {
        pub row: usize,
        pub column: usize,
    }

    pub struct SourceFile {
        pub use_directives: Vec<UseDirective>,
        pub definitions: Vec<Definition>,
    }

    pub struct UseDirective {
        pub imported_types: Option<Vec<Identifier>>,
        pub segments: Option<Vec<Identifier>>,
        pub from: Option<String>,
    }

    pub struct SpecDefinition {
        pub name: Identifier,
        pub definitions: Vec<Definition>,
    }

    pub struct StructDefinition {
        pub name: Identifier,
        pub fields: Vec<StructField>,
        pub methods: Vec<FunctionDefinition>,
    }

    pub struct StructField {
        pub name: Identifier,
        pub type_: Type,
    }

    pub struct EnumDefinition {
        pub name: Identifier,
        pub variants: Vec<Identifier>,
    }

    pub struct Identifier {
        pub name: String,
    }

    pub struct ConstantDefinition {
        pub name: Identifier,
        pub type_: Type,
        pub value: Literal,
    }

    pub struct FunctionDefinition {
        pub name: Identifier,
        pub parameters: Option<Vec<Parameter>>,
        pub returns: Option<Type>,
        pub body: BlockType,
    }

    pub struct ExternalFunctionDefinition {
        pub name: Identifier,
        pub arguments: Option<Vec<Identifier>>,
        pub returns: Option<Type>,
    }

    pub struct TypeDefinition {
        pub name: Identifier,
        pub type_: Type,
    }

    pub struct Parameter {
        pub name: Identifier,
        pub type_: Type,
    }

    pub struct Block {
        pub statements: Vec<Statement>,
    }

    pub struct ExpressionStatement {
        pub expression: Expression,
    }

    pub struct ReturnStatement {
        pub expression: Expression,
    }

    pub struct LoopStatement {
        pub condition: Option<Expression>,
        pub body: BlockType,
    }

    pub struct BreakStatement {}

    pub struct IfStatement {
        pub condition: Expression,
        pub if_arm: BlockType,
        pub else_arm: Option<BlockType>,
    }

    pub struct VariableDefinitionStatement {
        pub name: Identifier,
        pub type_: Type,
        pub value: Option<Expression>,
        pub is_undef: bool,
    }

    pub struct TypeDefinitionStatement {
        pub name: Identifier,
        pub type_: Type,
    }

    pub struct AssignExpression {
        pub left: Box<Expression>,
        pub right: Box<Expression>,
    }

    pub struct ArrayIndexAccessExpression {
        pub array: Box<Expression>,
        pub index: Box<Expression>,
    }

    pub struct MemberAccessExpression {
        pub expression: Box<Expression>,
        pub name: Identifier,
    }

    pub struct FunctionCallExpression {
        pub function: Box<Expression>,
        pub arguments: Option<Vec<(Identifier, Expression)>>,
    }

    pub struct UzumakiExpression {}

    pub struct PrefixUnaryExpression {
        pub expression: Box<Expression>,
        pub operator: UnaryOperatorKind,
    }

    pub struct AssertStatement {
        pub expression: Box<Expression>,
    }

    pub struct ParenthesizedExpression {
        pub expression: Box<Expression>,
    }

    pub struct BinaryExpression {
        pub left: Box<Expression>,
        pub operator: OperatorKind,
        pub right: Box<Expression>,
    }

    pub struct ArrayLiteral {
        pub elements: Vec<Expression>,
    }

    pub struct BoolLiteral {
        pub value: bool,
    }

    pub struct StringLiteral {
        pub value: String,
    }

    pub struct NumberLiteral {
        pub value: String,
        pub type_: Type,
    }

    pub struct UnitLiteral {}

    pub struct SimpleType {
        pub name: String,
    }

    pub struct GenericType {
        pub base: Identifier,
        pub parameters: Vec<Type>,
    }

    pub struct FunctionType {
        pub parameters: Option<Vec<Type>>,
        pub returns: Box<Type>,
    }

    pub struct QualifiedName {
        pub qualifier: Identifier,
        pub name: Identifier,
    }

    pub struct TypeQualifiedName {
        pub alias: Identifier,
        pub name: Identifier,
    }

    pub struct TypeArray {
        pub element_type: Box<Type>,
        pub size: Option<Box<Expression>>,
    }

}
