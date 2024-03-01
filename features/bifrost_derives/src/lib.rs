use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(EventComponent)]
pub fn event_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_event_macro(&ast)
}
fn impl_event_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl bifrost_ecs::core::event::EventComponent for #name {}
        impl bifrost_ecs::core::component::AsAny for #name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }
    };

    gen.into()
}
