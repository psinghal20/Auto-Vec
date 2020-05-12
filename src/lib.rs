extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn auto_vec(_args: TokenStream, input: TokenStream) -> TokenStream {
    let scalar = parse_macro_input!(input as syn::ItemFn);
    let ori_scalar = scalar.clone();
    let visibility = &scalar.vis;
    let name = scalar.sig.ident;
    let vec_name = format!("{}_vec", name);
    let vec_ident = syn::Ident::new(&vec_name, name.span());

    let sig_clone_for_error = ori_scalar.sig.clone();
    // Check to ensure the function takes inputs
    if scalar.sig.inputs.len() == 0 {
        return syn::Error::new_spanned(sig_clone_for_error, "Expected one or more arguments, Found None").to_compile_error().into();
    }
    // Check to ensure function has a return type
    if let syn::ReturnType::Default = scalar.sig.output {
        return syn::Error::new_spanned(sig_clone_for_error, "Expected a return type, Found None").to_compile_error().into();
    }

    // Adds clone trait bound to generics to clone values out of vectors
    let clone_trait_token_stream = quote! { std::clone::Clone }.into();
    let clone_trait_ast: syn::TraitBound = parse_macro_input!(clone_trait_token_stream);
    let mut generics = scalar.sig.generics;
    generics.params.iter_mut().for_each(|param| {
        if let syn::GenericParam::Type(ty) = param {
            ty.bounds.push(syn::TypeParamBound::Trait(clone_trait_ast.clone()));
        }
    });

    // Extract inputs from function signature
    let inputs = scalar.sig.inputs.iter().map(|f| {
        if let syn::FnArg::Typed(arg) = f {
            let arg_ident = arg.pat.clone();
            let ty  = &arg.ty;
            return quote!{ #arg_ident: Vec<#ty> };
        } else {
            // return syn::Error::new_spanned(f.clone(), "Expected a typed argument").to_compile_error().into();
            panic!("Expected typed arguments, found untyped self argument in function {}", name);
        }
    });

    // Extract input argument idents from the function signature 
    let input_idents = scalar.sig.inputs.iter().map(|input| {
        if let syn::FnArg::Typed(arg) = input {
            arg.pat.clone()
        } else {
            panic!("Expected typed arguments, found untyped self argument in function {}", name);
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
            quote! { Vec<#ty> }
        }
        _ => {
            panic!("Expected a return type, Found ()");
        }
    };

    // Generated extended method to take vectorized inputs
    let extended_method = quote! {
        #visibility fn #vec_ident#generics(#(#inputs,)*) -> #outputs {
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
