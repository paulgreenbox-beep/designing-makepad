//! A deliberately small, document-agnostic icon vocabulary.
//!
//! The visual assets live in `resources/icons/` and come from Lucide.  This
//! module contains no GLYF types: an editor, a browser shell, or a settings
//! application can all use the same names and accessibility labels.

use core::fmt;

/// One icon from the curated Lucide subset bundled by this crate.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum IconName {
    FilePlus,
    FolderOpen,
    Save,
    Package,
    Undo,
    Redo,
    Copy,
    Scissors,
    Clipboard,
    Plus,
    Close,
    ChevronDown,
    ChevronLeft,
    ChevronRight,
    More,
    MoreVertical,
    Settings,
    Help,
    Info,
    Pointer,
    Point,
    Region,
    Crop,
    Fit,
    ZoomIn,
    ZoomOut,
    Pan,
    Layers,
    Image,
    PanelRight,
    PanelLeft,
    Eye,
    Warning,
    Trash,
    Check,
    FileImage,
    Sliders,
    LayoutPanelLeft,
}

impl IconName {
    /// The catalog order doubles as the gallery order in the design lab.
    pub const ALL: &[Self] = &[
        Self::FilePlus,
        Self::FolderOpen,
        Self::Save,
        Self::Package,
        Self::Undo,
        Self::Redo,
        Self::Copy,
        Self::Scissors,
        Self::Clipboard,
        Self::Plus,
        Self::Close,
        Self::ChevronDown,
        Self::ChevronLeft,
        Self::ChevronRight,
        Self::More,
        Self::MoreVertical,
        Self::Settings,
        Self::Help,
        Self::Info,
        Self::Pointer,
        Self::Point,
        Self::Region,
        Self::Crop,
        Self::Fit,
        Self::ZoomIn,
        Self::ZoomOut,
        Self::Pan,
        Self::Layers,
        Self::Image,
        Self::PanelRight,
        Self::PanelLeft,
        Self::Eye,
        Self::Warning,
        Self::Trash,
        Self::Check,
        Self::FileImage,
        Self::Sliders,
        Self::LayoutPanelLeft,
    ];

