#![allow(dead_code)]

use super::types::{
    ArrayIndexAccessExpression, ArrayLiteral, AssertStatement, AssignExpression, BinaryExpression,
    Block, BlockType, BoolLiteral, BreakStatement, ConstantDefinition, Definition, EnumDefinition,
    Expression, ExpressionStatement, ExternalFunctionDefinition, FunctionCallExpression,
    FunctionDefinition, FunctionType, GenericType, Identifier, IfStatement, Literal, Location,
    LoopStatement, MemberAccessExpression, NumberLiteral, OperatorKind, Parameter,
    ParenthesizedExpression, PrefixUnaryExpression, QualifiedName, ReturnStatement, SimpleType,
    SourceFile, SpecDefinition, Statement, StringLiteral, StructDefinition, StructField, Type,
    TypeArray, TypeDefinition, TypeDefinitionStatement, TypeQualifiedName, UnaryOperatorKind,
    UnitLiteral, UseDirective, UzumakiExpression, VariableDefinitionStatement,
};

fn get_node_id() -> u32 {
    uuid::Uuid::new_v4().as_u128() as u32
}

impl SourceFile {
    pub fn new(location: Location) -> Self {
        SourceFile {
            id: get_node_id(),
            location,
            use_directives: Vec::new(),
            definitions: Vec::new(),
        }
    }

    pub fn add_use_directive(&mut self, use_directive: UseDirective) {
        self.use_directives.push(use_directive);
    }

    pub fn add_definition(&mut self, definition: Definition) {
        self.definitions.push(definition);
    }
}

impl UseDirective {
    pub fn new(
        imported_types: Option<Vec<Identifier>>,
        segments: Option<Vec<Identifier>>,
        from: Option<String>,
        location: Location,
    ) -> Self {
        UseDirective {
            id: get_node_id(),
            location,
            imported_types,
            segments,
            from,
        }
    }
}

impl SpecDefinition {
    pub fn new(name: Identifier, definitions: Vec<Definition>, location: Location) -> Self {
        SpecDefinition {
            id: get_node_id(),
            location,
            name,
            definitions,
        }
    }
}

impl StructDefinition {
    pub fn new(
        name: Identifier,
        fields: Vec<StructField>,
        methods: Vec<FunctionDefinition>,
        location: Location,
    ) -> Self {
        StructDefinition {
            id: get_node_id(),
            location,
            name,
            fields,
            methods,
        }
    }
}

impl StructField {
    pub fn new(name: Identifier, type_: Type, location: Location) -> Self {
        StructField {
            id: get_node_id(),
            location,
            name,
            type_,
        }
    }
}

impl EnumDefinition {
    pub fn new(name: Identifier, variants: Vec<Identifier>, location: Location) -> Self {
        EnumDefinition {
            id: get_node_id(),
            location,
            name,
            variants,
        }
    }
}

impl Identifier {
    pub fn new(name: String, location: Location) -> Self {
        Identifier {
            id: get_node_id(),
            location,
            name,
        }
    }
}

impl Default for Identifier {
    fn default() -> Self {
        Identifier {
            id: get_node_id(),
            location: Location::default(),
            name: String::new(),
        }
    }
}

impl ConstantDefinition {
    pub fn new(name: Identifier, type_: Type, value: Literal, location: Location) -> Self {
        ConstantDefinition {
            id: get_node_id(),
            location,
            name,
            type_,
            value,
        }
    }
}

impl FunctionDefinition {
    pub fn new(
        name: Identifier,
        arguments: Option<Vec<Parameter>>,
        returns: Option<Type>,
        body: BlockType,
        location: Location,
    ) -> Self {
        FunctionDefinition {
            id: get_node_id(),
            location,
            name,
            parameters: arguments,
            returns,
            body,
        }
    }

    pub fn name(&self) -> &str {
        &self.name.name
    }

    pub fn has_parameters(&self) -> bool {
        self.parameters.is_some() && !self.parameters.as_ref().unwrap().is_empty()
    }

    pub fn is_void(&self) -> bool {
        self.returns.is_none()
    }
}

impl ExternalFunctionDefinition {
    pub fn new(
        name: Identifier,
        arguments: Option<Vec<Identifier>>,
        returns: Option<Type>,
        location: Location,
    ) -> Self {
        ExternalFunctionDefinition {
            id: get_node_id(),
            location,
            name,
            arguments,
            returns,
        }
    }
}

impl TypeDefinition {
    pub fn new(name: Identifier, type_: Type, location: Location) -> Self {
        TypeDefinition {
            id: get_node_id(),
            location,
            name,
            type_,
        }
    }
}

impl Parameter {
    pub fn new(location: Location, name: Identifier, type_: Type) -> Self {
        Parameter {
            id: get_node_id(),
            location,
            name,
            type_,
        }
    }

    pub fn name(&self) -> &str {
        &self.name.name
    }
}

impl Block {
    pub fn new(location: Location, statements: Vec<Statement>) -> Self {
        Block {
            id: get_node_id(),
            location,
            statements,
        }
    }

    pub fn add_statement(&mut self, statement: Statement) {
        self.statements.push(statement);
    }
}

impl ExpressionStatement {
    pub fn new(location: Location, expression: Expression) -> Self {
        ExpressionStatement {
            id: get_node_id(),
            location,
            expression,
        }
    }
}

impl ReturnStatement {
    pub fn new(location: Location, expression: Expression) -> Self {
        ReturnStatement {
            id: get_node_id(),
            location,
            expression,
        }
    }
}

