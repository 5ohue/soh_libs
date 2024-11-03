//-----------------------------------------------------------------------------
use proc_macro::TokenStream;
use quote::quote;
//-----------------------------------------------------------------------------

/// This attribute implements a lot of the generic stuff for vectors:
/// - Derive macros ( Debug, Copy, etc... )
/// - Operator overloads ( add, sub, mul, div )
/// - Convert trait
/// - Some simple math ( dot product, length )
#[proc_macro_attribute]
pub fn impl_vec(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse input
    let input = syn::parse_macro_input!(item as syn::ItemStruct);

    // Get struct info
    let struct_name = &input.ident;
    let float_type = input.generics.params.iter().next().unwrap();

    let syn::Fields::Named(fields) = &input.fields else {
        panic!("Fields must be named!");
    };
    let field = fields
        .named
        .iter()
        .filter_map(|field| field.ident.clone())
        .collect::<Vec<_>>();

    // Info for quote!
    let derives = [
        quote! { Debug },
        quote! { Default },
        quote! { Clone },
        quote! { Copy },
        quote! { Hash },
        quote! { PartialEq },
        quote! { Eq },
        quote! { PartialOrd },
        quote! { Ord },
    ];

    let num_of_fields = field.len();
    let indexes = (0..num_of_fields)
        .map(|idx| syn::Index::from(idx))
        .collect::<Vec<_>>();
    let float_types = (0..num_of_fields)
        .map(|_| float_type.clone())
        .collect::<Vec<_>>();

    let a = quote! {
        #[cfg(feature = "serde")]
        use serde::{Serialize, Deserialize};

        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        #[derive(#(#derives,)*)]
        #input

        // Struct implementation
        impl<#float_type> #struct_name<#float_type>
        where
            #float_type: num_traits::Float,
        {
            // Math functions

            /// Calculate the squared length of the vector (faster than [Self::length])
            pub fn length2(&self) -> #float_type {
                return #(self.#field.powi(2))+*;
            }

            /// Calculate the length of the vector ( for comparisons prefer using [Self::length2] )
            pub fn length(&self) -> #float_type {
                return self.length2().sqrt();
            }

            /// Calculate and return a normalized version of `self`
            pub fn normalized(&self) -> Self {
                return *self / self.length();
            }

            /// Make the length of vector 1.0
            pub fn normalize(&mut self) {
                *self /= self.length();
            }

            /// Calculate the dot product of two vectors
            pub fn dot(vec1: &Self, vec2: &Self) -> #float_type {
                return #(vec1.#field * vec2.#field)+*;
            }

            // Common functions

            /// Apply a mapping functor to coordinates to create a new vector
            pub fn map<Func, U>(&self, mut f: Func) -> #struct_name<U>
            where
                U: num_traits::Float,
                Func: FnMut(#float_type) -> U,
            {
                return #struct_name {
                    #(#field: f(self.#field),)*
                };
            }

            /// Apply a functor on the vector, changing it's coordinates
            pub fn transform<Func>(&mut self, mut f: Func)
            where
                Func: FnMut(&mut #float_type),
            {
                #(f(&mut self.#field);)*
            }
        }

        // Convert
        impl<S, D> crate::Convert<#struct_name<D>> for #struct_name<S>
        where
            S: num_traits::Float,
            D: num_traits::Float,
            D: From<S>,
            {
                fn convert(&self) -> #struct_name<D> {
                    return #struct_name {
                        #(#field: self.#field.into(), )*
                    };
                }
            }

        // Operator overloads
        impl<#float_type> std::ops::Add for #struct_name<#float_type>
        where
            #float_type: num_traits::Float,
        {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                return #struct_name {
                    #(#field: self.#field + rhs.#field, )*
                };
            }
        }
        impl<#float_type> std::ops::AddAssign for #struct_name<#float_type>
        where
            #float_type: num_traits::Float,
        {
            fn add_assign(&mut self, rhs: Self) {
                #(self.#field = self.#field + rhs.#field;)*
            }
        }

        impl<#float_type> std::ops::Sub for #struct_name<#float_type>
        where
            #float_type: num_traits::Float,
        {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                return #struct_name {
                    #(#field: self.#field - rhs.#field, )*
                };
            }
        }
        impl<#float_type> std::ops::SubAssign for #struct_name<#float_type>
        where
            #float_type: num_traits::Float,
        {
            fn sub_assign(&mut self, rhs: Self) {
                #(self.#field = self.#field - rhs.#field;)*
            }
        }

        impl<#float_type> std::ops::Mul<#float_type> for #struct_name<#float_type>
        where
            #float_type: num_traits::Float,
        {
            type Output = Self;

            fn mul(self, rhs: #float_type) -> Self::Output {
                return #struct_name {
                    #(#field: self.#field * rhs, )*
                };
            }
        }
        impl<#float_type> std::ops::MulAssign<#float_type> for #struct_name<#float_type>
        where
            #float_type: num_traits::Float,
        {
            fn mul_assign(&mut self, rhs: #float_type) {
                #(self.#field = self.#field * rhs;)*
            }
        }

        impl<#float_type> std::ops::Div<#float_type> for #struct_name<#float_type>
        where
            #float_type: num_traits::Float,
        {
            type Output = Self;

            fn div(self, rhs: #float_type) -> Self::Output {
                return #struct_name {
                    #(#field: self.#field / rhs, )*
                };
            }
        }
        impl<#float_type> std::ops::DivAssign<#float_type> for #struct_name<#float_type>
        where
            #float_type: num_traits::Float,
        {
            fn div_assign(&mut self, rhs: #float_type) {
                #(self.#field = self.#field / rhs;)*
            }
        }

        // From implementations
        impl<#float_type> FromIterator<#float_type> for #struct_name<#float_type>
        where
            #float_type: Default,
        {
            fn from_iter<I: IntoIterator<Item = #float_type>>(iter: I) -> Self {
                let mut iter = iter.into_iter();

            #(let #field = iter.next().unwrap_or_default();)*

                return #struct_name { #(#field, )* };
            }
        }

        impl<#float_type> From<[#float_type; #num_of_fields]> for #struct_name<#float_type>
        where
            #float_type: Copy,
        {
            fn from(value: [#float_type; #num_of_fields]) -> Self {
                return #struct_name {
                    #(#field: value[#indexes],)*
                };
            }
        }

        impl<#float_type> From<#struct_name<#float_type>> for [#float_type; #num_of_fields]
        where
            #float_type: Copy,
        {
            fn from(value: #struct_name<#float_type>) -> Self {
                return [
                    #(value.#field),*
                ];
            }
        }

        impl<#float_type> From<( #(#float_types),* )> for #struct_name<#float_type>
        where
            #float_type: Copy,
        {
            fn from(value: ( #(#float_types),* )) -> Self {
                return #struct_name {
                    #(#field: value.#indexes,)*
                };
            }
        }

        impl<#float_type> From<#struct_name<#float_type>> for ( #(#float_types),* )
        where
            #float_type: Copy,
        {
            fn from(value: #struct_name<#float_type>) -> Self {
                return (
                    #(value.#field),*
                );
            }
        }
    };

    return TokenStream::from(a);
}

//-----------------------------------------------------------------------------