    /// File stem below `resources/icons/`.
    pub const fn file_stem(self) -> &'static str {
        match self {
            Self::FilePlus => "file-plus",
            Self::FolderOpen => "folder-open",
            Self::Save => "save",
            Self::Package => "package",
            Self::Undo => "undo-2",
            Self::Redo => "redo-2",
            Self::Copy => "copy",
            Self::Scissors => "scissors",
            Self::Clipboard => "clipboard",
            Self::Plus => "plus",
            Self::Close => "x",
            Self::ChevronDown => "chevron-down",
            Self::ChevronLeft => "chevron-left",
            Self::ChevronRight => "chevron-right",
            Self::More => "ellipsis",
            Self::MoreVertical => "ellipsis-vertical",
            Self::Settings => "settings-2",
            Self::Help => "circle-question-mark",
            Self::Info => "info",
            Self::Pointer => "mouse-pointer-2",
            Self::Point => "crosshair",
            Self::Region => "square-dashed-mouse-pointer",
            Self::Crop => "crop",
            Self::Fit => "maximize",
            Self::ZoomIn => "zoom-in",
            Self::ZoomOut => "zoom-out",
            Self::Pan => "move",
            Self::Layers => "layers",
            Self::Image => "image",
            Self::PanelRight => "panel-right-open",
            Self::PanelLeft => "panel-left-open",
            Self::Eye => "eye",
            Self::Warning => "triangle-alert",
            Self::Trash => "trash-2",
            Self::Check => "check",
            Self::FileImage => "file-image",
            Self::Sliders => "sliders-horizontal",
            Self::LayoutPanelLeft => "layout-panel-left",
        }
    }

    /// Stable human name for tooltips and accessibility surfaces.
    pub const fn label(self) -> &'static str {
        match self {
            Self::FilePlus => "New file",
            Self::FolderOpen => "Open file",
            Self::Save => "Save",
            Self::Package => "Package",
            Self::Undo => "Undo",
            Self::Redo => "Redo",
            Self::Copy => "Copy",
            Self::Scissors => "Cut",
            Self::Clipboard => "Paste",
            Self::Plus => "Add",
            Self::Close => "Close",
            Self::ChevronDown => "More options",
            Self::ChevronLeft => "Back",
            Self::ChevronRight => "Forward",
            Self::More => "More actions (horizontal)",
            Self::MoreVertical => "More actions (vertical)",
            Self::Settings => "Settings",
            Self::Help => "Help",
            Self::Info => "Information",
            Self::Pointer => "Select",
            Self::Point => "Point annotation",
            Self::Region => "Region annotation",
            Self::Crop => "Crop",
            Self::Fit => "Fit to view",
            Self::ZoomIn => "Zoom in",
            Self::ZoomOut => "Zoom out",
            Self::Pan => "Pan",
            Self::Layers => "Layers",
            Self::Image => "Image",
            Self::PanelRight => "Show right panel",
            Self::PanelLeft => "Show left panel",
            Self::Eye => "Visibility",
            Self::Warning => "Warning",
            Self::Trash => "Delete",
            Self::Check => "Confirm",
            Self::FileImage => "Image file",
            Self::Sliders => "Adjustments",
            Self::LayoutPanelLeft => "Panel layout",
        }
    }

    /// Relative resource path for documentation and asset audits.
    pub const fn resource_path(self) -> &'static str {
        match self {
            Self::FilePlus => "resources/icons/file-plus.svg",
            Self::FolderOpen => "resources/icons/folder-open.svg",
            Self::Save => "resources/icons/save.svg",
            Self::Package => "resources/icons/package.svg",
            Self::Undo => "resources/icons/undo-2.svg",
            Self::Redo => "resources/icons/redo-2.svg",
            Self::Copy => "resources/icons/copy.svg",
            Self::Scissors => "resources/icons/scissors.svg",
            Self::Clipboard => "resources/icons/clipboard.svg",
            Self::Plus => "resources/icons/plus.svg",
            Self::Close => "resources/icons/x.svg",
            Self::ChevronDown => "resources/icons/chevron-down.svg",
            Self::ChevronLeft => "resources/icons/chevron-left.svg",
            Self::ChevronRight => "resources/icons/chevron-right.svg",
            Self::More => "resources/icons/ellipsis.svg",
            Self::MoreVertical => "resources/icons/ellipsis-vertical.svg",
            Self::Settings => "resources/icons/settings-2.svg",
            Self::Help => "resources/icons/circle-question-mark.svg",
            Self::Info => "resources/icons/info.svg",
            Self::Pointer => "resources/icons/mouse-pointer-2.svg",
            Self::Point => "resources/icons/crosshair.svg",
            Self::Region => "resources/icons/square-dashed-mouse-pointer.svg",
            Self::Crop => "resources/icons/crop.svg",
            Self::Fit => "resources/icons/maximize.svg",
            Self::ZoomIn => "resources/icons/zoom-in.svg",
            Self::ZoomOut => "resources/icons/zoom-out.svg",
            Self::Pan => "resources/icons/move.svg",
            Self::Layers => "resources/icons/layers.svg",
            Self::Image => "resources/icons/image.svg",
            Self::PanelRight => "resources/icons/panel-right-open.svg",
            Self::PanelLeft => "resources/icons/panel-left-open.svg",
            Self::Eye => "resources/icons/eye.svg",
            Self::Warning => "resources/icons/triangle-alert.svg",
            Self::Trash => "resources/icons/trash-2.svg",
            Self::Check => "resources/icons/check.svg",
            Self::FileImage => "resources/icons/file-image.svg",
            Self::Sliders => "resources/icons/sliders-horizontal.svg",
            Self::LayoutPanelLeft => "resources/icons/layout-panel-left.svg",
        }
    }
}

impl fmt::Display for IconName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.file_stem())
    }
}

/// Colour family shared by a rune, its stone edge, and its hover underglow.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SignalColorSet {
    Ember,
    Azure,
    Verdant,
    Violet,
    Gold,
}

impl SignalColorSet {
    pub const ALL: &[Self] = &[
        Self::Ember,
        Self::Azure,
        Self::Verdant,
        Self::Violet,
        Self::Gold,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Ember => "Ember",
            Self::Azure => "Azure",
            Self::Verdant => "Verdant",
            Self::Violet => "Violet",
            Self::Gold => "Gold",
        }
    }

    pub const fn accent_hex(self) -> &'static str {
        match self {
            Self::Ember => "#FF7A18",
            Self::Azure => "#36B6FF",
            Self::Verdant => "#7BDC70",
            Self::Violet => "#B980FF",
            Self::Gold => "#F4C34D",
        }
    }
}

