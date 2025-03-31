//-----------------------------------------------------------------------------
use proc_macro::TokenStream;
use quote::quote;
//-----------------------------------------------------------------------------

struct VecData {
    struct_name: syn::Ident,
    ttype: syn::GenericParam,

    num_of_fields: usize,
    field_names: Vec<syn::Ident>,        // Each field identifier
    field_types: Vec<syn::GenericParam>, // Each field type ( T repeated N times )
    field_indexes: Vec<syn::Index>,      // Number index for each field
}

fn get_data(input: &syn::ItemStruct) -> VecData {
    let struct_name = &input.ident;
    let ttype = input.generics.params.iter().next().unwrap();

    let syn::Fields::Named(fields) = &input.fields else {
        panic!("Fields must be named!");
    };

    let num_of_fields = fields.named.len();
    let field_names = fields
        .named
        .iter()
        .filter_map(|field| field.ident.clone())
        .collect::<Vec<_>>();
    let field_indexes = (0..num_of_fields).map(syn::Index::from).collect::<Vec<_>>();
    let field_types = vec![ttype.clone(); num_of_fields];

    return VecData {
        struct_name: struct_name.clone(),
        ttype: ttype.clone(),

        num_of_fields,
        field_names,
        field_types,
        field_indexes,
    };
}

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

    let VecData {
        struct_name,
        ttype,
        num_of_fields,
        field_names,
        field_types,
        field_indexes,
    } = get_data(&input);

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
            pub const fn new(#(#field_names: #ttype),*) -> Self {
                return #struct_name {
                    #(#field_names,)*
                };
            }

            /// Apply a mapping functor to coordinates to create a new vector
            pub fn map<Func, U>(&self, mut f: Func) -> #struct_name<U>
            where
                Func: FnMut(#ttype) -> U,
            {
                return #struct_name {
                    #(#field_names: f(self.#field_names),)*
                };
            }

            /// Apply a functor on the vector, changing it's coordinates
            pub fn transform<Func>(&mut self, mut f: Func)
            where
                Func: FnMut(&mut #ttype),
            {
                #(f(&mut self.#field_names);)*
            }
        }

        //----------------------------------------------------------------------
        // One, Zero
        impl<#ttype> crate::traits::WholeConsts for #struct_name<#ttype>
        where
            #ttype: crate::traits::WholeConsts,
        {
            const ZERO: Self = #struct_name { #(#field_names: #ttype::ZERO),* };
            const ONE:  Self = #struct_name { #(#field_names: #ttype::ONE),* };
            const TWO:  Self = #struct_name { #(#field_names: #ttype::TWO),* };
        }

        impl<#ttype> #struct_name<#ttype>
        where
            #ttype: crate::traits::WholeConsts,
        {
            /// Vector with all components equal zero
            pub const fn zero() -> Self {
                return <Self as crate::traits::WholeConsts>::ZERO;
            }

            /// Vector with all components equal one
            pub const fn one() -> Self {
                return <Self as crate::traits::WholeConsts>::ONE;
            }

            /// Vector with all components equal two
            pub const fn two() -> Self {
                return <Self as crate::traits::WholeConsts>::TWO;
            }
        }

        //---------------------------------------------------------------------
        // Float, Int impl
        impl<#ttype> #struct_name<#ttype>
        where
            #ttype: num_traits::Num + Copy,
        {
            /// Calculate the squared len of the vector (faster than [Self::len])
            pub fn len2(&self) -> #ttype {
                return #(self.#field_names * self.#field_names)+*;
            }

            /// Calculate the dot product of two vectors
            pub fn dot(vec1: &Self, vec2: &Self) -> #ttype {
                return #(vec1.#field_names * vec2.#field_names)+*;
            }

            /// Component vise multiplication
            pub fn mul(vec1: &Self, vec2: &Self) -> Self {
                return #struct_name {
                    #(#field_names: vec1.#field_names * vec2.#field_names),*
                };
            }

            /// Component vise division
            pub fn div(vec1: &Self, vec2: &Self) -> Self {
                return #struct_name {
                    #(#field_names: vec1.#field_names / vec2.#field_names),*
                };
            }
        }

        // Float impl
        impl<#ttype> #struct_name<#ttype>
        where
            #ttype: num_traits::Float,
        {
            /// Calculate the len of the vector ( for comparisons prefer using [Self::len2] )
            #len_impl

            /// Calculate and return a normalized version of `self`
            pub fn normalized(&self) -> Self {
                return *self / self.len();
            }
        }

        impl<#ttype> #struct_name<#ttype>
        where
            #ttype: num_traits::Float + std::ops::DivAssign,
        {
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
                    #(#field_names: self.#field_names.into(), )*
                };
            }
        }

        // Operator implementations
        macro_rules! impl_op {
            ($trait:ident, $fn:ident, $op:tt) => {
                impl<#ttype> std::ops::$trait for #struct_name<#ttype>
                where
                    #ttype: std::ops::$trait<Output = #ttype>,
                {
                    type Output = Self;
                    fn $fn(self, rhs: Self) -> Self::Output {
                        return Self { #(#field_names: self.#field_names $op rhs.#field_names),* };
                    }
                }
            };
        }

        macro_rules! impl_op_assign {
            ($trait:ident, $fn:ident, $op:tt) => {
                impl<#ttype> std::ops::$trait for #struct_name<#ttype>
                where
                    #ttype: std::ops::$trait,
                {
                    fn $fn(&mut self, rhs: Self) {
                        #(self.#field_names $op rhs.#field_names;)*
                    }
                }
            };
        }

        macro_rules! impl_scalar_op {
            ($trait:ident, $fn:ident, $op:tt) => {
                impl<#ttype> std::ops::$trait<#ttype> for #struct_name<#ttype>
                where
                    #ttype: std::ops::$trait<Output = #ttype> + Copy,
                {
                    type Output = Self;
                    fn $fn(self, rhs: #ttype) -> Self::Output {
                        return Self { #(#field_names: self.#field_names $op rhs),* };
                    }
                }
            };
        }

        macro_rules! impl_scalar_op_assign {
            ($trait:ident, $fn:ident, $op:tt) => {
                impl<#ttype> std::ops::$trait<#ttype> for #struct_name<#ttype>
                where
                    #ttype: std::ops::$trait + Copy,
                {
                    fn $fn(&mut self, rhs: #ttype) {
                        #(self.#field_names $op rhs;)*
                    }
                }
            };
        }

        impl_op!(Add, add, +);
        impl_op_assign!(AddAssign, add_assign, +=);

        impl_op!(Sub, sub, -);
        impl_op_assign!(SubAssign, sub_assign, -=);

        impl_scalar_op!(Mul, mul, *);
        impl_scalar_op_assign!(MulAssign, mul_assign, *=);

        impl_scalar_op!(Div, div, /);
        impl_scalar_op_assign!(DivAssign, div_assign, /=);

        impl<#ttype> std::ops::Neg for #struct_name<#ttype>
        where
            T: std::ops::Neg<Output = T>,
        {
            type Output = Self;

            fn neg(self) -> Self::Output {
                return #struct_name {
                    #( #field_names: -self.#field_names, )*
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

            #(let #field_names = iter.next().unwrap_or_default();)*

                return #struct_name { #(#field_names, )* };
            }
        }

        // Cannot have that! Conflicting impl for when `S` and `D` are the same
        //
        // impl<S, D> From<#struct_name<S>> for #struct_name<D>
        // where
        //     S: Copy,
        //     D: Copy + From<S>,
        // {
        //     fn from(value: #struct_name<S>) -> #struct_name<D> {
        //         return #struct_name {
        //             #(#field_names: self.#field_names.into(), )*
        //         };
        //     }
        // }

        impl<#ttype> From<[#ttype; #num_of_fields]> for #struct_name<#ttype>
        where
            #ttype: Copy,
        {
            fn from(value: [#ttype; #num_of_fields]) -> Self {
                return #struct_name {
                    #(#field_names: value[#field_indexes],)*
                };
            }
        }

        impl<#ttype> From<#struct_name<#ttype>> for [#ttype; #num_of_fields]
        where
            #ttype: Copy,
        {
            fn from(value: #struct_name<#ttype>) -> Self {
                return [
                    #(value.#field_names),*
                ];
            }
        }

        impl<#ttype> From<( #(#field_types),* )> for #struct_name<#ttype>
        where
            #ttype: Copy,
        {
            fn from(value: ( #(#field_types),* )) -> Self {
                return #struct_name {
                    #(#field_names: value.#field_indexes,)*
                };
            }
        }

        impl<#ttype> From<#struct_name<#ttype>> for ( #(#field_types),* )
        where
            #ttype: Copy,
        {
            fn from(value: #struct_name<#ttype>) -> Self {
                return (
                    #(value.#field_names),*
                );
            }
        }
    };

    TokenStream::from(a)
}

//-----------------------------------------------------------------------------
