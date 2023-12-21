use proc_macro2::TokenStream;
use std::fmt::Write;
use syn::{parse2, DeriveInput};

use leptos::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <h2>"Syn Visualizer"</h2>
        <ControlledComponent/>
    }
}

fn get_ast_value(input: TokenStream) -> Result<DeriveInput, syn::Error> {
    parse2::<DeriveInput>(input)
}

#[component]
fn ControlledComponent() -> impl IntoView {
    let (name, set_name) = create_signal("struct Testing{name: usize,}".to_string());
    let get_ast = move || match name().parse::<TokenStream>() {
        Ok(name) => match get_ast_value(name) {
            Ok(tree) => {
                let mut temp = String::new();
                write!(&mut temp, "{tree:#?}").unwrap();
                leptos::logging::log!("{}",temp);
                temp
            }

            Err(_) => return "Unable to conver the input to a Abstract syntax tree".to_string(),
        },
        Err(_) => return "Unable to convert input into a token Stream".to_string(),
    };

    view! {
        <textarea type="text"
            on:input=move |ev| {
                set_name(event_target_value(&ev));
            }
            prop:cols=100
            prop:rows=30
        >{name}</textarea>
        <p>{get_ast}</p>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
