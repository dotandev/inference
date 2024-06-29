/*
Expression(ExpressionStatement {
    location: Location {
        start: Position { row: 14, column: 4 },
        end: Position { row: 14, column: 31 }
    },
    expression: Assign(AssignExpression {
        location: Location {
            start: Position { row: 14, column: 4 },
            end: Position { row: 14, column: 30 }
        },
        left: MemberAccess(MemberAccessExpression {
            location: Location {
                start: Position { row: 14, column: 4 },
                end: Position { row: 14, column: 20 }
            },
            expression: Type(Identifier(Identifier {
                location: Location {
                    start: Position { row: 14, column: 4 },
                    end: Position { row: 14, column: 7 }
                },
                name: "ctx"
            })),
            name: Identifier {
                location: Location {
                    start: Position { row: 14, column: 8 },
                    end: Position { row: 14, column: 20 }
                },
                name: "max_curr_mem"
            }
        }),
        right: Binary(BinaryExpression {
            location: Location {
                start: Position { row: 14, column: 23 },
                end: Position { row: 14, column: 30 }
            },
            left: Literal(Number(NumberLiteral {
                location: Location {
                    start: Position { row: 14, column: 23 },
                    end: Position { row: 14, column: 24 }
                },
                value: 2
            })),
            operator: "2 ** 16", //FIXME
            right: Literal(Number(NumberLiteral {
                location: Location {
                    start: Position { row: 14, column: 28 },
                    end: Position { row: 14, column: 30 }
                },
                value: 16
            }))
        })
    })
})
*/

/*
Expression(ExpressionStatement {
    location: Location {
        start: Position { row: 19, column: 4 },
        end: Position { row: 19, column: 44 }
    },
    expression: Assert(AssertExpression {
        location: Location {
            start: Position { row: 19, column: 4 },
            end: Position { row: 19, column: 43 }
        },
        expression: Binary(BinaryExpression {
            location: Location {
                start: Position { row: 19, column: 11 },
                end: Position { row: 19, column: 43 }
            },
            left: Binary(BinaryExpression {
                location: Location {
                    start: Position { row: 19, column: 11 },
                    end: Position { row: 19, column: 31 }
                },
                left: Type(Identifier(Identifier {
                    location: Location {
                        start: Position { row: 19, column: 11 },
                        end: Position { row: 19, column: 12 }
                    },
                    name: "a"
                })),
                operator: "a < ctx.max_curr_mem",//FIXME
                right: MemberAccess(MemberAccessExpression {
                    location: Location {
                        start: Position { row: 19, column: 15 },
                        end: Position { row: 19, column: 31 }
                    },
                    expression: Type(Identifier(Identifier {
                        location: Location {
                            start: Position { row: 19, column: 15 },
                            end: Position { row: 19, column: 18 }
                        },
                        name: "ctx"
                    })),
                    name: Identifier {
                        location: Location {
                            start: Position { row: 19, column: 19 },
                            end: Position { row: 19, column: 31 }
                        },
                        name: "max_curr_mem"
                    }
                })
            }),
            operator: "a < ctx.max_curr_mem && !(a % 4)",//FIXME
            right: PrefixUnary(PrefixUnaryExpression {
                location: Location {
                    start: Position { row: 19, column: 35 },
                    end: Position { row: 19, column: 43 }
                },
                expression: Parenthesized(ParenthesizedExpression {
                    location: Location {
                        start: Position { row: 19, column: 36 },
                        end: Position { row: 19, column: 43 }
                    },
                    expression: Binary(BinaryExpression {
                        location: Location {
                            start: Position { row: 19, column: 37 },
                            end: Position { row: 19, column: 42 }
                        },
                        left: Type(Identifier(Identifier {
                            location: Location {
                                start: Position { row: 19, column: 37 },
                                end: Position { row: 19, column: 38 }
                            },
                            name: "a"
                        })),
                        operator: "a % 4", //FIXME
                        right: Literal(Number(NumberLiteral {
                            location: Location {
                                start: Position { row: 19, column: 41 },
                                end: Position { row: 19, column: 42 }
                            },
                            value: 4
                        }))
                    })
                })
            })
        })
    })
})
*/

