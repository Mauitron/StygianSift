# Stygian Sift: Navigate the Digital Styx with a Touch of Madness
![image](https://github.com/user-attachments/assets/ffbc17f2-7f52-4f32-bd49-4d439b5dd0b6)

## 📚 Embark on a Slightly Unhinged File Management Adventure

Lost in the labyrinth of your project files? Fear not, for Stygian Sift is here to guide you 
through the digital underworld. Inspired by ME, Charon, the mythical ferryman, this tool will 
navigate you through the rivers of data and forests of directories with unparalleled ease. 
Created by a slightly unhinged boatman who spent way too much time ferrying souls and 
muttering about directory structures, this isn't your average file manager. It's a fever 
dream of functionality wrapped in a user interface that actually makes sense.

## Features and Capabilities

- [Paint your files directories with colors](#-paint-your-files-directories-with-colors)
- [Shortcuts that actually make your life shorter in a good way](#-shortcuts-that-actually-make-your-life-shorter-in-a-good-way)
- [Terminal-based because real programmers dont click](#%EF%B8%8F-terminal-based-because-real-programmers-dont-click)
- [Customizable keybindings bend the very interface to your will](#️-customizable-keybindings-bend-the-very-interface-to-your-will)
- [Fuzzy search summon files with but a whisper of their true name](#-fuzzy-search-summon-files-with-but-a-whisper-of-their-true-name)
- [Navigational tree sorcery](#-navigational-tree-sorcery)
- [BYOE bring your own editor](#-byoe-bring-your-own-editor)
- [The undo resurrect your files from the dead](#️-the-undo-system-your-safety-net-in-the-void)
- [Preview pane peek into files or directories](#️-preview-pane-your-window-into-file-souls)
- [Metadata mania](#-metadata-mania)
- [The config file your digital diary](#-the-config-file-your-digital-diary)
- [Crafted by a boatman with more loose screws than the argo](#️-crafted-by-a-boatman-with-more-loose-screws-than-the-argo)
- [Multi-select because why stop at one](#-multi-select-because-why-stop-at-one)

## 🎨 Paint Your Files Directories with Colors
![image](https://github.com/user-attachments/assets/5f2de4e9-461e-4ce2-b108-b5538570f65d)

In a stroke of genius (or possibly heatstroke), I devised a color-coding system that's as 
powerful as it is perplexing:

- Red: "Touch not, lest ye awaken the Cerberus!" (Or just corrupt your build, same thing)
- Yellow: "Caution: May contain traces of genius or utter nonsense"
- Blue: "Stable as a three-legged chair, but it'll do"
- Green: "Free to modify, or turn into digital origami, whatever floats your boat"

Plot twist: YOU get to decide what these colors mean. It's like playing god with your file 
system, but less smiting and more organizing! These chromatic enchantments go beyond mere 
decoration - they're a full permission and operation control system:

- Define whether files can be deleted, moved, or renamed per color
- Control whether colored files appear in searches
- Colors cascade up directory trees to protect entire structures
- Set custom rules per color for granular control
- Use filters to view only files of specific colors

## 🚀 Shortcuts That Actually Make Your Life Shorter (in a good way)

![image](https://github.com/user-attachments/assets/3ca96b6a-147b-477f-844c-df5e0ad8c1d6)


- Basic shortcuts (0-9) for your most-used directories
- Shift + numbers for setting new shortcuts
- Control + numbers for accessing shortcut layers (Coming soon!)
- Each shortcut remembers your position in that directory
- Name your shortcuts for better organization
- Quick-jump between related directories

## 🖥️ Terminal-based Because Real Programmers Dont Click

Embrace the command line, mortal! Navigate your files with the wild abandon of the 
caffeinated genius that you are! If you need more, summon the terminal within the selected 
directory to perform more complex commands. Return to Stygian Sift when you're done, no 
breadcrumbs needed. Who needs pointing devices when your coding-pinkies could bench more than 
you? Ask yourself, Why point-and-click when you can type-and-curse?

## ⌨️ Customizable Keybindings: Bend the Very Interface to Your Will
![image](https://github.com/user-attachments/assets/03fbe400-5a89-4a6c-9fd4-087230c396e4)

Make the interface dance to your tune. Remap keys until your muscle memory sings with joy.

## 🔍 Fuzzy Search: Summon Files with But a Whisper of Their True Name
![image](https://github.com/user-attachments/assets/d33e391f-7799-4f42-bdcb-20fd9d7321c1)

I mean, you can look for what ever you want... I understand your hands might be busy, The 
stygian fuzzy search understands your intentions, even if your fingers don't.

## 🌳 Navigational Tree Sorcery

Navigate through your directories with the grace of a digital dryad. Sort, filter, and 
traverse your file system, and look cool doing it!

- Entries fade based on distance from your selection
- Customizable dimming intensity and distance
- Color and brightness preservation for important items
- Visual breadcrumbs through directory structures
- Instant feedback on current location

## 📝 BYOE (Bring Your Own Editor)

Tired of fighting with vs code? Nostalgic for your trusty notepad? Fear not! Stygian Sift 
lets you summon your preferred text editor to put quill to parchment! Why wouldn't you want 
tools to do your work right at your fingertips? (Terminal based text editors highly 
recommended)

- Set your editor of choice (I won't judge... much)
- Open files with a keystroke, because who has time for double-clicking?
- As always, Return to Stygian's realm when you're done, because yes.

## 🕳️ The Undo System: Your Safety Net in the Void

Stygian Sift implements a sophisticated undo system that would make time travelers jealous:

- RAM-based undo for quick operations
- Disk storage for larger operations 
- Configurable storage limits for both RAM and disk
- Preserves file metadata and permissions
- Handles multiple file operations
- Undo stack survives program restarts
- Automatic cleanup of old undo data

## 👁️ Preview Pane: Your Window into File Souls
![image](https://github.com/user-attachments/assets/ef755418-c5ab-4d40-9b34-aa17b71eb46c)

The preview pane is more than just a pretty face:

- Handles massive files without breaking a sweat
- Smart file type detection
- Syntax highlighting for code
- Hex view for binary files
- Directory tree previews
- Metadata display
- Customizable preview size
- Automatic encoding detection

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

## Installation

```bash
# Install via cargo
cargo install stygian-sift

# Or build from source
git clone https://github.com/username/stygian-sift
cd stygian-sift
cargo build --release
```

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

## Common Workflows

## ✨ Multi-select Because Why Stop at One

Select files and folders with the enthusiasm of a kid in a candy store. Bulk operations have 
never been more satisfying!

1. Enter select mode with `Ctrl+t`
2. Mark files with `Space`
3. Perform operations (copy, move, delete) on selection
4. Exit select mode with `Esc`

### Power User Tips

- Use color coding to protect important directories
- Set up shortcuts for frequent locations
- Master the search filters for quick file finding
- Utilize git integration for repo management

## Keyboard Shortcut Reference

Here are some of the most commonly used shortcuts:

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

Remember, in Stygian Sift, I don't just think outside the box. I fold the box into a fancy 
hat and wear it! Why? because it looks snazzy. May your code be buggy in interesting ways, 
and may you always find that one file you swear you put somewhere "logical"!

P.S. No refunds, exchanges, or soul-backsies. Apparently 1 Obol is far below minimum wage. 
All sales are final!

P.P.S Stygian Sift is a work in progress. Visual bugs are to be expected.
