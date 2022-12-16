use std::sync::Arc;

use crate::panel_ui::PanelUI;
use rustc_hash::FxHashMap;
use stardust_xr_molecules::fusion::{
    client::LogicStepInfo,
    items::{
        panel::{PanelItem, PanelItemInitData},
        ItemAcceptor, ItemAcceptorHandler, ItemUIHandler,
    },
    node::NodeType,
    HandlerWrapper,
};

pub struct Orbital {
    panel_items: FxHashMap<String, HandlerWrapper<PanelItem, PanelUI>>,
}
impl Orbital {
    pub fn new() -> Self {
        Orbital {
            panel_items: FxHashMap::default(),
        }
    }

    pub fn logic_step(&mut self, _info: LogicStepInfo) {
        for item in self.panel_items.values() {
            item.lock_wrapped().step();
        }
        // let items = self.panel_items.items();
        // let focus = items
        // 	.iter()
        // 	.map(|(_, wrapper)| (wrapper, wrapper.lock_inner().step()))
        // 	.reduce(|a, b| if a.1 > b.1 { b } else { a });
        // if let Some((focus, _)) = focus {
        // 	self.focused = focus.weak_wrapped();
        // }
    }

    fn add_item(&mut self, uid: &str, item: PanelItem, init_data: PanelItemInitData) {
        let ui = PanelUI::new(init_data, item.alias());
        let handler = item.wrap(ui).unwrap();
        handler.lock_wrapped().mouse.lock_wrapped().panel_item_ui =
            Arc::downgrade(handler.wrapped());
        self.panel_items.insert(uid.to_string(), handler);
    }
    fn remove_item(&mut self, uid: &str) {
        self.panel_items.remove(uid);
    }
}

impl ItemAcceptorHandler<PanelItem> for Flatland {
    fn captured(&mut self, uid: &str, item: PanelItem, init_data: PanelItemInitData) {
        self.add_item(uid, item, init_data);
    }
    fn released(&mut self, uid: &str) {
        self.remove_item(uid);
    }
}
