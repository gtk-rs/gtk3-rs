# gtk-rs examples 

A few gtk-rs examples. To build, just do:

```Shell
> cargo build
```

or to enable GTK 3.x depending on the version needed by the example (check Cargo.toml `[features]` to see all specific GTK compile features available):

```Shell
> cargo build --features gtk_3_18
> cargo build --all-features
```

And then run the executables with:

``` Shell
./target/debug/EXAMPLE-NAME
```

or with cargo run (repeating the compilation features used above), example:

``` Shell
cargo run --all-features --bin EXAMPLE-NAME
```

Please be sure to have installed all the required libraries before building examples (the list is available on the [gtk-rs](https://github.com/gtk-rs/gtk/) repository).

## Example Screenshots

Screenshots of examples using cario or futures is missing. Examples are from cross compiling projects for Windows 10.

#### Full GTK Example
<img src="../images/rust-gtk/GTKTest.png">

#### Accessibility
<img src="../images/rust-gtk/Accessibility.png">

#### A Basic Screen
<img src="../images/rust-gtk/Basic.png">

#### Basic Subclassing
<img src="../images/rust-gtk/BasicSubclass.png">

#### Builder Basics
<img src="../images/rust-gtk/BuilderBasics.png">

#### Builders
<img src="../images/rust-gtk/Builders.png">

#### Builder Signal
<img src="../images/rust-gtk/BuilderSignal.png">

#### Child Properties
<img src="../images/rust-gtk/ChildProperties.png">

#### Simple Clipboard
<img src="../images/rust-gtk/ClipboardSimple.png">

#### Clock
<img src="../images/rust-gtk/Clock.png">

#### Clone Macros
<img src="../images/rust-gtk/CloneMacros.png">

#### Communication Thread
<img src="../images/rust-gtk/CommunicationThread.png">

#### Complex Drag and Drop
<img src="../images/rust-gtk/ComplexDragDrop.png">

#### CSS
<img src="../images/rust-gtk/CSS.png">

#### Entry Completion
<img src="../images/rust-gtk/EntryCompletion.png">

#### Grid
<img src="../images/rust-gtk/Grid.png">

#### Icon View
<img src="../images/rust-gtk/Iconview.png">

#### List Box
<img src="../images/rust-gtk/ListBox.png">

#### List Store
<img src="../images/rust-gtk/ListStore.png">

#### Menu 
<img src="../images/rust-gtk/Menu.png">

#### Multithreading
<img src="../images/rust-gtk/Multithreading.png">

#### Multi Window
<img src="../images/rust-gtk/MultiWindow.png">

#### Notebook
<img src="../images/rust-gtk/Notebook.png">

#### Overlay
<img src="../images/rust-gtk/Overlay.png">

#### Progress Tracker
<img src="../images/rust-gtk/ProgressTracker.png">

#### Simple Drag and Drop
<img src="../images/rust-gtk/SimpleDragDrop.png">

#### Sync Widgets
<img src="../images/rust-gtk/SyncWidget.png">

#### System Menu
<img src="../images/rust-gtk/SystemMenu.png">

#### Text Viewer
<img src="../images/rust-gtk/Textviewer.png">

#### Tree View
<img src="../images/rust-gtk/Treeview.png">

#### Tree View with Sort
<img src="../images/rust-gtk/TreeviewModelSort.png">

## LICENSE
The gtk-rs examples repository is licensed under the MIT license, please refer to the LICENSE and COPYRIGHT files for more information.
