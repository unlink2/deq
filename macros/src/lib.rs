extern crate deq_core;
extern crate proc_macro;
extern crate quote;
extern crate syn;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parser, parse_macro_input, DeriveInput};

/// Dervies the transaction code
#[proc_macro_derive(Transaction)]
pub fn transaction_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_transaction_macro(&ast)
}

/// Adds transaction data to a struct
/// must be applied to a struct
/// Transaction data should not be modified by anything but
/// the commit, revert and begin methods
#[proc_macro_attribute]
pub fn transaction_fields(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    fields.named.push(
                        syn::Field::parse_named
                            .parse2(quote! { pub transaction_data: TransactionData<Self> })
                            .unwrap(),
                    );
                }
                _ => (),
            }

            return quote! {
                #ast
            }
            .into();
        }
        _ => panic!("`transactions_fields` must be used with structs"),
    }
}

fn impl_transaction_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Transaction for #name {
            fn begin(&mut self) {
                self.transaction_data.t.push(self.clone());
            }

            fn commit(&mut self) -> Result<(), TransactionError> {
                match self.transaction_data.t.pop() {
                    Some(_) => Ok(()),
                    None => Err(TransactionError::new(TransactionErrorType::TransactionNotStarted))
                }
            }

            fn revert(&mut self) -> Result<(), TransactionError> {
                match self.transaction_data.t.pop() {
                    Some(prev) => {
                        *self = prev;
                        return Ok(());
                    },
                    None => Err(TransactionError::new(TransactionErrorType::TransactionNotStarted))
                }
            }

            fn len(&self) -> usize {
                self.transaction_data.t.len()
            }
        }
    };
    gen.into()
}
