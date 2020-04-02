
use super::util::*;

pub fn generate_result_finish_snippet(result_ident: &syn::Ident, ty: &syn::Type) -> proc_macro2::TokenStream {
    match ty {     
        syn::Type::Reference(_) => {
            panic!("Cannot return reference")
        },
        syn::Type::Path(type_path) => {
            let type_path_segment = type_path.path.segments.last().unwrap();
            generate_result_finish_snippet_for_arg_type(type_path_segment, &quote! { #result_ident })
        },
        syn::Type::Tuple(syn::TypeTuple{elems, ..}) => {
            let mut i = 0;
            let tuple_snippets = elems.iter().map(|t| {
                let tuple_i=syn::Index::from(i);
                let temp_name = format!("tuple_{}", i);
                let temp_ident = syn::Ident::new(temp_name.as_str(), proc_macro2::Span::call_site());
                i = i + 1;
                let snippet = generate_result_finish_snippet(&temp_ident, t);
                quote!{ let #temp_ident = #result_ident.#tuple_i; #snippet }
            });
            quote!{ #(#tuple_snippets)* }
        },
        other_type => panic!("Unsupported return type: {:#?}, not a path", other_type)
    }
}

fn generate_result_finish_snippet_for_arg_type(type_path_segment: &syn::PathSegment, result_expr: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let type_str = type_path_segment.ident.to_string();
    match type_str.as_str() {
        "Result" => {    
            match &type_path_segment.arguments {
                syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments{args, ..}) => {
                    if args.len() != 2 {
                        panic!("Result type must have exactly 2 generic type arguments");
                    }

                    if let (syn::GenericArgument::Type(result_type), syn::GenericArgument::Type(err_type)) =
                           (args.first().unwrap(), args.last().unwrap()) {
                        let ok_res_ident = syn::Ident::new("ok_res", proc_macro2::Span::call_site());
                        let ok_snippet = generate_result_finish_snippet(&ok_res_ident, result_type);
                        let err_res_ident = syn::Ident::new("err_res", proc_macro2::Span::call_site());
                        let err_snippet = generate_result_err_snippet(&err_res_ident, err_type);

                        quote!{
                            match #result_expr {
                                Ok(#ok_res_ident) => {
                                    #ok_snippet
                                },
                                Err(#err_res_ident) => {
                                    #err_snippet
                                }
                            }
                        }                                
                    } else {
                        panic!("Result type arguments must be types")
                    }
                },
                _ => panic!("Result angle brackets expected")
            }
            
        },
        "Address" =>
            quote!{
                self.api.finish_bytes32(#result_expr.as_fixed_bytes());
            },
        "Vec" => {
            let vec_generic_type_segm = vec_generic_arg_type_segment(&type_path_segment);
            let type_str = vec_generic_type_segm.ident.to_string();
            match type_str.as_str() {
                "u8" => 
                    quote!{
                        self.api.finish_vec(& #result_expr);
                    },
                _ => {
                    let elem_finish_snippet = generate_result_finish_snippet_for_arg_type(
                        &vec_generic_type_segm, 
                        &quote! { elem });
                    quote!{
                        for (_, elem) in #result_expr.iter().enumerate() {
                            #elem_finish_snippet
                        }
                    }
                }
            }
        },
        "BigInt" =>
            quote!{
                self.api.finish_big_int(& #result_expr);
            },
        "BigUint" =>
            quote!{
                self.api.finish_big_uint(& #result_expr);
            },
        "i64" =>
            quote!{
                self.api.finish_i64(#result_expr);
            },
        "bool" =>
            quote!{
                self.api.finish_i64( if #result_expr { 1i64 } else { 0i64 });
            },
        other_stype_str => {
            panic!("Unsupported return type: {:?}", other_stype_str)
        }
    }
}

pub fn generate_result_err_snippet(err_ident: &syn::Ident, _ty: &syn::Type) -> proc_macro2::TokenStream {
    quote! {
        let (message_ptr, message_len) = ErrorMessage::message_ptr_and_len(#err_ident);
        self.api.signal_error_raw(message_ptr, message_len);
    }
}

pub fn generate_body_with_result(return_type: &syn::ReturnType, mbody: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    match return_type.clone() {
        syn::ReturnType::Default => quote!{#mbody;},
        syn::ReturnType::Type(_, ty) => {
            let result_ident = syn::Ident::new("result", proc_macro2::Span::call_site());
            let finish = generate_result_finish_snippet(&result_ident, &ty);
            quote!{
                let #result_ident = { #mbody };
                #finish
            }
        },
    }
}