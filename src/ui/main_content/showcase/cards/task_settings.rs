use gpui_kit::{
    component::{ActiveTheme, Icon, Sizable, switch::Switch, tag::Tag},
    *,
};

use crate::ui::main_content::showcase::card::card;

struct Setting {
    icon: Icon,
    title: &'static str,
    description: &'static str,
    pro: bool,
    enabled: bool,
}

pub struct TaskSettings {
    settings: Vec<Setting>,
}

impl TaskSettings {
    pub fn new() -> Self {
        Self {
            settings: vec![
                Setting {
                    icon: icon!("lock-icon.svg"),
                    title: "Internal Only",
                    description: "Only members can see this task",
                    pro: false,
                    enabled: false,
                },
                Setting {
                    icon: icon!("circle-dashed-icon.svg"),
                    title: "Optional",
                    description: "Participants can skip this task",
                    pro: false,
                    enabled: true,
                },
                Setting {
                    icon: icon!("magic-wand-icon.svg"),
                    title: "Autocomplete",
                    description: "Automatically complete this task based on the object property",
                    pro: true,
                    enabled: false,
                },
                Setting {
                    icon: icon!("sliders-horizontal-icon.svg"),
                    title: "Conditional Visibility",
                    description: "Hide or show this task based on properties for the object",
                    pro: true,
                    enabled: false,
                },
            ],
        }
    }
}

fn pro_tag() -> impl IntoElement {
    Tag::primary().small().rounded_full().child(
        div()
            .flex()
            .items_center()
            .gap_1()
            .child(icon!("star-icon.svg").small())
            .child("Pro Feature"),
    )
}

impl Render for TaskSettings {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let muted = cx.theme().muted_foreground;
        let rows = self.settings.iter().enumerate().map(|(index, setting)| {
            let title = div()
                .flex()
                .items_center()
                .gap_2()
                .child(div().text_base().child(setting.title))
                .children(setting.pro.then(pro_tag));

            div()
                .flex()
                .items_center()
                .gap_3()
                .child(setting.icon.clone().text_color(muted))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .flex()
                        .flex_col()
                        .child(title)
                        .child(div().text_sm().text_color(muted).child(setting.description)),
                )
                .child(
                    // Switch defaults to the theme primary color when checked.
                    Switch::new(("task-setting", index))
                        .checked(setting.enabled)
                        .on_click(cx.listener(move |this, checked: &bool, _, cx| {
                            this.settings[index].enabled = *checked;
                            cx.notify();
                        })),
                )
        });

        card(cx).gap_5().children(rows)
    }
}
