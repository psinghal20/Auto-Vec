extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn auto_vec(_args: TokenStream, input: TokenStream) -> TokenStream {
    let scalar = parse_macro_input!(input as syn::ItemFn);
    let ori_scalar = scalar.clone();
    let name = scalar.sig.ident;
    let vec_name = format!("{}_auto_vec", name);
    let vec_ident = syn::Ident::new(&vec_name, name.span());

    // Check to ensure the function takes inputs
    if scalar.sig.inputs.len() == 0 {
        panic!("Expected one or more arguments, Found None in method {}", name);
    }

    // Extract inputs from function signature
    let inputs = scalar.sig.inputs.iter().map(|f| {
        if let syn::FnArg::Typed(arg) = f {
            let arg_ident = arg.pat.clone();
            if let syn::Type::Path(ref typ) = arg.ty.as_ref() {
                if typ.path.segments.len() != 1 {
                    panic!("Expected a type, found {:?}", typ.path.segments);
                }
                let segment = &typ.path.segments[0];
                return quote! { #arg_ident: Vec<#segment>};
            } else {
                panic!("Expected Path, found {:?}", arg.ty);
            }
        } else {
            panic!("Expected arguments, found {:?}", f);
        }
    });

    // Extract input argument idents from the function signature 
    let input_idents = scalar.sig.inputs.iter().map(|input| {
        if let syn::FnArg::Typed(arg) = input {
            arg.pat.clone()
        } else {
            panic!("Expected arguments, found {:?}", input);
        }
    });

    // Copy of idents used for function call
    let input_idents_for_foo_call = input_idents.clone();
    // Copy of idents to be used for length assertions
    let input_idents_for_len_assertion = input_idents.clone();
    let mut input_idents_for_len_assestion_next = input_idents.clone();
    input_idents_for_len_assestion_next.next();

    // Extract first input ident to be used for for loop
    let first_input_ident = input_idents.clone().into_iter().next();
    
    // Extract output types from function signature
    let outputs = match scalar.sig.output {
        syn::ReturnType::Type(_, ty) => {
            if let syn::Type::Path(ref typ) = ty.as_ref() {
                if typ.path.segments.len() != 1 {
                    panic!("Expected Output type, found {:?}", typ);
                }
                let segment = &typ.path.segments[0];
                quote! { Vec<#segment> }
            } else {
                panic!("Expected output type, Found {:#?}", ty);
            }
        }
        _ => {
            panic!("Unimplemented!");
        }
    };

    // Generated extended method to take vectorized inputs
    let extended_method = quote! {
        pub fn #vec_ident(#(#inputs,)*) -> #outputs {
            #(assert_eq!(#input_idents_for_len_assertion.len(), #input_idents_for_len_assestion_next.len(), "Input vectors of not same length");)*
            let mut result = vec![];
            for i in 0..#first_input_ident.len() {
                result.push(#name(#(#input_idents_for_foo_call[i].clone(),)*));
            }
            return result;
        }
    };

    let result = quote! {
        #ori_scalar

        #extended_method
    };
    result.into()
}