/*
Function(FunctionDefinition {
    location: Location {
        start: Position { row: 23, column: 2 },
        end: Position { row: 31, column: 3 }
    },
    name: Identifier {
        location: Location {
            start: Position { row: 23, column: 5 },
            end: Position { row: 23, column: 17 }
        },
        name: "count_values"
    },
    arguments: None, //FIXME
    returns: Some(Simple(SimpleType {
        location: Location {
            start: Position { row: 23, column: 56 },
            end: Position { row: 23, column: 59 }
        },
        name: "u32"
    })),
    body: Block {
        location: Location {
            start: Position { row: 23, column: 60 },
            end: Position { row: 31, column: 3 }
        },
        statements: [
            VariableDefinition(VariableDefinitionStatement {
                location: Location {
                    start: Position { row: 24, column: 4 },
                    end: Position { row: 24, column: 22 }
                },
                name: Identifier {
                    location: Location {
                        start: Position { row: 24, column: 8 },
                        end: Position { row: 24, column: 11 }
                    },
                    name: "res"
                },
                type_: Simple(SimpleType {
                    location: Location {
                        start: Position { row: 24, column: 14 },
                        end: Position { row: 24, column: 17 }
                    },
                    name: "u32"
                }),
                value: None //FIXME
            }),
            For(ForStatement {
                location: Location {
                    start: Position { row: 25, column: 4 },
                    end: Position { row: 29, column: 5 }
                },
                initializer: Some(VariableDefinitionStatement {
                    location: Location {
                        start: Position { row: 25, column: 9 },
                        end: Position { row: 25, column: 29 }
                    },
                    name: Identifier {
                        location: Location {
                            start: Position { row: 25, column: 13 },
                            end: Position { row: 25, column: 14 }
                        },
                        name: "i"
                    },
                    type_: Identifier(Identifier {
                        location: Location {
                            start: Position { row: 25, column: 17 },
                            end: Position { row: 25, column: 24 }
                        },
                        name: "Address"
                    }),
                    value: None //FIXME
                }),
                condition: Some(Binary(BinaryExpression {
                    location: Location {
                        start: Position { row: 25, column: 30 },
                        end: Position { row: 25, column: 36 }
                    },
                    left: Type(Identifier(Identifier {
                        location: Location {
                            start: Position { row: 25, column: 30 },
                            end: Position { row: 25, column: 31 }
                        },
                        name: "i"
                    })),
                    operator: "i <= b",//FIXME
                    right: Type(Identifier(Identifier {
                        location: Location {
                            start: Position { row: 25, column: 35 },
                            end: Position { row: 25, column: 36 }
                        },
                        name: "b"
                    }))
                })),
                update: Some(FunctionCall(FunctionCallExpression {
                    location: Location {
                        start: Position { row: 25, column: 38 },
                        end: Position { row: 25, column: 46 }
                    },
                    function: MemberAccess(MemberAccessExpression {
                        location: Location {
                            start: Position { row: 25, column: 38 },
                            end: Position { row: 25, column: 44 }
                        },
                        expression: Type(Identifier(Identifier {
                            location: Location {
                                start: Position { row: 25, column: 38 },
                                end: Position { row: 25, column: 39 }
                            },
                            name: "i"
                        })),
                        name: Identifier {
                            location: Location {
                                start: Position { row: 25, column: 40 },
                                end: Position { row: 25, column: 44 }
                            },
                            name: "next"
                        }
                    }),
                    arguments: None
                })),
                body: Block(Block {
                    location: Location {
                        start: Position { row: 25, column: 48 },
                        end: Position { row: 29, column: 5 }
                    },
                    statements: [
                        If(IfStatement {
                            location: Location {
                                start: Position { row: 26, column: 6 },
                                end: Position { row: 28, column: 7 }
                            },
                            condition: Parenthesized(ParenthesizedExpression {
                                location: Location {
                                    start: Position { row: 26, column: 9 },
                                    end: Position { row: 26, column: 31 }
                                },
                                expression: Binary(BinaryExpression {
                                    location: Location {
                                        start: Position { row: 26, column: 10 },
                                        end: Position { row: 26, column: 30 }
                                    },
                                    left: FunctionCall(FunctionCallExpression {
                                        location: Location {
                                            start: Position { row: 26, column: 10 },
                                            end: Position { row: 26, column: 23 }
                                        },
                                        function: MemberAccess(MemberAccessExpression {
                                            location: Location {
                                                start: Position { row: 26, column: 10 },
                                                end: Position { row: 26, column: 20 }
                                            },
                                            expression: Type(Qualified(QualifiedType {
                                                location: Location {
                                                    start: Position { row: 26, column: 10 },
                                                    end: Position { row: 26, column: 17 }
                                                },
                                                qualifier: Identifier {
                                                    location: Location {
                                                        start: Position { row: 26, column: 10 },
                                                        end: Position { row: 26, column: 13 }
                                                    },
                                                    name: "ctx"
                                                },
                                                name: Identifier {
                                                    location: Location {
                                                        start: Position { row: 26, column: 14 },
                                                        end: Position { row: 26, column: 17 }
                                                    },
                                                    name: "mem"
                                                }
                                            })),
                                            name: Identifier {
                                                location: Location {
                                                    start: Position { row: 26, column: 18 },
                                                    end: Position { row: 26, column: 20 }
                                                },
                                                name: "at"
                                            }
                                        }),
                                        arguments: Some([Type(Identifier(Identifier {
                                            location: Location {
                                                start: Position { row: 26, column: 21 },
                                                end: Position { row: 26, column: 22 }
                                            },
                                            name: "i"
                                        }))])
                                    }),
                                    operator: "ctx.mem.at(i) == val",//FIXME
                                    right: Type(Identifier(Identifier {
                                        location: Location {
                                            start: Position { row: 26, column: 27 },
                                            end: Position { row: 26, column: 30 }
                                        },
                                        name: "val"
                                    }))
                                })
                            }),
                            if_arm: Block {
                                location: Location {
                                    start: Position { row: 26, column: 32 },
                                    end: Position { row: 28, column: 7 }
                                },
                                statements: [
                                    Expression(ExpressionStatement {
                                        location: Location {
                                            start: Position { row: 27, column: 8 },
                                            end: Position { row: 27, column: 22 }
                                        },
                                        expression: Assign(AssignExpression {
                                            location: Location {
                                                start: Position { row: 27, column: 8 },
                                                end: Position { row: 27, column: 21 }
                                            },
                                            left: Type(Identifier(Identifier {
                                                location: Location {
                                                    start: Position { row: 27, column: 8 },
                                                    end: Position { row: 27, column: 11 }
                                                },
                                                name: "res"
                                            })),
                                            right: Binary(BinaryExpression {
                                                location: Location {
                                                    start: Position { row: 27, column: 14 },
                                                    end: Position { row: 27, column: 21 }
                                                },
                                                left: Type(Identifier(Identifier {
                                                    location: Location {
                                                        start: Position { row: 27, column: 14 },
                                                        end: Position { row: 27, column: 17 }
                                                    },
                                                    name: "res"
                                                })),
                                                operator: "res + 1",//FIXME
                                                right: Literal(Number(NumberLiteral {
                                                    location: Location {
                                                        start: Position { row: 27, column: 20 },
                                                        end: Position { row: 27, column: 21 }
                                                    },
                                                    value: 1
                                                }))
                                            })
                                        })
                                    })
                                ]
                            },
                            else_arm: None
                        })
                    ]
                })
            }),
            Return(ReturnStatement {
                location: Location {
                    start: Position { row: 30, column: 4 },
                    end: Position { row: 30, column: 15 }
                },
                expression: Type(Identifier(Identifier {
                    location: Location {
                        start: Position { row: 30, column: 11 },
                        end: Position { row: 30, column: 14 }
                    },
                    name: "res"
                }))
            })
        ]
    }
})
*/

