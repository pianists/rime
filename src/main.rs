mod mir;

use mir::{adt::{Adt, Variant}, function::{ACommand, AExpr, ALet, Case, Function, VarId}, program::Program, types::{AdtId, Type, VariantId}};

fn main() {
    let prog = Program {
        adts: vec![
            Adt {
                name: "ABC".to_string(),
                variants: vec![
                    Variant { name: "A".to_string(), fields: vec![] },
                    Variant { name: "B".to_string(), fields: vec![] },
                    Variant { name: "C".to_string(), fields: vec![] },
                ],
            },
        ],
        functions: vec![
            Function {
                name: "rotate".to_string(),
                args: vec![(VarId(0), Type::Adt { id: AdtId(0), type_args: vec![] })],
                ret_type: Type::Adt { id: AdtId(0), type_args: vec![] },
                body: AExpr {
                    lets: vec![
                        ALet {
                            var: VarId(1),
                            command: ACommand::NewAdt {
                                adt: AdtId(0),
                                variant: VariantId(1),
                                args: vec![],
                            },
                        },
                        ALet {
                            var: VarId(2),
                            command: ACommand::NewAdt {
                                adt: AdtId(0),
                                variant: VariantId(2),
                                args: vec![],
                            },
                        },
                        ALet {
                            var: VarId(3),
                            command: ACommand::NewAdt {
                                adt: AdtId(0),
                                variant: VariantId(0),
                                args: vec![],
                            },
                        },
                        ALet {
                            var: VarId(4),
                            command: ACommand::Match {
                                var: VarId(0),
                                cases: vec![
                                    Case {
                                        variant: VariantId(0),
                                        bindings: vec![],
                                        ret: VarId(1),
                                    },
                                    Case {
                                        variant: VariantId(1),
                                        bindings: vec![],
                                        ret: VarId(2),
                                    },
                                    Case {
                                        variant: VariantId(2),
                                        bindings: vec![],
                                        ret: VarId(3),
                                    },
                                ],
                            },
                        },
                    ],
                    ret: VarId(4),
                },
            }
        ],
    };
    println!("Rime is a functional programming language!");
}
