use std::collections::HashMap;

use ::ast::typed_ast;
use ast::ast;

pub struct GlobalInfo {
    global: typed_ast::Global,
    arguments: Vec<typed_ast::Type>,
    return_type: typed_ast::Type,
}

enum Error<'src> {
    UnboundedLiteral { literal: ast::Literal<'src> },
}

pub struct ErrorEnvironment<'src> {
    errors: Vec<Error<'src>>,
}

impl<'src> ErrorEnvironment<'src> {
    fn unbounded_literal(&mut self, literal: ast::Literal<'src>) {
        self.errors.push(Error::UnboundedLiteral { literal })
    }
}

pub fn typecheck_function<'src>(
    function: &ast::Function<'src>,
    global_map: &HashMap<ast::Global, GlobalInfo>,
    error_environment: &mut ErrorEnvironment<'src>,
) -> typed_ast::Function {
    let global_info = global_map
        .get(&function.name)
        .expect("internal error, typecheck_function called with insufficient map");
    let mut local_map = HashMap::new();
    let mut type_index = 0;
    let body = function
        .body
        .iter()
        .map(|expression| {
            typecheck_expression(
                expression,
                global_map,
                &mut local_map,
                error_environment,
                &mut type_index,
            )
            .0
        })
        .collect();
    typed_ast::Function {
        name: global_info.global,
        arguments: global_info.arguments.clone(),
        return_type: global_info.return_type,
        body,
    }
}

struct LocalInfo {
    local: typed_ast::Local,
    r#type: typed_ast::Type,
}

fn typecheck_expression<'src>(
    expression: &ast::Expression<'src>,
    global_map: &HashMap<ast::Global, GlobalInfo>,
    local_map: &mut HashMap<ast::Local<'src>, LocalInfo>,
    error_environment: &mut ErrorEnvironment<'src>,
    type_index: &mut usize,
) -> (typed_ast::Expression, typed_ast::Type) {
    match expression {
        ast::Expression::Literal { literal } => {
            let r#type = typed_ast::Type::Temporary { index: *type_index };
            *type_index += 1;
            error_environment.unbounded_literal(*literal);
            (
                typed_ast::Expression::Literal {
                    literal: typed_ast::Literal::UnsignedUnknown(
                        literal.src.parse().expect(
                            "literal must be unsigned integer. This should emit better error",
                        ),
                    ),
                },
                r#type,
            )
        }
        ast::Expression::Assignment {
            new_definition,
            r#type,
            local,
            value,
        } => {
            let (value, inferred_type) =
                typecheck_expression(&value, global_map, local_map, error_environment, type_index);
            let typed_type = local_map
                .get(local)
                .map(|local_info| local_info.r#type)
                .or_else(|| r#type.map(typecheck_type));
            // Constrain type later
            let r#type = typed_type.unwrap_or(inferred_type);
            let typed_local = if *new_definition {
                let local_count = local_map.len();
                let typed_local = typed_ast::Local { index: local_count };
                local_map.insert(
                    *local,
                    LocalInfo {
                        local: typed_local,
                        r#type,
                    },
                );
                typed_local
            } else {
                local_map.get(local).expect("Not a new definition").local
            };
            (
                typed_ast::Expression::Assignment {
                    local: typed_local,
                    value: Box::new(value),
                },
                r#type,
            )
        }
        ast::Expression::Call {
            function,
            arguments,
        } => todo!(),
        ast::Expression::BinaryOperator {
            binary_operator,
            left,
            right,
        } => todo!(),
    }
}

fn typecheck_type(r#type: ast::Type) -> typed_ast::Type {
    match r#type {
        ast::Type::Unsigned32 => typed_ast::Type::Unsigned32,
        ast::Type::Unsigned64 => typed_ast::Type::Unsigned64,
    }
}
