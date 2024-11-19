//-----------------------------------------------------------------------------
use proc_macro::TokenStream;
use quote::quote;
//-----------------------------------------------------------------------------

#[proc_macro_derive(Vertex)]
pub fn vertex_derive(item: TokenStream) -> TokenStream {
    /*
     * Parse input
     */
    let input = syn::parse_macro_input!(item as syn::ItemStruct);

    /*
     * Struct name
     */
    let name = &input.ident;

    /*
     * Get fields (in order!)
     */
    // Get field types
    let field_types = match &input.fields {
        syn::Fields::Named(fields) => fields
            .named
            .iter()
            .map(|field| &field.ty)
            .collect::<Vec<_>>(),
        syn::Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .map(|field| &field.ty)
            .collect::<Vec<_>>(),
        _ => {
            panic!()
        }
    };

    // Get field names
    let fields = match &input.fields {
        syn::Fields::Named(fields) => fields
            .named
            .iter()
            .filter_map(|field| field.ident.clone())
            .collect::<Vec<_>>(),
        syn::Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .filter_map(|field| field.ident.clone())
            .collect::<Vec<_>>(),
        _ => {
            panic!()
        }
    };

    let num_of_fields = fields.len();
    let field_locations = (0..num_of_fields).map(syn::Index::from).collect::<Vec<_>>();

    /*
     * Build the implementation
     */
    let a = quote! {
        impl soh::vk::Vertex for #name {
            fn get_attribute_description() -> Vec<soh::vk::vertex::AttributeDescription> {
                let mut res = Vec::new();

                #(
                    res.push(soh::vk::vertex::AttributeDescription {
                        location: #field_locations,
                        format: <#field_types as soh::vk::vertex::ToFormat>::format(),
                        offset: std::mem::offset_of!(Self, #fields) as u32,
                    });
                )*

                return res;
            }
        }
    };

    TokenStream::from(a)
}

//-----------------------------------------------------------------------------
