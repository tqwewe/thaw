use crate::{
    components::{Demo, DemoCode},
    pages::MobilePage,
};
use indoc::indoc;
use leptos::*;
use melt_ui::mobile::*;

#[component]
pub fn TabbarPage() -> impl IntoView {
    view! {
        <div style="display: flex">
            <div style="width: 896px; margin: 0 auto;">
                <h1>"Tabbar"</h1>
                <Demo>
                    ""
                    <DemoCode slot>
                        {
                            indoc!(r#"
                            let selected = create_rw_signal(String::from("o"));
                            
                            <Tabbar selected>
                                <TabbarItem name="a">
                                    "and"
                                </TabbarItem>
                                <TabbarItem name="i">
                                    "if"
                                </TabbarItem>
                                <TabbarItem name="o" icon=icondata::AiIcon::AiCloseOutlined>
                                    "or"
                                </TabbarItem>
                            </Tabbar>
                            "#)
                        }
                    </DemoCode>
                </Demo>
            </div>
            <div>
                <MobilePage path="/melt-ui?path=/mobile/nav-bar" />
            </div>
        </div>
    }
}

#[component]
pub fn TabbarDemoPage() -> impl IntoView {
    let selected = create_rw_signal(String::from("o"));
    view! {
        <div style="height: 100vh; background: #f5f5f5">
            { move || selected.get() }
            <Tabbar selected>
                <TabbarItem name="a">
                    "and"
                </TabbarItem>
                <TabbarItem name="i">
                    "if"
                </TabbarItem>
                <TabbarItem name="o" icon=icondata::AiIcon::AiCloseOutlined>
                    "or"
                </TabbarItem>
            </Tabbar>
        </div>
    }
}
