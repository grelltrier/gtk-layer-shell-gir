// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use gtk_layer_shell_sys;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
pub enum Edge {
    Left,
    Right,
    Top,
    Bottom,
    EntryNumber,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Edge::{}", match *self {
            Edge::Left => "Left",
            Edge::Right => "Right",
            Edge::Top => "Top",
            Edge::Bottom => "Bottom",
            Edge::EntryNumber => "EntryNumber",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl ToGlib for Edge {
    type GlibType = gtk_layer_shell_sys::GtkLayerShellEdge;

    fn to_glib(&self) -> gtk_layer_shell_sys::GtkLayerShellEdge {
        match *self {
            Edge::Left => gtk_layer_shell_sys::GTK_LAYER_SHELL_EDGE_LEFT,
            Edge::Right => gtk_layer_shell_sys::GTK_LAYER_SHELL_EDGE_RIGHT,
            Edge::Top => gtk_layer_shell_sys::GTK_LAYER_SHELL_EDGE_TOP,
            Edge::Bottom => gtk_layer_shell_sys::GTK_LAYER_SHELL_EDGE_BOTTOM,
            Edge::EntryNumber => gtk_layer_shell_sys::GTK_LAYER_SHELL_EDGE_ENTRY_NUMBER,
            Edge::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<gtk_layer_shell_sys::GtkLayerShellEdge> for Edge {
    fn from_glib(value: gtk_layer_shell_sys::GtkLayerShellEdge) -> Self {
        skip_assert_initialized!();
        match value {
            0 => Edge::Left,
            1 => Edge::Right,
            2 => Edge::Top,
            3 => Edge::Bottom,
            4 => Edge::EntryNumber,
            value => Edge::__Unknown(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
pub enum Layer {
    Background,
    Bottom,
    Top,
    Overlay,
    EntryNumber,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Layer::{}", match *self {
            Layer::Background => "Background",
            Layer::Bottom => "Bottom",
            Layer::Top => "Top",
            Layer::Overlay => "Overlay",
            Layer::EntryNumber => "EntryNumber",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl ToGlib for Layer {
    type GlibType = gtk_layer_shell_sys::GtkLayerShellLayer;

    fn to_glib(&self) -> gtk_layer_shell_sys::GtkLayerShellLayer {
        match *self {
            Layer::Background => gtk_layer_shell_sys::GTK_LAYER_SHELL_LAYER_BACKGROUND,
            Layer::Bottom => gtk_layer_shell_sys::GTK_LAYER_SHELL_LAYER_BOTTOM,
            Layer::Top => gtk_layer_shell_sys::GTK_LAYER_SHELL_LAYER_TOP,
            Layer::Overlay => gtk_layer_shell_sys::GTK_LAYER_SHELL_LAYER_OVERLAY,
            Layer::EntryNumber => gtk_layer_shell_sys::GTK_LAYER_SHELL_LAYER_ENTRY_NUMBER,
            Layer::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<gtk_layer_shell_sys::GtkLayerShellLayer> for Layer {
    fn from_glib(value: gtk_layer_shell_sys::GtkLayerShellLayer) -> Self {
        skip_assert_initialized!();
        match value {
            0 => Layer::Background,
            1 => Layer::Bottom,
            2 => Layer::Top,
            3 => Layer::Overlay,
            4 => Layer::EntryNumber,
            value => Layer::__Unknown(value),
        }
    }
}

