use crate::app::App;
use crate::model::ultimate_shop::UltimateShop;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph};

pub struct UltimateShopUI {
    pub selected_tab: ShopTab,
    pub ultimate_list_items: Vec<ListItem<'static>>,
    pub ultimate_list_state: ListState,
    pub upgrade_list_items: Vec<ListItem<'static>>,
    pub upgrade_list_state: ListState,
    pub message: Option<String>,
    pub message_timer: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShopTab {
    Ultimates,
    StatUpgrades,
}

impl Default for UltimateShopUI {
    fn default() -> Self {
        let mut ult_state = ListState::default();
        ult_state.select(Some(0));
        let mut upg_state = ListState::default();
        upg_state.select(Some(0));

        Self {
            selected_tab: ShopTab::Ultimates,
            ultimate_list_items: vec![],
            ultimate_list_state: ult_state,
            upgrade_list_items: vec![],
            upgrade_list_state: upg_state,
            message: None,
            message_timer: 0.0,
        }
    }
}

impl UltimateShopUI {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_lists(&mut self, app: &App, shop: &UltimateShop) {
        // Update ultimates list
        let mut ult_items = vec![];
        for ult in &shop.ultimates {
            let status = if app
                .character
                .shop_inventory
                .owns_ultimate(&ult.ultimate_type)
            {
                "✓ OWNED".to_string()
            } else if ult.is_locked(app.floor_level) {
                format!("LOCKED (Lvl {})", ult.unlock_level)
            } else {
                format!("{} gold", ult.cost)
            };
            ult_items.push(ListItem::new(format!(
                "  {} - {} - {}",
                ult.ultimate_type.name(),
                ult.ultimate_type.description(),
                status
            )));
        }
        self.ultimate_list_items = ult_items;
        self.ultimate_list_state.select(Some(0));

        // Update upgrades list
        let mut upg_items = vec![];
        for upg in &shop.stat_upgrades {
            let owned = app
                .character
                .shop_inventory
                .get_upgrade_count(&upg.upgrade_type);
            let status = if upg.is_locked(app.floor_level) {
                format!("LOCKED (Lvl {})", upg.unlock_level)
            } else if upg.max_upgrades > 0 && owned >= upg.max_upgrades {
                "MAX".to_string()
            } else {
                format!("{} gold (Lvl: {})", upg.cost, owned + 1)
            };
            upg_items.push(ListItem::new(format!(
                "  {} - {} - {}",
                upg.upgrade_type.name(),
                upg.upgrade_type.description(),
                status
            )));
        }
        self.upgrade_list_items = upg_items;
        self.upgrade_list_state.select(Some(0));
    }

    pub fn update_lists_from_shop(&mut self, shop: &UltimateShop) {
        // Only repopulate if lists are empty (first time opening shop)
        if !self.ultimate_list_items.is_empty() && !self.upgrade_list_items.is_empty() {
            return; // Lists already populated, don't reset selections
        }

        let mut ult_items = vec![];
        for ult in &shop.ultimates {
            ult_items.push(ListItem::new(format!(
                "  {} - {} - {} gold",
                ult.ultimate_type.name(),
                ult.ultimate_type.description(),
                ult.cost
            )));
        }
        self.ultimate_list_items = ult_items;
        // Only set initial selection if not already set
        if self.ultimate_list_state.selected().is_none() {
            self.ultimate_list_state.select(Some(0));
        }

        let mut upg_items = vec![];
        for upg in &shop.stat_upgrades {
            upg_items.push(ListItem::new(format!(
                "  {} - {} - {} gold",
                upg.upgrade_type.name(),
                upg.upgrade_type.description(),
                upg.cost
            )));
        }
        self.upgrade_list_items = upg_items;
        // Only set initial selection if not already set
        if self.upgrade_list_state.selected().is_none() {
            self.upgrade_list_state.select(Some(0));
        }
    }

    pub fn next(&mut self) {
        match self.selected_tab {
            ShopTab::Ultimates => {
                if let Some(sel) = self.ultimate_list_state.selected() {
                    self.ultimate_list_state
                        .select(Some((sel + 1) % self.ultimate_list_items.len().max(1)));
                }
            }
            ShopTab::StatUpgrades => {
                if let Some(sel) = self.upgrade_list_state.selected() {
                    self.upgrade_list_state
                        .select(Some((sel + 1) % self.upgrade_list_items.len().max(1)));
                }
            }
        }
    }

