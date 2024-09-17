use bevy::ecs::query::{QueryData, QueryFilter};
use bevy::prelude::*;
use bevy::utils::{EntityHashMap, EntityHashSet};
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContext;
use egui_dock::egui::{Ui, WidgetText};
use egui_dock::{DockState, NodeIndex, SurfaceIndex};
use crate::prelude::*;
use crate::tab_manager::command::BoxedTabCommand;

#[derive(Resource)]
pub struct TabManager {
    commands:   Vec<BoxedTabCommand>,

    dock_state: DockState<Entity>,
    cx_state:   QueryState<Mut<'static, EguiContext>, With<PrimaryWindow>>,
    tab_state:  QueryState<TabQueryData, TabQueryFilter>,
}

impl FromWorld for TabManager {
    fn from_world(world: &mut World) -> Self {
        Self {
            dock_state: DockState::new(vec![]),
            commands:   vec![],
            cx_state:   QueryState::new(world),
            tab_state:  QueryState::new(world),
        }
    }
}

macro_rules! generate_tab_split_fn {
    ($($name:ident),*) => {
        impl TabManager {
            $(
                pub fn $name(&mut self, parent: NodeIndex, fraction: f32, tab_ids: impl IntoIterator<Item=Entity>) -> [NodeIndex;2] {
                    let tabs: Vec<Entity> = tab_ids.into_iter().collect();
                    let nodes = self.dock_state
                        .main_surface_mut()
                        .$name(parent, fraction, tabs.clone());
                    tabs.iter().for_each(|tab| {
                        self.commands.push(Box::new(command::AddTab {
                            tab_id:  *tab,
                            surface: SurfaceIndex::main(),
                            node:    nodes[1],
                        }));
                    });
                    nodes
                }
            )*
        }
    };
}
generate_tab_split_fn!(split_left, split_right, split_above, split_below);
impl TabManager {
    pub fn set_root_tab(&mut self, tab_id: Entity) -> NodeIndex {
        self.dock_state = DockState::new(vec![tab_id]);
        self.commands.push(Box::new(command::AddTab {
            tab_id,
            surface: SurfaceIndex::main(),
            node:    NodeIndex::root(),
        }));
        NodeIndex::root()
    }
    pub fn flush_command(&mut self, world: &mut World) {
        std::mem::take(&mut self.commands)
            .iter()
            .for_each(|command| {
            command.apply(world, self);
        });
    }
    pub fn show(world: &mut World) {
        world.resource_scope(|world, mut manager: Mut<Self>| {
            let manager = &mut *manager;
            let mut cx = match manager.cx_state.get_single(world) {
                Ok(cx) => cx.clone(),
                Err(_err) => return,
            };
            {
                let extract_tabs = manager.extract_tabs(world);
                let dock_state = &mut manager.dock_state;
                let commands = &mut manager.commands;

                let open_tab_count = extract_tabs
                    .values()
                    .filter(|tab| tab.is_opening)
                    .count();
                let can_open_tab_count = extract_tabs
                    .values()
                    .filter(|tab| !tab.is_opening && tab.can_open)
                    .count();

                egui_dock::DockArea::new(dock_state)
                    .show_add_popup(can_open_tab_count > 0)
                    .show_add_buttons(can_open_tab_count > 0)
                    .show_close_buttons(open_tab_count > 1)
                    .show(cx.get_mut(), &mut TabViewer {
                        world,
                        extract_tabs,
                        commands,
                    });
            }
            manager.flush_command(world);
        });
    }

    fn extract_tabs(&mut self, world: &mut World) -> ExtractTabs {
        let open_tabs: EntityHashSet<Entity> = self.dock_state
            .iter_all_tabs()
            .map(|(_, id)| *id)
            .collect();
        self.tab_state
            .iter(world)
            .map(|tab| {
                (
                    tab.entity,
                    ExtractTab {
                        name:             Name::clone(&*tab.name),
                        clear_background: **tab.clear_background,
                        on_show:          **tab.on_show,
                        on_open:          **tab.on_open,
                        on_close:         **tab.on_close,
                        can_open:         **tab.can_open,
                        is_opening:       open_tabs.contains(&tab.entity),
                    }
                )
            })
            .collect()
    }
}