impl fmt::Display for SignalColorSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

/// A reusable silhouette for the dark rune stone behind an editor icon.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RuneStoneShape {
    Circle,
    Squircle,
    Tall,
    Wide,
    Hex,
    Shard,
    Arch,
    River,
}

impl RuneStoneShape {
    pub const ALL: &[Self] = &[
        Self::Circle,
        Self::Squircle,
        Self::Tall,
        Self::Wide,
        Self::Hex,
        Self::Shard,
        Self::Arch,
        Self::River,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Circle => "Circle",
            Self::Squircle => "Squircle",
            Self::Tall => "Tall",
            Self::Wide => "Wide",
            Self::Hex => "Hex",
            Self::Shard => "Shard",
            Self::Arch => "Arch",
            Self::River => "River",
        }
    }
}

impl fmt::Display for RuneStoneShape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}
/// One original, multi-colour icon from the Signal editor set.
///
/// Signal icons retain their white line work and use a semantic colour family
/// for their action accent. They are intended for `DsRuneStoneButton`; the
/// normal `DsIconButton` deliberately tints its source icon to a theme colour.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SignalIconName {
    New,
    Open,
    Save,
    Image,
    Select,
    Point,
    Region,
    Fit,
    ZoomIn,
    ZoomOut,
    Layers,
    Package,
    Undo,
    Redo,
    Copy,
    Cut,
    Paste,
}

impl SignalIconName {
    /// Catalog order for the Signal icon gallery and asset audits.
    pub const ALL: &[Self] = &[
        Self::New,
        Self::Open,
        Self::Save,
        Self::Image,
        Self::Select,
        Self::Point,
        Self::Region,
        Self::Fit,
        Self::ZoomIn,
        Self::ZoomOut,
        Self::Layers,
        Self::Package,
        Self::Undo,
        Self::Redo,
        Self::Copy,
        Self::Cut,
        Self::Paste,
    ];

    /// Semantic colour family for the rune accent and the stone hover glow.
    pub const fn color_set(self) -> SignalColorSet {
        match self {
            Self::New | Self::Save => SignalColorSet::Ember,
            Self::Image | Self::Layers | Self::Redo => SignalColorSet::Verdant,
            Self::Select | Self::Point | Self::Region => SignalColorSet::Azure,
            Self::Fit | Self::ZoomIn | Self::ZoomOut => SignalColorSet::Violet,
            Self::Open | Self::Package | Self::Undo | Self::Copy | Self::Cut | Self::Paste => {
                SignalColorSet::Gold
            }
        }
    }

