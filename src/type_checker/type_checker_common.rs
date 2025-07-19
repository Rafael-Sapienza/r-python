use crate::ir::ast::{Function, FormalArgument, Type};

pub fn function_to_type(func: &Function) -> Type {
    let return_type = func.kind.clone();
    let param_types = func.params.iter()
        .map(|arg| arg.argument_type.clone())
        .collect::<Vec<_>>();
    Type::TFunction(Box::new(return_type), param_types)
}

pub fn type_to_function(f_type: &Type) -> Result<Function, String> {
    if let Type::TFunction(boxed_ret_type, param_types) = f_type {
        let kind = (**boxed_ret_type).clone();
        let params = param_types.iter().enumerate().map(|(i, p_type)| {
            FormalArgument {
                argument_name: format!("param{}", i + 1), //param1, param2, ...
                argument_type: p_type.clone(),
            }
        }).collect();
        Ok(Function {
            name: "".to_string(),
            kind,
            params,
            body: None,
        })
    } else {
        Err("Expected Type::TFunction, but got some other Type".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_to_tfunction() {
        let func1 = Function {
            name: "main".to_string(),
            kind: Type::TInteger,
            params: vec![],
            body: None,
        };

        let func2 = Function {
            name: "sum".to_string(),
            kind: Type::TReal,
            params: vec![
                FormalArgument::new("x".to_string(), Type::TInteger),
                FormalArgument::new("y".to_string(), Type::TInteger),
            ],
            body: None,
        };

        let tfunc1 = function_to_type(&func1);
        let tfunc2 = function_to_type(&func2);

        assert_eq!(
            tfunc1,
            Type::TFunction(Box::new(Type::TInteger), vec![])
        );

        assert_eq!(
            tfunc2,
            Type::TFunction(
                Box::new(Type::TReal),
                vec![Type::TInteger, Type::TInteger]
            )
        );
    }

    #[test]
    fn test_tfunction_to_function() {
        let tfunc = Type::TFunction(
            Box::new(Type::TBool),
            vec![Type::TString, Type::TList(Box::new(Type::TReal))]
        );

        let result = type_to_function(&tfunc);
        assert!(result.is_ok());

        let func = result.unwrap();
        assert_eq!(func.kind, Type::TBool);
        assert_eq!(func.params.len(), 2);
        assert_eq!(func.params[0].argument_type, Type::TString);
        assert_eq!(func.params[1].argument_type, Type::TList(Box::new(Type::TReal)));
        assert_eq!(func.params[0].argument_name, "param1");
        assert_eq!(func.params[1].argument_name, "param2");

        // Caso de erro: tipo inválido
        let not_a_function = Type::TList(Box::new(Type::TInteger));
        let error_result = type_to_function(&not_a_function);
        assert!(error_result.is_err());
    }

    #[test]
    fn test_round_trip_conversions() {
        // TFunction → Function → TFunction
        let original_tfunc = Type::TFunction(
            Box::new(Type::TReal),
            vec![Type::TInteger, Type::TBool],
        );

        let func = type_to_function(&original_tfunc).unwrap();
        let tfunc_back = function_to_type(&func);
        assert_eq!(tfunc_back, original_tfunc);

        // Function → TFunction → Function (verificando igualdade parcial)
        let original_func = Function {
            name: "compute".to_string(),
            kind: Type::TMaybe(Box::new(Type::TInteger)),
            params: vec![
                FormalArgument::new("a".to_string(), Type::TInteger),
                FormalArgument::new("b".to_string(), Type::TInteger),
            ],
            body: None,
        };

        let tfunc = function_to_type(&original_func);
        let func_back = type_to_function(&tfunc).unwrap();

        assert_eq!(func_back.kind, original_func.kind);
        assert_eq!(func_back.params.len(), original_func.params.len());

        for (original, converted) in original_func.params.iter().zip(func_back.params.iter()) {
            assert_eq!(original.argument_type, converted.argument_type);
        }
    }
}
