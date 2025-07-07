use clap::Parser;
use color_eyre::Result;
use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use futures::{FutureExt, StreamExt};
use ratatui::{
    DefaultTerminal, Frame,
    widgets::{Paragraph, TableState},
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction, pubkey::Pubkey, signature::{Keypair, Signer}, transaction::Transaction
};
use spl_token::instruction::close_account;

use std::str::FromStr;

#[derive(Parser)]
#[command(name = "solana-burn-cli")]
#[command(about = "A TUI tool for burning Solana tokens and closing ATA accounts")]
pub struct Args {
    /// Private key in base58 format
    #[arg(short, long)]
    private_key: String,

    /// Solana RPC endpoint URL
    #[arg(short, long, default_value = "https://solana-rpc.publicnode.com")]
    rpc_url: String,

    // Whether to burn tokens
    // #[arg(long, default_value = "true")]
    // burn_token: bool,

    // Whether to close ATA accounts
    // #[arg(long, default_value = "true")]
    // close_ata: bool,
}

#[derive(Debug, Clone)]
struct TokenAccountInfo {
    address: Pubkey,
    mint: Pubkey,
    balance: u64,
    ui_balance: String,
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    let terminal = ratatui::init();
    let result = App::new(args).await?.run(terminal).await;
    ratatui::restore();
    result
}

pub struct App {
    /// Is the application running?
    running: bool,
    /// Event stream
    event_stream: EventStream,
    /// Keypair from private key
    keypair: Keypair,
    /// RPC client
    rpc_client: RpcClient,
    /// Configuration
    burn_token: bool,
    close_ata: bool,
    /// All token accounts with selection status
    token_accounts: Vec<(TokenAccountInfo, bool)>, // (account, is_selected)
    /// Filtered token accounts based on search
    filtered_accounts: Vec<(TokenAccountInfo, bool)>,
    /// Table state for the main table
    table_state: TableState,
    /// Search mode state
    search_mode: bool,
    /// Search input string
    search_input: String,
    /// Confirmation dialog state
    show_confirmation: bool,
    /// Number of selected accounts for confirmation
    selected_count_for_confirmation: usize,
}

impl App {
    /// Construct a new instance of [`App`].
    pub async fn new(args: Args) -> Result<Self> {
        // Decode private key
        let private_key_bytes = bs58::decode(&args.private_key)
            .into_vec()
            .map_err(|e| color_eyre::eyre::eyre!("Failed to decode private key: {}", e))?;

        let keypair = Keypair::try_from(&private_key_bytes[..])
            .map_err(|e| color_eyre::eyre::eyre!("Failed to create keypair: {}", e))?;

        // Create RPC client
        let rpc_client = RpcClient::new(args.rpc_url);

        let mut app = Self {
            running: false,
            event_stream: EventStream::new(),
            keypair,
            rpc_client,
            burn_token: true,
            close_ata: true,
            token_accounts: Vec::new(),
            filtered_accounts: Vec::new(),
            table_state: TableState::default(),
            search_mode: false,
            search_input: String::new(),
            show_confirmation: false,
            selected_count_for_confirmation: 0,
        };

        // Load token accounts
        app.load_token_accounts().await?;

        Ok(app)
    }

