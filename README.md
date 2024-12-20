<div align="center">
  <img src="./StygianSift_logo2.svg" width="1100" alt="StygianSift Logo">
</div>

### If you like StygianSift or want to see more from me, please consider buying me a [Coffee](https://buymeacoffee.com/charon0) ☕

[![Windows Support](https://img.shields.io/badge/Windows-Should%20Work-yellowgreen?logo=windows&logoColor=white "Works on Windows but has limited testing")](docs/windows-support.md)
![Linux Support](https://img.shields.io/badge/Linux-Supported-success?logo=linux&logoColor=white)
![Status](https://img.shields.io/badge/Status-Work%20In%20Progress-yellow)


## Features and Capabilities
## [I Don't Have Time! Summarize it for me!](#-tldr-feature-list)
- [Terminal-based because real programmers dont click](#%EF%B8%8F-terminal-based-because-real-programmers-dont-click)
- [Mouse Support: Because Real Programmers Don't Follow Arbitrary Stereotypes!](#️-mouse-support-because-real-programmers-dont-follow-arbitrary-stereotypes)
- [Paint your files & directories with colors](#-paint-your-files--directories-with-colors)
- [Shortcuts that actually make your life shorter in a good way](#-shortcuts-that-actually-make-your-life-shorter-in-a-good-way)
- [Customizable keybindings bend the very interface to your will](#️-customizable-keybindings-bend-the-very-interface-to-your-will)
- [Fuzzy search summon files with but a whisper of their true name](#-fuzzy-search-summon-files-with-but-a-whisper-of-their-true-name)
- [Navigational tree sorcery](#-navigational-tree-sorcery)
- [A Sexy Interface](#-a-visually-seductive-interface-which-is-not-a-wierd-thing-to-say)
- [BYOE bring your own editor](#-byoe-bring-your-own-editor)
- [Resurrect your files from the dead](#️-the-undo-system-your-safety-net-in-the-void)
- [The Preview pane: peek into files or directories](#️-the-preview-pane-your-window-into-the-souls-of-your-files)
- [Metadata mania](#-metadata-mania)
- [The config file your digital diary](#-the-config-file-your-digital-diary)
- [Crafted by a boatman with more loose screws than the argo](#️-crafted-by-a-boatman-with-more-loose-screws-than-the-argo)
- [Multi-select because why stop at one](#-multi-select-because-why-stop-at-one)
- [Installation](#-installation)

## 📚 Embark on a Slightly Unhinged File Management Adventure

Lost in the labyrinth of your project files? Fear not, for StygianSift is here to guide you 
through the digital underworld. Inspired by ME, Charon, the mythical ferryman, this tool will 
navigate you through the rivers of data and forests of directories with unparalleled ease. 
Created by a slightly unhinged boatman who spent way too much time ferrying souls and 
muttering about directory structures, this isn't your average file manager. It's a fever 
dream of functionality wrapped in a user interface that actually makes sense.

## 🪶 Dependencies: A Tale of Two Libraries
In my eternal quest to keep things simple (unlike my relationship with my oar),
StygianSift relies on just two dependencies

#### Crossterm
Because someone had to tame the terminal chaos

#### Rayon 
A temporary companion for parallel operations,
soon to be replaced by std's native features

Yes, you heard that right, just two! And soon to be one, as rayon prepares for its
journey across the Styx. Who needs a bloated cargo manifest when you have the raw power
of the Rust standard library?

Now, back to our regularly scheduled file management madness...

## 🖥️ Terminal-based Because Real Programmers Dont Click
![terminal_within](https://github.com/user-attachments/assets/44c0800b-80d6-4d5e-8549-67e48cfa049d)

Embrace the command line, mortal! Navigate your files with the wild abandon of the 
caffeinated genius that you are! If you need more, summon the terminal within the selected 
directory to perform more complex commands. With autocompletions and suggestions!
Return to StygianSift when you're done, no breadcrumbs needed. Who needs pointing devices
when your coding-pinkies could bench more than you? Ask yourself, Why point-and-click
when you can type-and-curse?

## 🖱️ Mouse Support: Because Real Programmers Don't Follow Arbitrary Stereotypes!
![context_menu](https://github.com/user-attachments/assets/bd3abe43-575b-4b46-bae9-35fa8b26115a)

Quiet! Turns out gatekeeping input devices is about as logical as you deciding to read this
far. Whether you're a vim wizard who types at the speed of thought or someone who 
appreciates the occasional naughty mouse click, StygianSift says "¿por qué no los dos?"

A decision that's sure to ruffle your neckbeard, I've added full mouse 
support because I am lazy, and sometimes clicking things is super convenient! 
I know, I know, heresy of the highest order. But hear me out:

- You will have a right-click context menu just like any GUI, complete with all your
  favorite file operations (and maybe some you probably didn't think existed)
- Mouse tracking that respect the dimming system, because why use a mouse if you
  miss out on all the bling!
- Click-to-navigate when your other hand is busy!
- Intuitive Multi-select because the mouse likes to pick and choose:
  - Hold Ctrl and click to select the files you want, ignore the icky ones! 
  - Previously selected files stay selected, because losing your careful selection is a pain 
    we wouldn't wish on our worst enemies
  - Drag-select multiple files like you're painting a masterpiece of productivity

Think of this addition as a diplomatic attempt to bring our two worlds together!
We care not if you use a mouse or a keyboard, in this realm, we believe in equal opportunity
file management. Whether you're a keystroke virtuoso or a click connoisseur,
you're welcome here.

Besides, let's be honest - sometimes you're "eating a sandwich" with one hand and need to 
manage files with the other. I know the struggle. StygianSift provides the solution.

Real programmers use whatever tools make them most productive, even if that means 
occasionally betraying the sacred commandments of terminal purism. Your workflow, your rules!

## 🎨 Paint Your Files & Directories with Colors
![image](https://github.com/user-attachments/assets/c2f5eb37-db4c-4706-9385-2a1ed3b8253c)

In a stroke of genius (or possibly heatstroke), I devised a color-coding system that's as 
powerful as it is perplexing:

- Red: "Touch not, lest ye awaken Cerberus!" (Or just corrupt your build, same thing)
- Yellow: "Caution: May contain traces of genius or utter nonsense"
- Blue: "Stable as a three-legged chair, but it'll do"
- Green: "Free to modify, or turn into digital origami, whatever floats your boat"
- (Pun very much intended)

Plot twist: YOU get to decide what these colors mean. It's like playing god with your file 
system, but less smiting and more organizing! These chromatic enchantments go beyond mere 
decoration - they're a full permission and operation control system:
![image](https://github.com/user-attachments/assets/41dd9787-f83e-4e97-81c5-9e0bfc7088d0)

- Define whether files can be deleted, moved, or renamed per color
- Control whether colored files appear in searches
- Colors cascade up directory trees to protect entire structures
- Set custom rules per color for granular control
- Use filters to view only files of specific colors

## 🚀 Shortcuts That Actually Make Your Life Shorter (in a good way)
![image](https://github.com/user-attachments/assets/4c3375cc-c574-46e2-a74c-041f778bac64)

- Basic shortcuts (0-9) for your most-used directories
- Shift + numbers for setting new shortcuts
- F1-F10 for accessing shortcut layers (Up to a 100 unique shortcuts!)
- Name or rename the shortcut layers for better organization
- Each shortcut remembers your position in that directory
- Name your shortcuts for better organization
- Quick-jump between related directories

## ⌨️ Customizable Keybindings: Bend the Very Interface to Your Will
![image](https://github.com/user-attachments/assets/03fbe400-5a89-4a6c-9fd4-087230c396e4)

Make the interface dance to your tune. Remap keys until your muscle memory sings with joy.

## 🔍 Fuzzy Search: Summon Files with But a Whisper of Their True Name
![image](https://github.com/user-attachments/assets/d33e391f-7799-4f42-bdcb-20fd9d7321c1)

I mean, you can look for what ever you want... I understand your hands might be busy, The 
Stygian Fuzzy Search™ understands your intentions, even if your fingers don't.

## 🌳 Navigational Tree Sorcery

Who says trees only grow one way? With StygianSift, your file tree is a
shape-shifting marvel:

- Sort by name, size, date, or the phase of the moon (okay, maybe not that last
  one)
- Alphabetical today, chronological tomorrow, size-based when you're feeling
  judgmental
- Watch your directories dance in excitement as they do your bidding!

## 🤤 A Visually Seductive Interface... Which is Not a Wierd Thing to Say!
![output_optimized](https://github.com/user-attachments/assets/54a041e0-c3a5-4099-9022-55bacabb7cc6)

- Dress the it down, if you're more into that sort of thing.
- Entries fade based on distance from your selection
- Customizable dimming intensity and distance (in code only at the moment)
- Color and brightness preservation for important items
- Visual breadcrumbs through directory structures
- Instant feedback on current location

## 📝 BYOE (Bring Your Own Editor)

Tired of fighting with vs code? Nostalgic for your trusty notepad? Fear not! StygianSift 
lets you summon your preferred text editor to put quill to parchment! Why wouldn't you want 
tools to do your work right at your fingertips? (Terminal based text editors highly 
recommended)

- Set your editor of choice (I won't judge... much)
- Open files with a keystroke, because who has time for double-clicking?
- As always, Return to the Stygian realm when you're done, because yes.

## 🕳️ The Undo System: Your Safety Net in the Void

StygianSift implements a sophisticated undo system that would make time travelers jealous:

- RAM-based undo for quick operations
- Disk storage for larger operations 
- Configurable storage limits for both RAM and disk
- Preserves file metadata and permissions
- Handles multiple file operations
- 
## ✨ Multi-select Because Why Stop at One
![output_optimized](https://github.com/user-attachments/assets/7e1ce4da-0105-40b4-b24b-6f5601d182f5)

You want to impress that girl over there? Show here how massive your file operations are!
Why settle for manipulating just one file when you can wrangle a whole herd? The multi-select
feature makes file operations a whole lot more convenient:

1. Enter select mode with `Ctrl+t`
2. Mark files with by holding `Shift` and normal navigation
3. Perform operations (copy, move, delete etc) on selection
4. Exit select mode with `Esc`

You can also just hold shift and move... but that's just standard stuff.

## 👁️ The Preview Pane: Your Window into the Souls of Your Files
![image](https://github.com/user-attachments/assets/ef755418-c5ab-4d40-9b34-aa17b71eb46c)

The preview pane is more than just a pretty face:

- Handles massive files without breaking a sweat
- Smart file type detection
- Text for you coding files
- Hex view for binary files
- Directory tree previews
- Metadata display
- Customizable preview size

## 📊 Metadata Mania

Dive deeper into your files' naughty secret little lives:

- Creation dates: because it's important to remember birthdays
- File sizes: Because they are always heavier than they look
- Permissions that read like ancient hieroglyphs (but useful ones!)
- Git status integration
- Extended attributes
- Owner and group information
- Inode details
- Hard link count
  
## 💾 The Config File: Your Digital Diary

Your preferences, remembered:

- Saves your settings 'lest you forget!
- Carries your choices across sessions like a loyal butler
- Configurable right in the TUI!

## 🛠️ Crafted by a Boatman with More Loose Screws Than the Argo

This isn't some tool put together by developers with "experience" or "sanity." It's the 
digital fever dream of a ferryman who decided that bits and bytes were more interesting than 
souls and regrets. Use at your own risk (of increased productivity and possible spontaneous 
iambic pentameter).

## 🌟 Why Trust Your Files to a Madman's Creation?

- Speed: Because your time is better spent not looking for things.
- Power: Maybe? I don't know what it means, It is written in Rust, does that count?.
- Flexibility: Adapt it to your workflow, not the other way around.
- Fun: Because if you're not laughing while managing files... well, you shouldn't. I am 
  being told that isn't normal, and that people are concerned.


## 💤 TLDR Feature List

### Mouse Support Features
- Right-click context menu for file operations
- Dimming-aware mouse tracking and highlighting
- Click-to-navigate directory traversal
- Multiple selection methods:
  - Click-to-select individual files
  - Ctrl+click for non-contiguous selection
  - Drag selection for ranges

### Window Support (beta)
 - Should compile and work on Windows
 - Not extensivly tested

### Core File Operations
- Create files
- Create directories
- Delete files/directories
- Move files/directories
- Copy files/directories
- Rename files (with or without extension preservation)
- Duplicate files/directories
- Execute files
- Open files in external editor

### Navigation Features
- Directory traversal (up/down/enter/back)
- Navigate by search
- Jump to parent directory
- Jump to top/bottom of list
- Shortcut system (0-9 keys)
- Shortcut layers (F1-F10)
- Directory history tracking
- Custom directory naming for shortcuts
- Position memory in directories

### Selection System
- Single file selection
- Multi-file selection
- Range selection
- Selection toggle
- Select all
- Clear selection

### Search Capabilities
- Fuzzy file search
- Color-based filtering
- Configurable search depth
- Real-time search results
- Search within current directory (will include files. TBA)

### Preview Features
- File content preview
- Directory preview
- Preview size adjustment
- File metadata display
- Large file handling
- Binary file handling

### Color Management
- Color coding files/directories
- Color-based operation rules
- Color cascading in directories
- Color filtering
- Color-based search
- Color toggle
- Color cycling

### Sort Options
- Name (ascending/descending)
- Type (ascending/descending)
- Size (ascending/descending)
- Date modified (ascending/descending)
- Color (ascending/descending)

### Undo System
- RAM-based undo storage
- Disk-based undo storage
- Configurable storage limits
- Operation history
- Multiple operation types support
- Automatic cleanup

### Git Integration
- Git status display
- Repository detection
- Unstabled features in code, if you want to try it out.

### UI Features
- Dynamic borders
- Responsive layout
- File type icons
- Permission indicators
- Scroll indicators
- Directory size display
- File count display (for search)
- Multi-column layout

### Terminal Integration
- Terminal command execution
- Working directory preservation
- Command output display
- Return to browser state

### Configuration
- Keybinding customization
- Color rule configuration
- Editor selection
- Search depth limits
- Undo storage limits
- Shortcut management

### Protection Features
- Color-based operation restrictions
- Permission checking
- Admin requirement detection
- Read-only enforcement
- Operation validation
- Recursive permission checking

### Display Features
- Dynamic dimming
- Color preservation
- Text wrapping
- Long filename handling
- Status line display
- Help menu
- Metadata display

### File Information
- Basic metadata
- Extended attributes
- Ownership details
- Permission strings
- Timestamp information
- Size calculations
- Inode information
- Hard link counting

# 💽 Installation

### Requirements
- Rust (latest stable version)
- A terminal that supports:
  - Unicode characters
  - True Color (for using the color system)

StygianSift requires a Nerd Font to display icons correctly. The recommended font is:

- **JetBrainsMono Nerd Font** (Primary font used in development)

Alternative compatible fonts:
- Any Nerd Font variant (FiraCode, Hack, DroidSansMono, Iosevka)

#### Installing the Required Font

#### Method 1: Download directly
1. Visit [Nerd Fonts website](https://www.nerdfonts.com/font-downloads)
2. Download "JetBrainsMono Nerd Font"
3. Install the font on your system

#### Method 2: Package Manager
- **Nix**: `nerdfonts.override { fonts = [ "JetBrainsMono" ]; }`
- **Ubuntu/Debian**: `sudo apt install fonts-jetbrains-mono-nerd`
- **Arch**: `sudo pacman -S ttf-jetbrains-mono-nerd`
- **MacOS**: `brew tap homebrew/cask-fonts && brew install --cask font-jetbrains-mono-nerd-font`

After installing the font, make sure your terminal emulator is configured to use "JetBrainsMono Nerd Font" or "JetBrainsMono NF".

If icons are not displaying correctly, check that:
1. The font is installed properly
2. Your terminal is using the Nerd Font variant
3. Your terminal supports Unicode and true color


### Build from Source
```bash
# Clone the repository
git clone https://github.com/Mauitron/StygianSift.git
cd StygianSift

# Build the release version
cargo build --release

# The binary will be located at
./target/release/StygianSift
```

### Make Globally Available (Optional)
```bash
On Unix-like systems (Linux, macOS), you can make the program available system-wide:

# Copy to your system's binary directory
sudo cp ./target/release/StygianSift /usr/local/bin/

# Now you can run StygianSift from any directory
StygianSift
```

### First Run
```bash
# Run directly
./target/release/StygianSift

# Or if installed system-wide
StygianSift
```

### Power User Tips

- Use color coding to protect important directories
- Map the keybindings to what feels best for you
- Add your text-editor so you can start work from within StygianSift
- Set up shortcuts for frequent locations
- Master the search filters for quick file finding
- Utilize git integration for repo management (comming soon!)

## Keyboard Bindings Reference

Here are some common Bindings to get you started:

| Category    | Action           | Default Key |
|------------|------------------|-------------|
| Navigation | Move up/down     | `k`/`j`     |
|            | Parent directory | `h`         |
|            | Enter directory  | `l`         |
| File Ops   | Copy            | `p`         |
|            | Paste           | `P`         |
|            | Delete          | `D`         |
| Selection  | Toggle select   | `Ctrl+t`    |
|            | Select all      | `Ctrl+a`    |
| View       | Toggle preview  | `Space`     |
|            | Toggle filters  | `Tab`       |

## Troubleshooting

Common issues and their solutions:

- **Permission denied**: Run with sudo or adjust file permissions
- **Preview not working**: Check terminal capabilities and encoding
- **Colors not displaying**: Ensure terminal supports true color
- **Performance issues**: Adjust search depth and preview settings

Ready to turn your digital realm from the mundane to the insane? Grab StygianSift today
and embark on a file odyssey like no other! Your friend will be both jealous and amazed!
Remember, in StygianSift, we don't just think outside the box. We fold the box into a fancy 
hat and wear it! Why? because it looks snazzy. May your code be buggy in interesting ways, 
and may you always find that one file you swear you put somewhere "logical"!

P.S. No refunds, exchanges, or soul-backsies. Apparently 1 Obol is far below minimum wage. 
All sales are final!

P.P.S StygianSift is a work in progress. Please report any bugs you find.
Visual bugs are expected to be the most common ones.
