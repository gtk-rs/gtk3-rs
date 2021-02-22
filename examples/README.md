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

Screenshots of examples using cairo or futures are missing.

#### Full GTK Example
<img src="images/GTKTest.png">

#### Accessibility
<img src="images/Accessibility.png">

#### A Basic Screen
<img src="images/Basic.png">

#### Basic Subclassing
<img src="images/BasicSubclass.png">

#### Builder Basics
<img src="images/BuilderBasics.png">

#### Builders
<img src="images/Builders.png">

#### Builder Signal
<img src="images/BuilderSignal.png">

#### Child Properties
<img src="images/ChildProperties.png">

#### Simple Clipboard
<img src="images/ClipboardSimple.png">

#### Clock
<img src="images/Clock.png">

#### Clone Macros
<img src="images/CloneMacros.png">

#### Communication Thread
<img src="images/CommunicationThread.png">

#### Complex Drag and Drop
<img src="images/ComplexDragDrop.png">

#### CSS
<img src="images/CSS.png">

#### Entry Completion
<img src="images/EntryCompletion.png">

#### Grid
<img src="images/Grid.png">

#### Icon View
<img src="images/Iconview.png">

#### List Box
<img src="images/ListBox.png">

#### List Store
<img src="images/ListStore.png">

#### Menu 
<img src="images/Menu.png">

#### Multithreading
<img src="images/Multithreading.png">

#### Multi Window
<img src="images/MultiWindow.png">

#### Notebook
<img src="images/Notebook.png">

#### Overlay
<img src="images/Overlay.png">

#### Progress Tracker
<img src="images/ProgressTracker.png">

#### Simple Drag and Drop
<img src="images/SimpleDragDrop.png">

#### Sync Widgets
<img src="images/SyncWidget.png">

#### System Menu
<img src="images/SystemMenu.png">

#### Text Viewer
<img src="images/Textviewer.png">

#### Tree View
<img src="images/Treeview.png">

#### Tree View with Sort
<img src="images/TreeviewModelSort.png">

## LICENSE
The gtk-rs examples repository is licensed under the MIT license, please refer to the LICENSE and COPYRIGHT files for more information.
