# New Features Guide

## Overview

This update includes two important new features:
1. **Search Functionality**: Quickly filter and find specific Token accounts by Mint address
2. **Safety Confirmation**: Confirmation dialog before processing operations to improve safety

## Search Functionality

### How to Use

#### 1. Enter Search Mode
- Press `F` key in the main interface to enter search mode
- The top of the interface will show "SEARCH MODE" prompt
- The bottom control bar will display search mode operation instructions

#### 2. Input Search Criteria
- Directly type any part of the Mint address
- Search is real-time, results update immediately as you type
- Search is case-insensitive
- Supports partial matching (e.g., typing "abc" will match all Mint addresses containing "abc")

#### 3. Operations in Search Results
- Use `↑/↓` keys to navigate in filtered results
- Press `Space` key to select/deselect current account
- Selection status syncs with the original account list

#### 4. Exit Search Mode
- Press `Enter` or `Esc` key to exit search mode
- After exiting, all accounts will be displayed
- Previous selection status is preserved

### Interface Changes

#### Interface in Search Mode
- **Top Info Bar**: Shows "SEARCH MODE: [search content]" and filtered result statistics
- **Main Table**: Only displays accounts matching search criteria
- **Bottom Control Bar**: Shows search mode specific operation instructions

#### Search Statistics
- Display format: `Selected: X/Y (Total: Z)`
  - X: Currently selected account count
  - Y: Filtered displayed account count
  - Z: Total account count

### Use Cases

1. **Find Specific Token**: Quickly locate when you know a Token's Mint address
2. **Batch Process Similar Tokens**: Filter related Tokens by common parts of Mint addresses
3. **Manage Large Account Lists**: Quickly find target accounts when you have many Token accounts

## Safety Confirmation Feature

### Overview
To prevent accidental operations, the processing hotkey has been changed from `P` to `Ctrl+P`, and a confirmation dialog is displayed.

### How to Use

#### 1. Trigger Confirmation Dialog
- After selecting accounts to process, press `Ctrl+P` key combination
- System will display a centered confirmation dialog
- Dialog shows the number of accounts to be processed and operation details

#### 2. Confirmation Dialog Content
- Shows the number of selected accounts
- Lists operations to be performed:
  - Burn all tokens in selected accounts
  - Close ATA accounts
  - Recover SOL rent
- Provides confirmation and cancel options

#### 3. Confirm or Cancel
- Press `Y` or `Enter` to confirm and execute operations
- Press `N` or `Esc` to cancel operation and return to main interface

### Safety Features
- Must explicitly confirm to execute dangerous operations
- Clearly displays operation content to be executed
- Can cancel operation at any time
- Prevents accidental keypress causing unintended operations

## Technical Implementation

### Search Functionality
- Search based on Mint address string matching
- Uses `to_lowercase()` for case-insensitive search
- Real-time filtering, no need to press enter to confirm
- Maintains original data integrity, only changes display content

### Confirmation Functionality
- Added confirmation dialog state management
- Implemented modal dialog UI with centered layout
- Added keyboard event handling for confirmation mode
- Integrated with existing processing workflow

## Notes

### Search Functionality
- Search only targets Mint addresses, not account addresses or balances
- Selection operations in search results affect the original account list
- After exiting search mode, all accounts are redisplayed
- Search input supports backspace key to delete characters

### Confirmation Functionality
- Confirmation dialog only appears when selected account count > 0
- Confirmation dialog blocks other operations until user makes a choice
- Canceling operation does not affect already selected account status