    /// Stone silhouette selected for the icon button.
    pub const fn stone_shape(self) -> RuneStoneShape {
        match self {
            Self::New | Self::Point => RuneStoneShape::Circle,
            Self::Save | Self::Layers => RuneStoneShape::Squircle,
            Self::ZoomIn | Self::Paste => RuneStoneShape::Tall,
            Self::Open | Self::ZoomOut => RuneStoneShape::Wide,
            Self::Fit | Self::Package => RuneStoneShape::Hex,
            Self::Select | Self::Redo | Self::Cut => RuneStoneShape::Shard,
            Self::Region | Self::Copy => RuneStoneShape::Arch,
            Self::Image | Self::Undo => RuneStoneShape::River,
        }
    }
    /// File stem below `resources/icons/signal/`.
    pub const fn file_stem(self) -> &'static str {
        match self {
            Self::New => "signal-new",
            Self::Open => "signal-open",
            Self::Save => "signal-save",
            Self::Image => "signal-image",
            Self::Select => "signal-select",
            Self::Point => "signal-point",
            Self::Region => "signal-region",
            Self::Fit => "signal-fit",
            Self::ZoomIn => "signal-zoom-in",
            Self::ZoomOut => "signal-zoom-out",
            Self::Layers => "signal-layers",
            Self::Package => "signal-package",
            Self::Undo => "signal-undo",
            Self::Redo => "signal-redo",
            Self::Copy => "signal-copy",
            Self::Cut => "signal-cut",
            Self::Paste => "signal-paste",
        }
    }

    /// Stable human name for tooltips and an icon picker.
    pub const fn label(self) -> &'static str {
        match self {
            Self::New => "New file",
            Self::Open => "Open file",
            Self::Save => "Save",
            Self::Image => "Image",
            Self::Select => "Select",
            Self::Point => "Point annotation",
            Self::Region => "Region annotation",
            Self::Fit => "Fit to view",
            Self::ZoomIn => "Zoom in",
            Self::ZoomOut => "Zoom out",
            Self::Layers => "Layers",
            Self::Package => "Package",
            Self::Undo => "Undo",
            Self::Redo => "Redo",
            Self::Copy => "Copy",
            Self::Cut => "Cut",
            Self::Paste => "Paste",
        }
    }

    /// Relative resource path for documentation and asset audits.
    pub const fn resource_path(self) -> &'static str {
        match self {
            Self::New => "resources/icons/signal/signal-new.svg",
            Self::Open => "resources/icons/signal/signal-open.svg",
            Self::Save => "resources/icons/signal/signal-save.svg",
            Self::Image => "resources/icons/signal/signal-image.svg",
            Self::Select => "resources/icons/signal/signal-select.svg",
            Self::Point => "resources/icons/signal/signal-point.svg",
            Self::Region => "resources/icons/signal/signal-region.svg",
            Self::Fit => "resources/icons/signal/signal-fit.svg",
            Self::ZoomIn => "resources/icons/signal/signal-zoom-in.svg",
            Self::ZoomOut => "resources/icons/signal/signal-zoom-out.svg",
            Self::Layers => "resources/icons/signal/signal-layers.svg",
            Self::Package => "resources/icons/signal/signal-package.svg",
            Self::Undo => "resources/icons/signal/signal-undo.svg",
            Self::Redo => "resources/icons/signal/signal-redo.svg",
            Self::Copy => "resources/icons/signal/signal-copy.svg",
            Self::Cut => "resources/icons/signal/signal-cut.svg",
            Self::Paste => "resources/icons/signal/signal-paste.svg",
        }
    }
}

impl fmt::Display for SignalIconName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.file_stem())
    }
}
/// API-level description of an icon button, independent of an application's
/// document model. `DsIconButton` is the corresponding visual preset.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IconButtonSpec {
    pub icon: IconName,
    pub tooltip: &'static str,
    pub selected: bool,
    pub disabled: bool,
    pub text: Option<&'static str>,
}

impl IconButtonSpec {
    pub const fn new(icon: IconName, tooltip: &'static str) -> Self {
        Self {
            icon,
            tooltip,
            selected: false,
            disabled: false,
            text: None,
        }
    }

    pub const fn selected(mut self) -> Self {
        self.selected = true;
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub const fn with_text(mut self, text: &'static str) -> Self {
        self.text = Some(text);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn curated_assets_have_unique_paths_and_labels() {
        let mut paths = BTreeSet::new();
        let mut labels = BTreeSet::new();
        for icon in IconName::ALL {
            assert!(paths.insert(icon.resource_path()));
            assert!(labels.insert(icon.label()));
        }
    }

    #[test]
    fn signal_assets_have_unique_paths_and_labels() {
        let mut paths = BTreeSet::new();
        let mut labels = BTreeSet::new();
        for icon in SignalIconName::ALL {
            assert!(icon.resource_path().starts_with("resources/icons/signal/"));
            assert!(paths.insert(icon.resource_path()));
            assert!(labels.insert(icon.label()));
        }
    }
    #[test]
    fn signal_assets_cover_every_colour_set_and_stone_shape() {
        let mut colour_sets = BTreeSet::new();
        let mut stone_shapes = BTreeSet::new();
        let mut accents = BTreeSet::new();
        for icon in SignalIconName::ALL {
            colour_sets.insert(icon.color_set());
            stone_shapes.insert(icon.stone_shape());
            accents.insert(icon.color_set().accent_hex());
        }
        assert_eq!(colour_sets.len(), SignalColorSet::ALL.len());
        assert_eq!(stone_shapes.len(), RuneStoneShape::ALL.len());
        assert_eq!(accents.len(), SignalColorSet::ALL.len());
    }
    #[test]
    fn button_spec_keeps_presentation_state_outside_document_types() {
        let spec = IconButtonSpec::new(IconName::Crop, "Crop selected image")
            .selected()
            .with_text("Crop");
        assert!(spec.selected);
        assert_eq!(spec.text, Some("Crop"));
    }
}