    pub fn previous(&mut self) {
        match self.selected_tab {
            ShopTab::Ultimates => {
                if let Some(sel) = self.ultimate_list_state.selected() {
                    let len = self.ultimate_list_items.len().max(1);
                    self.ultimate_list_state.select(Some((sel + len - 1) % len));
                }
            }
            ShopTab::StatUpgrades => {
                if let Some(sel) = self.upgrade_list_state.selected() {
                    let len = self.upgrade_list_items.len().max(1);
                    self.upgrade_list_state.select(Some((sel + len - 1) % len));
                }
            }
        }
    }

    pub fn switch_tab(&mut self) {
        self.selected_tab = match self.selected_tab {
            ShopTab::Ultimates => ShopTab::StatUpgrades,
            ShopTab::StatUpgrades => ShopTab::Ultimates,
        };
    }

    pub fn reset(&mut self) {
        // Reset the UI state when closing the shop
        self.selected_tab = ShopTab::Ultimates;
        self.ultimate_list_items.clear();
        self.ultimate_list_state.select(None);
        self.upgrade_list_items.clear();
        self.upgrade_list_state.select(None);
        self.message = None;
        self.message_timer = 0.0;
    }

    pub fn update_message_timer(&mut self, delta_time: f32) {
        if let Some(_) = &self.message {
            self.message_timer -= delta_time;
            if self.message_timer <= 0.0 {
                self.message = None;
                self.message_timer = 0.0;
            }
        }
    }

    pub fn show_message(&mut self, msg: String) {
        self.message = Some(msg);
        self.message_timer = 2.0;
    }
}

pub fn draw(
    f: &mut Frame,
    gold: u32,
    floor_level: u32,
    shop: &UltimateShop,
    shop_ui: &mut UltimateShopUI,
    area: Rect,
) {
    // Update lists if needed
    shop_ui.update_lists_from_shop(shop);

    // Draw semi-transparent background
    f.render_widget(Clear, area);

    let block = Block::default()
        .title(" Ultimate Shop ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(&block, area);

    let inner = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };

    // Create layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(inner);

    // Draw header with tabs
    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[0]);

    let ultimate_tab_style = if shop_ui.selected_tab == ShopTab::Ultimates {
        Style::default().fg(Color::Yellow).bold()
    } else {
        Style::default().fg(Color::Gray)
    };

    let upgrade_tab_style = if shop_ui.selected_tab == ShopTab::StatUpgrades {
        Style::default().fg(Color::Yellow).bold()
    } else {
        Style::default().fg(Color::Gray)
    };

    let ultimate_tab = Paragraph::new("Ultimate Abilities [TAB]")
        .style(ultimate_tab_style)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::BOTTOM));

    let upgrade_tab = Paragraph::new("Stat Upgrades [TAB]")
        .style(upgrade_tab_style)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::BOTTOM));

    f.render_widget(ultimate_tab, header_chunks[0]);
    f.render_widget(upgrade_tab, header_chunks[1]);

    // Draw content based on selected tab
    match shop_ui.selected_tab {
        ShopTab::Ultimates => {
            let list = List::new(shop_ui.ultimate_list_items.clone())
                .block(
                    Block::default()
                        .title(" Select Ultimate [↑/↓] [ENTER] ")
                        .borders(Borders::ALL),
                )
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Black).bg(Color::Cyan))
                .highlight_symbol(">> ");

            f.render_stateful_widget(list, chunks[1], &mut shop_ui.ultimate_list_state);
        }
        ShopTab::StatUpgrades => {
            let list = List::new(shop_ui.upgrade_list_items.clone())
                .block(
                    Block::default()
                        .title(" Select Upgrade [↑/↓] [ENTER] ")
                        .borders(Borders::ALL),
                )
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Black).bg(Color::Cyan))
                .highlight_symbol(">> ");

            f.render_stateful_widget(list, chunks[1], &mut shop_ui.upgrade_list_state);
        }
    }

    // Draw footer
    let footer_text = format!(
        "Gold: {} | Level: {} | [Q] Close | [SPACE] Purchase | [TAB] Switch Tab",
        gold, floor_level
    );
    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);

    f.render_widget(footer, chunks[2]);

    // Draw message if present
    if let Some(msg) = &shop_ui.message {
        let msg_para = Paragraph::new(msg.clone())
            .style(Style::default().fg(Color::Green))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));

        let msg_area = Rect {
            x: area.x + 5,
            y: area.y + 5,
            width: area.width.saturating_sub(10),
            height: 3,
        };

        f.render_widget(Clear, msg_area);
        f.render_widget(msg_para, msg_area);
    }
}