impl LoopStatement {
    pub fn new(location: Location, condition: Option<Expression>, body: BlockType) -> Self {
        LoopStatement {
            id: get_node_id(),
            location,
            condition,
            body,
        }
    }
}

impl BreakStatement {
    pub fn new(location: Location) -> Self {
        BreakStatement {
            id: get_node_id(),
            location,
        }
    }
}

impl IfStatement {
    pub fn new(
        location: Location,
        condition: Expression,
        if_arm: BlockType,
        else_arm: Option<BlockType>,
    ) -> Self {
        IfStatement {
            id: get_node_id(),
            location,
            condition,
            if_arm,
            else_arm,
        }
    }
}

impl VariableDefinitionStatement {
    pub fn new(
        location: Location,
        name: Identifier,
        type_: Type,
        value: Option<Expression>,
        is_undef: bool,
    ) -> Self {
        VariableDefinitionStatement {
            id: get_node_id(),
            location,
            name,
            type_,
            value,
            is_undef,
        }
    }

    pub fn name(&self) -> &str {
        &self.name.name
    }
}

impl TypeDefinitionStatement {
    pub fn new(location: Location, name: Identifier, type_: Type) -> Self {
        TypeDefinitionStatement {
            id: get_node_id(),
            location,
            name,
            type_,
        }
    }
}

impl AssignExpression {
    pub fn new(location: Location, left: Box<Expression>, right: Box<Expression>) -> Self {
        AssignExpression {
            id: get_node_id(),
            location,
            left,
            right,
        }
    }
}

impl ArrayIndexAccessExpression {
    pub fn new(location: Location, array: Box<Expression>, index: Box<Expression>) -> Self {
        ArrayIndexAccessExpression {
            id: get_node_id(),
            location,
            array,
            index,
        }
    }
}

impl MemberAccessExpression {
    pub fn new(location: Location, expression: Box<Expression>, name: Identifier) -> Self {
        MemberAccessExpression {
            id: get_node_id(),
            location,
            expression,
            name,
        }
    }
}

impl FunctionCallExpression {
    pub fn new(
        location: Location,
        function: Box<Expression>,
        arguments: Option<Vec<(Identifier, Expression)>>,
    ) -> Self {
        FunctionCallExpression {
            id: get_node_id(),
            location,
            function,
            arguments,
        }
    }
}

impl PrefixUnaryExpression {
    pub fn new(
        location: Location,
        expression: Box<Expression>,
        operator: UnaryOperatorKind,
    ) -> Self {
        PrefixUnaryExpression {
            id: get_node_id(),
            location,
            expression,
            operator,
        }
    }
}

impl UzumakiExpression {
    pub fn new(location: Location) -> Self {
        UzumakiExpression {
            id: get_node_id(),
            location,
        }
    }
}

impl AssertStatement {
    pub fn new(location: Location, expression: Box<Expression>) -> Self {
        AssertStatement {
            id: get_node_id(),
            location,
            expression,
        }
    }
}

impl ParenthesizedExpression {
    pub fn new(location: Location, expression: Box<Expression>) -> Self {
        ParenthesizedExpression {
            id: get_node_id(),
            location,
            expression,
        }
    }
}

impl BinaryExpression {
    pub fn new(
        location: Location,
        left: Box<Expression>,
        operator: OperatorKind,
        right: Box<Expression>,
    ) -> Self {
        BinaryExpression {
            id: get_node_id(),
            location,
            left,
            operator,
            right,
        }
    }
}

impl BoolLiteral {
    pub fn new(location: Location, value: bool) -> Self {
        BoolLiteral {
            id: get_node_id(),
            location,
            value,
        }
    }
}

impl ArrayLiteral {
    pub fn new(location: Location, elements: Vec<Expression>) -> Self {
        ArrayLiteral {
            id: get_node_id(),
            location,
            elements,
        }
    }
}

impl StringLiteral {
    pub fn new(location: Location, value: String) -> Self {
        StringLiteral {
            id: get_node_id(),
            location,
            value,
        }
    }
}

impl NumberLiteral {
    pub fn new(location: Location, value: String, type_: Type) -> Self {
        NumberLiteral {
            id: get_node_id(),
            location,
            value,
            type_,
        }
    }
}

impl UnitLiteral {
    pub fn new(location: Location) -> Self {
        UnitLiteral {
            id: get_node_id(),
            location,
        }
    }
}

impl SimpleType {
    pub fn new(location: Location, name: String) -> Self {
        SimpleType {
            id: get_node_id(),
            location,
            name,
        }
    }
}

impl GenericType {
    pub fn new(location: Location, base: Identifier, parameters: Vec<Type>) -> Self {
        GenericType {
            id: get_node_id(),
            location,
            base,
            parameters,
        }
    }
}

impl FunctionType {
    pub fn new(location: Location, parameters: Option<Vec<Type>>, returns: Box<Type>) -> Self {
        FunctionType {
            id: get_node_id(),
            location,
            parameters,
            returns,
        }
    }
}

impl QualifiedName {
    pub fn new(location: Location, qualifier: Identifier, name: Identifier) -> Self {
        QualifiedName {
            id: get_node_id(),
            location,
            qualifier,
            name,
        }
    }
}

impl TypeQualifiedName {
    pub fn new(location: Location, alias: Identifier, name: Identifier) -> Self {
        TypeQualifiedName {
            id: get_node_id(),
            location,
            alias,
            name,
        }
    }
}

impl TypeArray {
    pub fn new(location: Location, element_type: Box<Type>, size: Option<Box<Expression>>) -> Self {
        TypeArray {
            id: get_node_id(),
            location,
            element_type,
            size,
        }
    }
}