/*
Function(FunctionDefinition {
    location: Location {
        start: Position { row: 33, column: 2 },
        end: Position { row: 50, column: 3 }
    },
    name: Identifier {
        location: Location {
            start: Position { row: 33, column: 11 },
            end: Position { row: 33, column: 27 }
        },
        name: "preserving_count"
    },
    arguments: None, //FIXME
    returns: None,
    body: Block {
        location: Location {
            start: Position { row: 33, column: 39 },
            end: Position { row: 50, column: 3 }
        },
        statements: [
            VariableDefinition(VariableDefinitionStatement {
                location: Location {
                    start: Position { row: 34, column: 4 },
                    end: Position { row: 34, column: 19 }
                },
                name: Identifier {
                    location: Location {
                        start: Position { row: 34, column: 8 },
                        end: Position { row: 34, column: 9 }
                    },
                    name: "a"
                },
                type_: Identifier(Identifier {
                    location: Location {
                        start: Position { row: 34, column: 11 },
                        end: Position { row: 34, column: 18 }
                    },
                    name: "Address"
                }),
                value: None
            }),
            VariableDefinition(VariableDefinitionStatement {
                location: Location {
                    start: Position { row: 35, column: 4 },
                    end: Position { row: 35, column: 19 }
                },
                name: Identifier {
                    location: Location {
                        start: Position { row: 35, column: 8 },
                        end: Position { row: 35, column: 9 }
                    },
                    name: "b"
                },
                type_: Identifier(Identifier {
                    location: Location {
                        start: Position { row: 35, column: 11 },
                        end: Position { row: 35, column: 18 }
                    },
                    name: "Address"
                }),
                value: None
            }),
            Filter(FilterStatement {
                location: Location {
                    start: Position { row: 37, column: 4 },
                    end: Position { row: 41, column: 5 }
                },
                block: Block {
                    location: Location {
                        start: Position { row: 37, column: 11 },
                        end: Position { row: 41, column: 5 }
                    },
                    statements: [
                        Expression(ExpressionStatement {
                            location: Location {
                                start: Position { row: 38, column: 6 },
                                end: Position { row: 38, column: 36 }
                            },
                            expression: Assign(AssignExpression {
                                location: Location {
                                    start: Position { row: 38, column: 6 },
                                    end: Position { row: 38, column: 35 }
                                },
                                left: Type(Identifier(Identifier {
                                    location: Location {
                                        start: Position { row: 38, column: 6 },
                                        end: Position { row: 38, column: 7 }
                                    },
                                    name: "a"
                                })),
                                right: FunctionCall(FunctionCallExpression {
                                    location: Location {
                                        start: Position { row: 38, column: 10 },
                                        end: Position { row: 38, column: 35 }
                                    },
                                    function: Type(Identifier(Identifier {
                                        location: Location {
                                            start: Position { row: 38, column: 10 },
                                            end: Position { row: 38, column: 23 }
                                        },
                                        name: "valid_Address"
                                    })),
                                    arguments: Some([
                                        FunctionCall(FunctionCallExpression {
                                            location: Location {
                                                start: Position { row: 38, column: 24 },
                                                end: Position { row: 38, column: 34 }
                                            },
                                            function: MemberAccess(MemberAccessExpression {
                                                location: Location {
                                                    start: Position { row: 38, column: 24 },
                                                    end: Position { row: 38, column: 32 }
                                                },
                                                expression: Type(Simple(SimpleType {
                                                    location: Location {
                                                        start: Position { row: 38, column: 24 },
                                                        end: Position { row: 38, column: 27 }
                                                    },
                                                    name: "i32"
                                                })),
                                                name: Identifier {
                                                    location: Location {
                                                        start: Position { row: 38, column: 29 },
                                                        end: Position { row: 38, column: 32 }
                                                    },
                                                    name: "all"
                                                }
                                            }),
                                            arguments: None
                                        })
                                    ])
                                })
                            })
                        }),
                        Expression(ExpressionStatement {
                            location: Location {
                                start: Position { row: 39, column: 6 },
                                end: Position { row: 39, column: 36 }
                            },
                            expression: Assign(AssignExpression {
                                location: Location {
                                    start: Position { row: 39, column: 6 },
                                    end: Position { row: 39, column: 35 }
                                },
                                left: Type(Identifier(Identifier {
                                    location: Location {
                                        start: Position { row: 39, column: 6 },
                                        end: Position { row: 39, column: 7 }
                                    },
                                    name: "b"
                                })),
                                right: FunctionCall(FunctionCallExpression {
                                    location: Location {
                                        start: Position { row: 39, column: 10 },
                                        end: Position { row: 39, column: 35 }
                                    },
                                    function: Type(Identifier(Identifier {
                                        location: Location {
                                            start: Position { row: 39, column: 10 },
                                            end: Position { row: 39, column: 23 }
                                        },
                                        name: "valid_Address"
                                    })),
                                    arguments: Some([
                                        FunctionCall(FunctionCallExpression {
                                            location: Location {
                                                start: Position { row: 39, column: 24 },
                                                end: Position { row: 39, column: 34 }
                                            },
                                            function: MemberAccess(MemberAccessExpression {
                                                location: Location {
                                                    start: Position { row: 39, column: 24 },
                                                    end: Position { row: 39, column: 32 }
                                                },
                                                expression: Type(Simple(SimpleType {
                                                    location: Location {
                                                        start: Position { row: 39, column: 24 },
                                                        end: Position { row: 39, column: 27 }
                                                    },
                                                    name: "i32"
                                                })),
                                                name: Identifier {
                                                    location: Location {
                                                        start: Position { row: 39, column: 29 },
                                                        end: Position { row: 39, column: 32 }
                                                    },
                                                    name: "all"
                                                }
                                            }),
                                            arguments: None
                                        })
                                    ])
                                })
                            })
                        }),
                        Expression(ExpressionStatement {
                            location: Location {
                                start: Position { row: 40, column: 6 },
                                end: Position { row: 40, column: 20 }
                            },
                            expression: Assert(AssertExpression {
                                location: Location {
                                    start: Position { row: 40, column: 6 },
                                    end: Position { row: 40, column: 19 }
                                },
                                expression: Binary(BinaryExpression {
                                    location: Location {
                                        start: Position { row: 40, column: 13 },
                                        end: Position { row: 40, column: 19 }
                                    },
                                    left: Type(Identifier(Identifier {
                                        location: Location {
                                            start: Position { row: 40, column: 13 },
                                            end: Position { row: 40, column: 14 }
                                        },
                                        name: "a"
                                    })),
                                    operator: "a <= b", //FIXME
                                    right: Type(Identifier(Identifier {
                                        location: Location {
                                            start: Position { row: 40, column: 18 },
                                            end: Position { row: 40, column: 19 }
                                        },
                                        name: "b"
                                    }))
                                })
                            })
                        })
                    ]
                }
            }),
            VariableDefinition(VariableDefinitionStatement {
                location: Location {
                    start: Position { row: 43, column: 4 },
                    end: Position { row: 43, column: 31 }
                },
                name: Identifier {
                    location: Location {
                        start: Position { row: 43, column: 8 },
                        end: Position { row: 43, column: 11 }
                    },
                    name: "val"
                },
                type_: Simple(SimpleType {
                    location: Location {
                        start: Position { row: 43, column: 14 },
                        end: Position { row: 43, column: 17 }
                    },
                    name: "i32"
                }),
                value: None //FIXME
            }),
            VariableDefinition(VariableDefinitionStatement {
                location: Location {
                    start: Position { row: 45, column: 4 },
                    end: Position { row: 45, column: 47 }
                },
                name: Identifier {
                    location: Location {
                        start: Position { row: 45, column: 8 },
                        end: Position { row: 45, column: 14 }
                    },
                    name: "before"
                },
                type_: Simple(SimpleType {
                    location: Location {
                        start: Position { row: 45, column: 17 },
                        end: Position { row: 45, column: 20 }
                    },
                    name: "u32"
                }),
                value: None //FIXME
            }),
            Expression(ExpressionStatement {
                location: Location {
                    start: Position { row: 46, column: 4 },
                    end: Position { row: 46, column: 15 }
                },
                expression: FunctionCall(FunctionCallExpression {
                    location: Location {
                        start: Position { row: 46, column: 4 },
                        end: Position { row: 46, column: 14 }
                    },
                    function: Type(Identifier(Identifier {
                        location: Location {
                            start: Position { row: 46, column: 4 },
                            end: Position { row: 46, column: 8 }
                        },
                        name: "func"
                    })),
                    arguments: Some([
                        Type(Identifier(Identifier {
                            location: Location {
                                start: Position { row: 46, column: 9 },
                                end: Position { row: 46, column: 10 }
                            },
                            name: "a"
                        })),
                        Type(Identifier(Identifier {
                            location: Location {
                                start: Position { row: 46, column: 12 },
                                end: Position { row: 46, column: 13 }
                            },
                            name: "b"
                        }))
                    ])
                })
            }),
            VariableDefinition(VariableDefinitionStatement {
                location: Location {
                    start: Position { row: 47, column: 4 },
                    end: Position { row: 47, column: 46 }
                },
                name: Identifier {
                    location: Location {
                        start: Position { row: 47, column: 8 },
                        end: Position { row: 47, column: 13 }
                    },
                    name: "after"
                },
                type_: Simple(SimpleType {
                    location: Location {
                        start: Position { row: 47, column: 16 },
                        end: Position { row: 47, column: 19 }
                    },
                    name: "u32"
                }),
                value: None //FIXME
            }),
            Expression(ExpressionStatement {
                location: Location {
                    start: Position { row: 49, column: 4 },
                    end: Position { row: 49, column: 27 }
                },
                expression: Assert(AssertExpression {
                    location: Location {
                        start: Position { row: 49, column: 4 },
                        end: Position { row: 49, column: 26 }
                    },
                    expression: Binary(BinaryExpression {
                        location: Location {
                            start: Position { row: 49, column: 11 },
                            end: Position { row: 49, column: 26 }
                        },
                        left: Type(Identifier(Identifier {
                            location: Location {
                                start: Position { row: 49, column: 11 },
                                end: Position { row: 49, column: 17 }
                            },
                            name: "before"
                        })),
                        operator: "before == after", //FIXME
                        right: Type(Identifier(Identifier {
                            location: Location {
                                start: Position { row: 49, column: 21 },
                                end: Position { row: 49, column: 26 }
                            },
                            name: "after"
                        }))
                    })
                })
            })
        ]
    }
})
*/
