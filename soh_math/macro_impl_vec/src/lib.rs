//-----------------------------------------------------------------------------
use proc_macro::TokenStream;
use quote::quote;
//-----------------------------------------------------------------------------

/// This attribute implements a lot of the generic stuff for vectors:
/// - Derive macros ( Debug, Copy, etc... )
/// - Operator overloads ( add, sub, mul, div )
/// - Convert trait
/// - Some simple math ( dot product, len )
#[proc_macro_attribute]
pub fn impl_vec(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse input
    let input = syn::parse_macro_input!(item as syn::ItemStruct);

    // Get struct info
    let struct_name = &input.ident;
    let ttype = input.generics.params.iter().next().unwrap();

    let syn::Fields::Named(fields) = &input.fields else {
        panic!("Fields must be named!");
    };
    let field = fields
        .named
        .iter()
        .filter_map(|field| field.ident.clone())
        .collect::<Vec<_>>();

    // Info for quote!
    let num_of_fields = field.len();
    let indexes = (0..num_of_fields).map(syn::Index::from).collect::<Vec<_>>();
    let float_types = (0..num_of_fields)
        .map(|_| ttype.clone())
        .collect::<Vec<_>>();

    // Use hypot for 2D length
    let len_impl = if num_of_fields == 2 {
        quote! {
            pub fn len(&self) -> #ttype {
                return #ttype::hypot(self.x, self.y);
            }
        }
    } else {
        quote! {
            pub fn len(&self) -> #ttype {
                return self.len2().sqrt();
            }
        }
    };

    let a = quote! {
        #[repr(C)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
        #input

        // Struct implementations
        impl<#ttype> #struct_name<#ttype>
        where
            #ttype: Copy,
        {
            /// Constructor
            pub const fn new(#(#field: #ttype),*) -> Self {
                return #struct_name {
                    #(#field,)*
                };
            }

            /// Apply a mapping functor to coordinates to create a new vector
            pub fn map<Func, U>(&self, mut f: Func) -> #struct_name<U>
            where
                Func: FnMut(#ttype) -> U,
            {
                return #struct_name {
                    #(#field: f(self.#field),)*
                };
            }

            /// Apply a functor on the vector, changing it's coordinates
            pub fn transform<Func>(&mut self, mut f: Func)
            where
                Func: FnMut(&mut #ttype),
            {
                #(f(&mut self.#field);)*
            }
        }

        // One, Zero
        impl<#ttype> #struct_name<#ttype>
        where
            #ttype: num_traits::Num + Copy,
        {
            /// Vector with all components equal zero
            pub fn zero() -> Self {
                return #struct_name {
                    #(#field: #ttype::zero()),*
                }
            }

            /// Vector with all components equal one
            pub fn one() -> Self {
                return #struct_name {
                    #(#field: #ttype::one()),*
                }
            }
        }

        // Float, Int impl
        impl<#ttype> #struct_name<#ttype>
        where
            #ttype: num_traits::Num + Copy,
        {
            /// Calculate the squared len of the vector (faster than [Self::len])
            pub fn len2(&self) -> #ttype {
                return #(self.#field * self.#field)+*;
            }

            /// Calculate the dot product of two vectors
            pub fn dot(vec1: &Self, vec2: &Self) -> #ttype {
                return #(vec1.#field * vec2.#field)+*;
            }

            /// Component vise multiplication
            pub fn mul(vec1: &Self, vec2: &Self) -> Self {
                return #struct_name {
                    #(#field: vec1.#field * vec2.#field),*
                };
            }

            /// Component vise division
            pub fn div(vec1: &Self, vec2: &Self) -> Self {
                return #struct_name {
                    #(#field: vec1.#field / vec2.#field),*
                };
            }
        }

        // Float impl
        impl<#ttype> #struct_name<#ttype>
        where
            #ttype: num_traits::Float + std::ops::DivAssign,
        {
            // Math functions

            /// Calculate the len of the vector ( for comparisons prefer using [Self::len2] )
            #len_impl

            /// Calculate and return a normalized version of `self`
            pub fn normalized(&self) -> Self {
                return *self / self.len();
            }

            /// Make the len of vector 1.0
            pub fn normalize(&mut self) {
                *self /= self.len();
            }
        }

        // Convert
        impl<S, D> crate::Convert<#struct_name<D>> for #struct_name<S>
        where
            S: Copy,
            D: Copy + From<S>,
        {
            fn convert(&self) -> #struct_name<D> {
                return #struct_name {
                    #(#field: self.#field.into(), )*
                };
            }
        }

        // Operator overloads
        impl<#ttype> std::ops::Add for #struct_name<#ttype>
        where
            #ttype: num_traits::Num,
        {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                return #struct_name {
                    #(#field: self.#field + rhs.#field, )*
                };
            }
        }
        impl<#ttype> std::ops::AddAssign for #struct_name<#ttype>
        where
            #ttype: std::ops::AddAssign,
        {
            fn add_assign(&mut self, rhs: Self) {
                #(self.#field += rhs.#field;)*
            }
        }

        impl<#ttype> std::ops::Sub for #struct_name<#ttype>
        where
            #ttype: num_traits::Num,
        {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                return #struct_name {
                    #(#field: self.#field - rhs.#field, )*
                };
            }
        }
        impl<#ttype> std::ops::SubAssign for #struct_name<#ttype>
        where
            #ttype: std::ops::SubAssign,
        {
            fn sub_assign(&mut self, rhs: Self) {
                #(self.#field -= rhs.#field;)*
            }
        }

        impl<#ttype> std::ops::Mul<#ttype> for #struct_name<#ttype>
        where
            #ttype: num_traits::Num + Copy,
        {
            type Output = Self;

            fn mul(self, rhs: #ttype) -> Self::Output {
                return #struct_name {
                    #(#field: self.#field * rhs, )*
                };
            }
        }
        impl<#ttype> std::ops::MulAssign<#ttype> for #struct_name<#ttype>
        where
            #ttype: std::ops::MulAssign<#ttype> + Copy,
        {
            fn mul_assign(&mut self, rhs: #ttype) {
                #(self.#field *= rhs;)*
            }
        }

        impl<#ttype> std::ops::Div<#ttype> for #struct_name<#ttype>
        where
            #ttype: num_traits::Num + Copy,
        {
            type Output = Self;

            fn div(self, rhs: #ttype) -> Self::Output {
                return #struct_name {
                    #(#field: self.#field / rhs, )*
                };
            }
        }
        impl<#ttype> std::ops::DivAssign<#ttype> for #struct_name<#ttype>
        where
            #ttype: std::ops::DivAssign<#ttype> + Copy,
        {
            fn div_assign(&mut self, rhs: #ttype) {
                #( self.#field /= rhs; )*
            }
        }

        impl<#ttype> std::ops::Neg for #struct_name<#ttype>
        where
            T: std::ops::Neg<Output = T>,
        {
            type Output = Self;

            fn neg(self) -> Self::Output {
                return #struct_name {
                    #( #field: -self.#field, )*
                }
            }
        }

        // From implementations
        impl<#ttype> FromIterator<#ttype> for #struct_name<#ttype>
        where
            #ttype: Default,
        {
            fn from_iter<I: IntoIterator<Item = #ttype>>(iter: I) -> Self {
                let mut iter = iter.into_iter();

            #(let #field = iter.next().unwrap_or_default();)*

                return #struct_name { #(#field, )* };
            }
        }

        impl<#ttype> From<[#ttype; #num_of_fields]> for #struct_name<#ttype>
        where
            #ttype: Copy,
        {
            fn from(value: [#ttype; #num_of_fields]) -> Self {
                return #struct_name {
                    #(#field: value[#indexes],)*
                };
            }
        }

        impl<#ttype> From<#struct_name<#ttype>> for [#ttype; #num_of_fields]
        where
            #ttype: Copy,
        {
            fn from(value: #struct_name<#ttype>) -> Self {
                return [
                    #(value.#field),*
                ];
            }
        }

        impl<#ttype> From<( #(#float_types),* )> for #struct_name<#ttype>
        where
            #ttype: Copy,
        {
            fn from(value: ( #(#float_types),* )) -> Self {
                return #struct_name {
                    #(#field: value.#indexes,)*
                };
            }
        }

        impl<#ttype> From<#struct_name<#ttype>> for ( #(#float_types),* )
        where
            #ttype: Copy,
        {
            fn from(value: #struct_name<#ttype>) -> Self {
                return (
                    #(value.#field),*
                );
            }
        }
    };

    TokenStream::from(a)
}

//-----------------------------------------------------------------------------