struct ExtractTab {
    name:             Name,
    clear_background: bool,
    on_show:          ShowFn,
    on_open:          OpenFn,
    on_close:         CloseFn,
    can_open:         bool,
    is_opening:       bool,
}
type ExtractTabs = EntityHashMap<Entity, ExtractTab>;

#[derive(QueryData)]
struct TabQueryData {
    entity:           Entity,
    name:             Ref<'static, Name>,
    clear_background: Ref<'static, ClearBackground>,
    on_show:          Ref<'static, OnShowTab>,
    on_open:          Ref<'static, OnOpenTab>,
    on_close:         Ref<'static, OnCloseTab>,
    can_open:         Ref<'static, CanOpen>,
}

#[derive(QueryFilter)]
struct TabQueryFilter(With<Tab>);

struct TabViewer<'w, 's> {
    world:        &'w mut World,
    commands:     &'s mut Vec<BoxedTabCommand>,
    extract_tabs: ExtractTabs,
}
impl<'w, 's> egui_dock::TabViewer for TabViewer<'w, 's> {
    type Tab = Entity;

    fn title(&mut self, id: &mut Self::Tab) -> WidgetText {
        let tab = self.extract_tabs.get(id).unwrap();
        WidgetText::from(tab.name.as_str())
    }

    fn ui(&mut self, ui: &mut Ui, id: &mut Self::Tab) {
        let tab = self.extract_tabs.get(id).unwrap();
        (tab.on_show)(*id, self.world, ui);
    }

    fn on_close(&mut self, tab: &mut Self::Tab) -> bool {
        self.commands.push(Box::new(command::CloseTab(*tab)));
        true
    }

    fn add_popup(&mut self, ui: &mut Ui, surface: SurfaceIndex, node: NodeIndex) {
        ui.set_min_height(40.);
        ui.set_min_width(120.);
        self.extract_tabs.iter().for_each(|(id, tab)| {
            if tab.is_opening || !tab.can_open {
                return;
            }
            if ui.button(tab.name.as_str()).clicked() {
                self.commands.push(Box::new(command::AddTab {
                    tab_id: *id,
                    surface,
                    node,
                }));
            }
        })
    }

    fn clear_background(&self, id: &Self::Tab) -> bool {
        let tab = self.extract_tabs.get(id).unwrap();
        tab.clear_background
    }
}

pub(super) mod command {
    use bevy::prelude::{Entity, World};
    use egui_dock::{NodeIndex, SurfaceIndex};
    use crate::prelude::TabManager;

    pub(super) type BoxedTabCommand = Box<dyn TabCommand + Sync + Send + 'static>;

    pub(super) trait TabCommand {
        fn apply(&self, world: &mut World, manager: &mut TabManager);
    }

    pub(super) struct AddTab {
        pub(super) tab_id:  Entity,
        pub(super) surface: SurfaceIndex,
        pub(super) node:    NodeIndex,
    }
    impl TabCommand for AddTab {
        fn apply(&self, world: &mut World, manager: &mut TabManager) {
            manager.dock_state.set_focused_node_and_surface((self.surface, self.node));
            if None == manager.dock_state.find_tab(&self.tab_id) {
                manager.dock_state.push_to_focused_leaf(self.tab_id);
            }
            let tabs = manager.extract_tabs(world);
            tabs
                .get(&self.tab_id)
                .and_then(|tab| Some((tab.on_open)(self.tab_id, world)));
        }
    }

    pub(super) struct CloseTab(pub(super) Entity);

    impl TabCommand for CloseTab {
        fn apply(&self, world: &mut World, manager: &mut TabManager) {
            let tabs = manager.extract_tabs(world);
            tabs
                .get(&self.0)
                .and_then(|tab| Some((tab.on_close)(self.0, world)));
        }
    }
}