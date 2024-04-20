use std::hash::Hash;

use leptos::*;
use thaw_components::{Binder, CSSTransition, Follower, FollowerPlacement, FollowerWidth};
use thaw_utils::{class_list, mount_style, OptionalProp};

use crate::{theme::use_theme, SelectLabel, SelectOption, Theme};

#[component]
pub(super) fn RawSelect<T, F>(
    #[prop(optional, into)] options: MaybeSignal<Vec<SelectOption<T>>>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    label: SelectLabel,
    #[prop(optional, into)] is_menu_visible: Signal<bool>,
    #[prop(into)] on_select: Callback<(ev::MouseEvent, SelectOption<T>)>,
    #[prop(into)] show_menu: Callback<()>,
    #[prop(into)] hide_menu: Callback<()>,
    is_selected: F,
) -> impl IntoView
where
    T: Eq + Hash + Clone + 'static,
    F: Fn(&T) -> bool + Copy + 'static,
{
    mount_style("select", include_str!("./select.css"));

    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            let border_color_hover = theme.common.color_primary.clone();
            css_vars.push_str(&format!("--thaw-border-color-hover: {border_color_hover};"));
            css_vars.push_str(&format!(
                "--thaw-background-color: {};",
                theme.select.background_color
            ));
            css_vars.push_str(&format!("--thaw-font-color: {};", theme.select.font_color));
            css_vars.push_str(&format!(
                "--thaw-border-color: {};",
                theme.select.border_color
            ));
        });

        css_vars
    });

    let menu_css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            css_vars.push_str(&format!(
                "--thaw-background-color: {};",
                theme.select.menu_background_color
            ));
            css_vars.push_str(&format!(
                "--thaw-background-color-hover: {};",
                theme.select.menu_background_color_hover
            ));
            css_vars.push_str(&format!("--thaw-font-color: {};", theme.select.font_color));
            css_vars.push_str(&format!(
                "--thaw-font-color-selected: {};",
                theme.common.color_primary
            ));
        });
        css_vars
    });

    let trigger_ref = create_node_ref::<html::Div>();
    let menu_ref = create_node_ref::<html::Div>();

    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        use leptos::wasm_bindgen::__rt::IntoJsResult;
        let listener = window_event_listener(ev::click, move |ev| {
            let el = ev.target();
            let mut el: Option<web_sys::Element> =
                el.into_js_result().map_or(None, |el| Some(el.into()));
            let body = document().body().unwrap();
            while let Some(current_el) = el {
                if current_el == *body {
                    break;
                };
                if current_el == ***menu_ref.get().unwrap()
                    || current_el == ***trigger_ref.get().unwrap()
                {
                    return;
                }
                el = current_el.parent_element();
            }
            hide_menu.call(());
        });
        on_cleanup(move || listener.remove());
    }

    view! {
        <Binder target_ref=trigger_ref>
            <div
                class=class_list!["thaw-select", class.map(|c| move || c.get())]
                ref=trigger_ref
                on:click=move |_| show_menu.call(())
                style=move || css_vars.get()
            >
                {label.children}
            </div>
            <Follower
                slot
                show=is_menu_visible
                placement=FollowerPlacement::BottomStart
                width=FollowerWidth::Target
            >
                <CSSTransition
                    node_ref=menu_ref
                    name="fade-in-scale-up-transition"
                    appear=is_menu_visible.get_untracked()
                    show=is_menu_visible
                    let:display
                >
                    <div
                        class="thaw-select-menu"
                        style=move || {
                            display
                                .get()
                                .map(|d| d.to_string())
                                .unwrap_or_else(|| menu_css_vars.get())
                        }
                        ref=menu_ref
                    >
                        <For
                            each=move || options.get()
                            key=move |item| item.value.clone()
                            children=move |item| {
                                let item = store_value(item);
                                view! {
                                    <div
                                        class="thaw-select-menu__item"
                                        class=(
                                            "thaw-select-menu__item-selected",
                                            move || item.with_value(|item_value| is_selected(&item_value.value)),
                                        )
                                        on:click=move |ev| on_select.call((ev, item.get_value()))
                                    >
                                        {item.get_value().label}
                                    </div>
                                }
                            }
                        />
                    </div>
                </CSSTransition>
            </Follower>
        </Binder>
    }
}