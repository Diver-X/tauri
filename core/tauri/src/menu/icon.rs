// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use super::NativeIcon;
use crate::{run_main_thread, runtime::menu as muda, AppHandle, Icon, Runtime};

/// A menu item inside a [`Menu`] or [`Submenu`] and contains only text.
///
/// [`Menu`]: super::Menu
/// [`Submenu`]: super::Submenu
pub struct IconMenuItem<R: Runtime> {
  pub(crate) inner: muda::IconMenuItem,
  pub(crate) app_handle: AppHandle<R>,
}

impl<R: Runtime> Clone for IconMenuItem<R> {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.clone(),
      app_handle: self.app_handle.clone(),
    }
  }
}

/// # Safety
///
/// We make sure it always runs on the main thread.
unsafe impl<R: Runtime> Sync for IconMenuItem<R> {}
unsafe impl<R: Runtime> Send for IconMenuItem<R> {}

impl<R: Runtime> super::sealed::IsMenuItemBase for IconMenuItem<R> {
  fn inner(&self) -> &dyn muda::IsMenuItem {
    &self.inner
  }
}

impl<R: Runtime> super::IsMenuItem<R> for IconMenuItem<R> {
  fn kind(&self) -> super::MenuItemKind<R> {
    super::MenuItemKind::Icon(self.clone())
  }

  fn id(&self) -> crate::Result<u32> {
    self.id()
  }
}

impl<R: Runtime> IconMenuItem<R> {
  /// Create a new menu item.
  ///
  /// - `text` could optionally contain an `&` before a character to assign this character as the mnemonic
  /// for this menu item. To display a `&` without assigning a mnemenonic, use `&&`.
  pub fn new<S: AsRef<str>>(
    app_handle: &AppHandle<R>,
    text: S,
    enabled: bool,
    icon: Option<Icon>,
    acccelerator: Option<S>,
  ) -> Self {
    Self {
      inner: muda::IconMenuItem::new(
        text,
        enabled,
        icon
          .and_then(|i| -> Option<crate::runtime::Icon> { i.try_into().ok() })
          .and_then(|i| i.try_into().ok()),
        acccelerator.and_then(|s| s.as_ref().parse().ok()),
      ),
      app_handle: app_handle.clone(),
    }
  }

  /// Create a new icon menu item but with a native icon.
  ///
  /// See [`IconMenuItem::new`] for more info.
  ///
  /// ## Platform-specific:
  ///
  /// - **Windows / Linux**: Unsupported.
  pub fn with_native_icon<S: AsRef<str>>(
    app_handle: &AppHandle<R>,
    text: S,
    enabled: bool,
    native_icon: Option<NativeIcon>,
    acccelerator: Option<S>,
  ) -> Self {
    Self {
      inner: muda::IconMenuItem::with_native_icon(
        text,
        enabled,
        native_icon,
        acccelerator.and_then(|s| s.as_ref().parse().ok()),
      ),
      app_handle: app_handle.clone(),
    }
  }

  /// The application handle associated with this type.
  pub fn app_handle(&self) -> AppHandle<R> {
    self.app_handle.clone()
  }

  /// Returns a unique identifier associated with this menu item.
  pub fn id(&self) -> crate::Result<u32> {
    run_main_thread!(self, |self_: Self| self_.inner.id())
  }

  /// Get the text for this menu item.
  pub fn text(&self) -> crate::Result<String> {
    run_main_thread!(self, |self_: Self| self_.inner.text())
  }

  /// Set the text for this menu item. `text` could optionally contain
  /// an `&` before a character to assign this character as the mnemonic
  /// for this menu item. To display a `&` without assigning a mnemenonic, use `&&`.
  pub fn set_text<S: AsRef<str>>(&self, text: S) -> crate::Result<()> {
    let text = text.as_ref().to_string();
    run_main_thread!(self, |self_: Self| self_.inner.set_text(text))
  }

  /// Get whether this menu item is enabled or not.
  pub fn is_enabled(&self) -> crate::Result<bool> {
    run_main_thread!(self, |self_: Self| self_.inner.is_enabled())
  }

  /// Enable or disable this menu item.
  pub fn set_enabled(&self, enabled: bool) -> crate::Result<()> {
    run_main_thread!(self, |self_: Self| self_.inner.set_enabled(enabled))
  }

  /// Set this menu item accelerator.
  pub fn set_accelerator<S: AsRef<str>>(&self, acccelerator: Option<S>) -> crate::Result<()> {
    let accel = acccelerator.and_then(|s| s.as_ref().parse().ok());
    run_main_thread!(self, |self_: Self| self_.inner.set_accelerator(accel))?.map_err(Into::into)
  }

  /// Change this menu item icon or remove it.
  pub fn set_icon(&self, icon: Option<Icon>) -> crate::Result<()> {
    run_main_thread!(self, |self_: Self| self_.inner.set_icon(
      icon
        .and_then(|i| -> Option<crate::runtime::Icon> { i.try_into().ok() })
        .and_then(|i| i.try_into().ok())
    ))
  }

  /// Change this menu item icon to a native image or remove it.
  ///
  /// ## Platform-specific:
  ///
  /// - **Windows / Linux**: Unsupported.
  pub fn set_native_icon(&mut self, _icon: Option<NativeIcon>) -> crate::Result<()> {
    #[cfg(target_os = "macos")]
    return run_main_thread!(self, |mut self_: Self| self_.inner.set_native_icon(_icon));
    #[allow(unreachable_code)]
    Ok(())
  }
}