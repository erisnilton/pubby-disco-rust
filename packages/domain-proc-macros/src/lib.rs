use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields};

#[proc_macro_derive(Entity)]
pub fn derive_entity(input: TokenStream) -> TokenStream {
  let mut input = parse_macro_input!(input as DeriveInput);
  let struct_name = &input.ident;
  let fields = match input.data {
    syn::Data::Struct(ref mut data) => match data.fields {
      Fields::Named(ref mut fields) => &fields.named,
      _ => unimplemented!(),
    },
    _ => unimplemented!(),
  };

  let getters = fields.iter().map(|field| {
    let field_name = field.ident.as_ref().unwrap();
    let ty = &field.ty;
    let mut_name = syn::Ident::new(format!("{}_mut", field_name).as_str(), field_name.span());
    quote! {
      pub fn #field_name(&self) -> &#ty {
          &self.#field_name
      }

      pub fn #mut_name(&mut self) -> &mut #ty {
          &mut self.#field_name
      }
    }
  });

  let setters = fields.iter().filter_map(|field| {
    let field_name = field.ident.as_ref().unwrap();

    if matches!(
      field_name.to_string().as_str(),
      "id" | "updated_at" | "created_at"
    ) {
      return None;
    }

    let setter_name = syn::Ident::new(format!("set_{}", field_name).as_str(), field_name.span());

    let ty = &field.ty;

    Some(quote! {
      pub fn #setter_name(&mut self, #field_name: #ty) {
          self.#field_name = #field_name;
          self.updated_at = chrono::Timelike::with_nanosecond(&chrono::Utc::now().naive_utc(),0).unwrap();
      }
    })
  });

  let builder_name = syn::Ident::new(
    format!("{}Builder", struct_name).as_str(),
    struct_name.span(),
  );

  let builder_setters = fields.iter().map(|field| {
    let field_name = field.ident.as_ref().unwrap();
    let ty = &field.ty;
    quote! {
      pub fn #field_name(mut self, #field_name: #ty) -> Self {
          self.entity.#field_name = #field_name;
          self
      }
    }
  });

  let expanded = quote! {
    impl #struct_name {
        #(#getters)*

        #(#setters)*

        pub fn builder() -> #builder_name {
            #builder_name::new()
        }
    }

    pub(crate) struct #builder_name {
        entity: #struct_name
    }

    impl #builder_name {
        pub fn new() -> Self {
            Self {
                entity: #struct_name::default()
            }
        }

        #(#builder_setters)*

        pub fn build(self) -> #struct_name {
            self.entity
        }
    }

    impl From<#builder_name> for #struct_name {
        fn from(builder: #builder_name) -> Self {
            builder.build()
        }
    }

    impl From<#struct_name> for #builder_name {
        fn from(entity: #struct_name) -> Self {
            Self { entity }
        }
    }
  };

  TokenStream::from(expanded)
}