    /// Load token accounts from Solana RPC
    async fn load_token_accounts(&mut self) -> Result<()> {
        let owner_pubkey = self.keypair.pubkey();

        // Get token accounts by owner
        let accounts = self
            .rpc_client
            .get_token_accounts_by_owner(
                &owner_pubkey,
                solana_client::rpc_request::TokenAccountsFilter::ProgramId(spl_token::id()),
            )
            .map_err(|e| color_eyre::eyre::eyre!("Failed to get token accounts: {}", e))?;
        println!("accounts: {}", accounts.len());

        self.token_accounts.clear();

        for account in accounts {
            // Parse token account data

            let solana_account_decoder::UiAccountData::Json(account_data) = account.account.data
            else {
                panic!("Failed to parse account data");
            };

            let info = account_data
                .parsed
                .get("info")
                .and_then(|info| info.as_object())
                .ok_or_else(|| color_eyre::eyre::eyre!("Failed to parse info"))?;
            let token_amount = info
                .get("tokenAmount")
                .and_then(|ta| ta.as_object())
                .and_then(|ta| ta.get("amount"))
                .and_then(|amount| amount.as_str())
                .and_then(|amount| amount.parse::<u64>().ok())
                .ok_or_else(|| color_eyre::eyre::eyre!("Failed to parse token amount"))?;
            let ui_token_amount = info
                .get("tokenAmount")
                .and_then(|token_amount| token_amount.as_object())
                .and_then(|ta| ta.get("uiAmountString"))
                .and_then(|uas| uas.as_str())
                .ok_or_else(|| color_eyre::eyre::eyre!("Failed to parse ui token amount"))?
                .to_string();
            let mint = info
                .get("mint")
                .and_then(|mint| mint.as_str())
                .ok_or_else(|| color_eyre::eyre::eyre!("Failed to parse mint"))?
                .to_string();
            // TokenAccount size
            self.token_accounts.push((
                TokenAccountInfo {
                    address: Pubkey::from_str(&account.pubkey)
                        .map_err(|e| color_eyre::eyre::eyre!("Failed to parse pubkey: {}", e))?,
                    mint: Pubkey::from_str_const(&mint),
                    balance: token_amount,
                    ui_balance: ui_token_amount,
                },
                false,
            )); // Initially not selected
        }

        // Initialize filtered accounts with all accounts
        self.filtered_accounts = self.token_accounts.clone();

        // Select first item if available
        if !self.filtered_accounts.is_empty() {
            self.table_state.select(Some(0));
        }

        Ok(())
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_crossterm_events().await?;
        }
        Ok(())
    }

    /// Renders the user interface.
    fn draw(&mut self, frame: &mut Frame) {
        use ratatui::layout::{Constraint, Direction, Layout};
        use ratatui::style::{Color, Style};
        use ratatui::widgets::{Block, Borders};

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Main table
                Constraint::Length(3), // Footer with instructions
            ])
            .split(frame.area());

        // Header with pubkey and settings
        let pubkey_str = self.keypair.pubkey().to_string();
        let selected_count = self
            .filtered_accounts
            .iter()
            .filter(|(_, selected)| *selected)
            .count();
        let header_text = if self.search_mode {
            format!(
                "Pubkey: {} | SEARCH MODE: {} | Selected: {}/{} (Total: {})",
                pubkey_str,
                self.search_input,
                selected_count,
                self.filtered_accounts.len(),
                self.token_accounts.len()
            )
        } else {
            format!(
                "Pubkey: {} | Burn Token: {} | Close ATA: {} | Selected: {}/{}",
                pubkey_str,
                self.burn_token,
                self.close_ata,
                selected_count,
                self.filtered_accounts.len()
            )
        };
        let header = Paragraph::new(header_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Solana Burn CLI"),
            )
            .style(Style::default().fg(Color::White));
        frame.render_widget(header, chunks[0]);

        // Main table
        self.draw_main_table(frame, chunks[1]);

        // Footer with instructions
        let footer_text = if self.show_confirmation {
            "Confirmation: Y/Enter Confirm | N/Esc Cancel"
        } else if self.search_mode {
            "Search Mode: Type to filter by mint | Enter/Esc Exit Search | ↑/↓ Navigate | Space Toggle"
        } else {
            "Controls: ↑/↓ Navigate | Space/Enter Toggle | A Select All | C Clear All | F Search | Ctrl+P Process | Q Quit"
        };
        let footer = Paragraph::new(footer_text)
            .block(Block::default().borders(Borders::ALL).title("Controls"))
            .style(Style::default().fg(Color::Yellow));
        frame.render_widget(footer, chunks[2]);

        // Draw confirmation dialog if needed
        if self.show_confirmation {
            self.draw_confirmation_dialog(frame);
        }
    }

    fn draw_main_table(&mut self, frame: &mut Frame, area: ratatui::layout::Rect) {
        use ratatui::style::{Color, Modifier, Style};
        use ratatui::widgets::{Block, Borders, Cell, Row, Table};

        let header = Row::new(vec![
            Cell::from("Selected"),
            Cell::from("Address"),
            Cell::from("Mint"),
            Cell::from("Balance"),
        ])
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

        let rows: Vec<Row> = self
            .filtered_accounts
            .iter()
            .map(|(account, is_selected)| {
                let selected_str = if *is_selected { "✓" } else { " " };
                let address_str = self.format_address(&account.address);
                let mint_str = self.format_address(&account.mint);
                let balance_str = &account.ui_balance;

                Row::new(vec![
                    Cell::from(selected_str),
                    Cell::from(address_str),
                    Cell::from(mint_str),
                    Cell::from(balance_str.clone()),
                ])
            })
            .collect();

        let table = Table::new(
            rows,
            [
                ratatui::layout::Constraint::Length(8),      // Selected column
                ratatui::layout::Constraint::Percentage(30), // Address
                ratatui::layout::Constraint::Percentage(30), // Mint
                ratatui::layout::Constraint::Percentage(40), // Balance
            ],
        )
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Token Accounts ({})", self.filtered_accounts.len()))
                .border_style(Style::default().fg(Color::Green)),
        )
        .row_highlight_style(Style::default().bg(Color::DarkGray));

        frame.render_stateful_widget(table, area, &mut self.table_state);
    }

    /// Format address to show first 6 and last 4 characters with ellipsis
    fn format_address(&self, pubkey: &Pubkey) -> String {
        let addr_str = pubkey.to_string();
        if addr_str.len() > 10 {
            format!("{}...{}", &addr_str[..6], &addr_str[addr_str.len() - 4..])
        } else {
            addr_str
        }
    }

    /// Reads the crossterm events and updates the state of [`App`].
    async fn handle_crossterm_events(&mut self) -> Result<()> {
        tokio::select! {
            event = self.event_stream.next().fuse() => {
                if let Some(Ok(evt)) = event {
                    match evt {
                        Event::Key(key)
                            if key.kind == KeyEventKind::Press
                                => self.on_key_event(key),
                        Event::Mouse(_) => {}
                        Event::Resize(_, _) => {}
                        _ => {}
                    }
                }
            }
            _ = tokio::time::sleep(tokio::time::Duration::from_millis(100)) => {
                // Sleep for a short duration to avoid busy waiting.
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        if self.show_confirmation {
            self.handle_confirmation_keys(key);
        } else if self.search_mode {
            self.handle_search_mode_keys(key);
        } else {
            self.handle_normal_mode_keys(key);
        }
    }

    fn handle_search_mode_keys(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.exit_search_mode();
            }
            KeyCode::Enter => {
                self.exit_search_mode();
            }
            KeyCode::Backspace => {
                self.search_input.pop();
                self.filter_accounts();
            }
            KeyCode::Up => {
                self.previous();
            }
            KeyCode::Down => {
                self.next();
            }
            KeyCode::Char(c) => {
                if c == ' ' {
                    self.toggle_selection();
                } else {
                    self.search_input.push(c);
                    self.filter_accounts();
                }
            }
            _ => {}
        }
    }

    fn handle_normal_mode_keys(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),

            // Arrow keys for navigation
            (_, KeyCode::Up) => {
                self.previous();
            }
            (_, KeyCode::Down) => {
                self.next();
            }

            // Space or Enter to toggle selection
            (_, KeyCode::Enter | KeyCode::Char(' ')) => {
                self.toggle_selection();
            }

            // A to select all
            (_, KeyCode::Char('a') | KeyCode::Char('A')) => {
                self.select_all();
            }

            // C to clear all selections
            (_, KeyCode::Char('c') | KeyCode::Char('C')) => {
                self.clear_all();
            }

            // F to enter search mode
            (_, KeyCode::Char('f') | KeyCode::Char('F')) => {
                self.enter_search_mode();
            }

            // Ctrl+P to show confirmation for processing selected accounts
            (KeyModifiers::CONTROL, KeyCode::Char('p') | KeyCode::Char('P')) => {
                self.show_process_confirmation();
            }

            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }

    fn next(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.filtered_accounts.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.filtered_accounts.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    fn toggle_selection(&mut self) {
        if let Some(i) = self.table_state.selected() {
            if i < self.filtered_accounts.len() {
                let account_address = self.filtered_accounts[i].0.address;
                // Find the account in the original list and toggle it
                if let Some(pos) = self.token_accounts.iter().position(|(acc, _)| acc.address == account_address) {
                    self.token_accounts[pos].1 = !self.token_accounts[pos].1;
                    // Update the filtered list to reflect the change
                    self.filtered_accounts[i].1 = self.token_accounts[pos].1;
                }
            }
        }
    }

    fn select_all(&mut self) {
        for (_, selected) in &mut self.token_accounts {
            *selected = true;
        }
        // Update filtered accounts to reflect changes
        self.sync_filtered_accounts();
    }

    fn clear_all(&mut self) {
        for (_, selected) in &mut self.token_accounts {
            *selected = false;
        }
        // Update filtered accounts to reflect changes
        self.sync_filtered_accounts();
    }

    fn enter_search_mode(&mut self) {
        self.search_mode = true;
        self.search_input.clear();
    }

    fn exit_search_mode(&mut self) {
        self.search_mode = false;
        self.search_input.clear();
        // Reset to show all accounts
        self.filtered_accounts = self.token_accounts.clone();
        // Reset selection to first item
        if !self.filtered_accounts.is_empty() {
            self.table_state.select(Some(0));
        }
    }

    fn filter_accounts(&mut self) {
        if self.search_input.is_empty() {
            self.filtered_accounts = self.token_accounts.clone();
        } else {
            self.filtered_accounts = self.token_accounts
                .iter()
                .filter(|(account, _)| {
                    account.mint.to_string().to_lowercase().contains(&self.search_input.to_lowercase())
                })
                .cloned()
                .collect();
        }
        // Reset selection to first item
        if !self.filtered_accounts.is_empty() {
            self.table_state.select(Some(0));
        } else {
            self.table_state.select(None);
        }
    }

    fn sync_filtered_accounts(&mut self) {
        // Update filtered accounts to reflect selection changes in token_accounts
        for (filtered_account, filtered_selected) in &mut self.filtered_accounts {
            if let Some((_, selected)) = self.token_accounts.iter().find(|(acc, _)| acc.address == filtered_account.address) {
                *filtered_selected = *selected;
            }
        }
    }

    fn show_process_confirmation(&mut self) {
        let selected_count = self
            .token_accounts
            .iter()
            .filter(|(_, selected)| *selected)
            .count();

        if selected_count > 0 {
            self.selected_count_for_confirmation = selected_count;
            self.show_confirmation = true;
        }
    }

    fn handle_confirmation_keys(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                self.show_confirmation = false;
                self.process_selected();
            }
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                self.show_confirmation = false;
            }
            _ => {}
        }
    }

    fn draw_confirmation_dialog(&mut self, frame: &mut Frame) {
        use ratatui::layout::{Alignment, Constraint, Direction, Layout};
        use ratatui::style::{Color, Style};
        use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};

        // Calculate the center area for the dialog
        let area = frame.area();
        let dialog_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Length(8),
                Constraint::Percentage(30),
            ])
            .split(area)[1];

        let dialog_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ])
            .split(dialog_area)[1];

        // Clear the area
        frame.render_widget(Clear, dialog_area);

        // Create the confirmation message
        let message = format!(
            "Are you sure you want to process {} selected account(s)?\n\nThis will:\n• Burn all tokens in selected accounts\n• Close the ATA accounts\n• Recover SOL rent\n\nPress Y to confirm, N to cancel",
            self.selected_count_for_confirmation
        );

        let dialog = Paragraph::new(message)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Confirm Processing")
                    .border_style(Style::default().fg(Color::Red))
            )
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        frame.render_widget(dialog, dialog_area);
    }

    fn process_selected(&mut self) {
        let selected_accounts: Vec<_> = self
            .token_accounts
            .iter()
            .filter(|(_, selected)| *selected)
            .map(|(account, _)| account.clone())
            .collect();

        if selected_accounts.is_empty() {
            return;
        }

        let latest_blockhash = self.rpc_client.get_latest_blockhash().unwrap();
        println!(
            "Processing {} selected accounts...",
            selected_accounts.len()
        );
        selected_accounts.chunks(12).for_each(|chunk| {
            let ixs = chunk
                .iter()
                .flat_map(|account| {
                    [self.create_burn_instruction(account)
                        .unwrap_or_else(|e| panic!("Failed to create burn instruction: {e}")),
                     self.create_close_ata_instruction(account)
                        .unwrap_or_else(|e| panic!("Failed to create close ATA instruction: {e}"))]
                })
                .collect::<Vec<_>>();
            let tx = Transaction::new_signed_with_payer(
                &ixs,
                Some(&self.keypair.pubkey()),
                &[&self.keypair],
                latest_blockhash,
            );
            // let tx = self.rpc_client.simulate_transaction(&tx).unwrap();
            // println!("tx: {:?}", tx);
            self.rpc_client
                .send_and_confirm_transaction_with_spinner(&tx)
                .unwrap_or_else(|e| panic!("Failed to send transaction: {e}"));
        });
    }

    fn create_burn_instruction(&self, account: &TokenAccountInfo) -> anyhow::Result<Instruction> {
        spl_token::instruction::burn(
            &spl_token::id(),
            &account.address,
            &account.mint,
            &self.keypair.pubkey(),
            &[&self.keypair.pubkey()],
            account.balance,
        )
        .map_err(|e| e.into())
    }

    fn create_close_ata_instruction(&self, account: &TokenAccountInfo) -> anyhow::Result<Instruction> {
        close_account(
            &spl_token::id(),
            &account.address,
            &self.keypair.pubkey(),
            &self.keypair.pubkey(),
            &[&self.keypair.pubkey()],
        )
        .map_err(|e| e.into())
    }
}
