extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr, parse::Parse, punctuated::Punctuated, Token, ItemFn, parse_quote};

struct EnvVars {
    vars: Vec<String>,
}

impl Parse for EnvVars {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let vars_punct: Punctuated<LitStr, Token![,]> = Punctuated::parse_terminated(input)?;
        let vars = vars_punct.iter().map(|lit| lit.value()).collect();
        Ok(EnvVars { vars })
    }
}

#[proc_macro_attribute]
pub fn require_env(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the function
    let mut item_fn = parse_macro_input!(input as ItemFn);
    
    // Parse the attribute arguments as a comma-separated list of string literals
    let env_vars = if args.is_empty() {
        vec![]
    } else {
        match syn::parse::<EnvVars>(args) {
            Ok(env_vars) => env_vars.vars,
            Err(_) => vec![],
        }
    };
    
    // Get the original function body
    let original_body = &item_fn.block;
    
    // Create env var checks
    let env_checks = env_vars.iter().map(|var_name| {
        let var_name_str = var_name.clone();
        quote! {
            if std::env::var(#var_name_str).is_err() {
                println!("Skipping test: Required environment variable '{}' not found", #var_name_str);
                return;
            }
        }
    });
    
    // Replace function body with our checks followed by the original body
    let new_body = parse_quote!({
        #(#env_checks)*
        #original_body
    });
    
    item_fn.block = Box::new(new_body);
    
    // No longer adding the #[test] attribute automatically
    // This allows the macro to be used with other test attributes
    
    TokenStream::from(quote! {
        #item_fn
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    
    // We can't use our own proc macro in the same crate, but here's how it would be used:
    // 
    // #[require_env("TEST_ENV_VAR1", "TEST_ENV_VAR2")]
    // fn test_with_required_env() {
    //     // This test will run only if both TEST_ENV_VAR1 and TEST_ENV_VAR2 are set
    //     println!("Test is running because all required env vars were found!");
    //     let var1 = std::env::var("TEST_ENV_VAR1").unwrap();
    //     let var2 = std::env::var("TEST_ENV_VAR2").unwrap();
    //     println!("TEST_ENV_VAR1 = {}", var1);
    //     println!("TEST_ENV_VAR2 = {}", var2);
    //     assert!(true);
    // }
}
