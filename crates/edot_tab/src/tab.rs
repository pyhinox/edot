use std::borrow::Cow;
use bevy::prelude::*;
use bevy_egui::egui::Ui;

#[derive(Default, Debug, Component, Reflect)]
pub struct Tab;

#[derive(Bundle)]
pub struct TabBundle {
    pub name:  Name,
    pub tab:   Tab,
    pub clear_background: ClearBackground,
    pub on_show:   OnShowTab,
    pub on_open:   OnOpenTab,
    pub on_close:  OnCloseTab,
    pub can_open:  CanOpen,
}

pub struct TabBuilder {
    tab: TabBundle,
}

impl TabBuilder {
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self {
            tab: TabBundle {
                name:      Name::new(name),
                tab:       Tab,
                clear_background: ClearBackground::default(),
                on_show:   OnShowTab(|_, _, _| {}),
                on_open:   OnOpenTab(|_, _| {}),
                on_close:  OnCloseTab(|_, _| {}),
                can_open: CanOpen::default(),
            }
        }
    }

    pub fn clear_background(mut self, clear: bool) -> Self {
        self.tab.clear_background = ClearBackground(clear);
        self
    }

    pub fn on_show(mut self, f: ShowFn) -> Self {
        self.tab.on_show = OnShowTab(f);
        self
    }

    pub fn on_open(mut self, f: OpenFn) -> Self {
        self.tab.on_open = OnOpenTab(f);
        self
    }

    pub fn on_close(mut self, f: CloseFn) -> Self {
        self.tab.on_close = OnCloseTab(f);
        self
    }

    pub fn build(self) -> TabBundle {
        self.tab
    }
}

impl Into<TabBundle> for TabBuilder {
    fn into(self) -> TabBundle {
        self.build()
    }
}

#[derive(Debug, Component, Reflect, Deref, DerefMut, Copy, Clone)]
pub struct ClearBackground(pub bool);

impl Default for ClearBackground {
    fn default() -> Self {
        Self(true)
    }
}

pub(crate) type ShowFn = for<'w, 's> fn(tab: Entity, world: &'w mut World, ui: &'s mut Ui);
pub(crate) type OpenFn = for<'w, 's> fn(tab: Entity, world: &'w mut World);
pub(crate) type CloseFn = for<'w, 's> fn(tab: Entity, world: &'w mut World);
#[derive(Debug, Component, Deref, DerefMut, Copy, Clone)]
pub struct OnShowTab(pub ShowFn);
#[derive(Debug, Component, Deref, DerefMut, Copy, Clone)]
pub struct OnOpenTab(pub OpenFn);
#[derive(Debug, Component, Deref, DerefMut, Copy, Clone)]
pub struct OnCloseTab(pub CloseFn);

#[derive(Debug, Component, Deref, DerefMut, Copy, Clone)]
pub struct CanOpen(pub bool);
impl Default for CanOpen {
    fn default() -> Self {
        Self(true)
    }
}