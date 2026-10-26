//! Pure Rust, application-agnostic document-tab lifecycle state.
//!
//! [`TabRegistry`] owns stable [`TabId`] values, visual order, active
//! selection and small visual flags. IDs are never derived from a title, an
//! index, a widget, or a backend handle. A Makepad component merely projects
//! the model into renderer keys and returns actions to the application shell.

use std::{
    cmp::Ordering,
    collections::BTreeMap,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    num::NonZeroU64,
};

/// Stable identity for a tab belonging to one logical payload type.
pub struct TabId<T> {
    raw: NonZeroU64,
    marker: PhantomData<fn() -> T>,
}

impl<T> Copy for TabId<T> {}

impl<T> Clone for TabId<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> fmt::Debug for TabId<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_tuple("TabId").field(&self.raw).finish()
    }
}

impl<T> PartialEq for TabId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl<T> Eq for TabId<T> {}

impl<T> PartialOrd for TabId<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for TabId<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.raw.cmp(&other.raw)
    }
}

impl<T> Hash for TabId<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw.hash(state);
    }
}

impl<T> TabId<T> {
    fn new(raw: NonZeroU64) -> Self {
        Self {
            raw,
            marker: PhantomData,
        }
    }

    /// Returns the stable numeric portion for diagnostics, semantics, or a
    /// renderer mapping. It is not an ordering index.
    pub const fn get(self) -> u64 {
        self.raw.get()
    }
}

impl<T> fmt::Display for TabId<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.raw.fmt(formatter)
    }
}

/// UI flags selected by the application when a document tab is created.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TabOptions {
    /// Shows the unsaved/dirty indicator in a visual projection.
    pub dirty: bool,
    /// Whether the visual component exposes a close affordance.
    pub closable: bool,
}

impl Default for TabOptions {
    fn default() -> Self {
        Self {
            dirty: false,
            closable: true,
        }
    }
}

/// A title, visual state and application-owned payload for one document tab.
#[derive(Debug)]
pub struct DocumentTab<T> {
    title: String,
    dirty: bool,
    closable: bool,
    value: T,
}

impl<T> DocumentTab<T> {
    /// Display text only; it is not canonical identity and may be duplicated.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Whether the application considers the document unsaved.
    pub const fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Whether the application allows this document to be closed by this tab strip.
    pub const fn is_closable(&self) -> bool {
        self.closable
    }

    /// Application-owned state associated with this document.
    pub fn value(&self) -> &T {
        &self.value
    }
}

/// A targeted operation received an ID that is not currently live.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TabRegistryError {
    /// The requested tab was closed or never belonged to this registry.
    UnknownTab,
    /// The tab is intentionally permanent and has no close affordance.
    TabNotClosable,
}

impl fmt::Display for TabRegistryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownTab => formatter.write_str("tab ID is not live"),
            Self::TabNotClosable => formatter.write_str("tab is not closable"),
        }
    }
}

impl std::error::Error for TabRegistryError {}

/// Tab identifiers were exhausted. This requires more than `u64::MAX` adds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TabIdExhausted;

impl fmt::Display for TabIdExhausted {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("tab ID allocator is exhausted")
    }
}

impl std::error::Error for TabIdExhausted {}

/// Ordered document-tab state with a separate active selection.
///
/// A new tab becomes active. Closing the active tab selects the tab to its
/// right; when it has no right neighbor, selection moves left. Closing an
/// inactive tab preserves selection. Closing the final tab leaves no active
/// tab.
pub struct TabRegistry<T> {
    next_raw_id: u64,
    entries: BTreeMap<TabId<T>, DocumentTab<T>>,
    order: Vec<TabId<T>>,
    active: Option<TabId<T>>,
}

impl<T> TabRegistry<T> {
    /// Creates an empty tab collection with its own stable-ID allocator.
    pub const fn new() -> Self {
        Self {
            next_raw_id: 0,
            entries: BTreeMap::new(),
            order: Vec::new(),
            active: None,
        }
    }

    /// Appends a closable, clean tab and selects it.
    pub fn add(&mut self, title: impl Into<String>, value: T) -> Result<TabId<T>, TabIdExhausted> {
        self.add_with_options(title, value, TabOptions::default())
    }

    /// Appends a tab with application-selected visual flags and selects it.
    pub fn add_with_options(
        &mut self,
        title: impl Into<String>,
        value: T,
        options: TabOptions,
    ) -> Result<TabId<T>, TabIdExhausted> {
        self.next_raw_id = self.next_raw_id.checked_add(1).ok_or(TabIdExhausted)?;
        let raw = NonZeroU64::new(self.next_raw_id).expect("incremented ID is non-zero");
        let id = TabId::new(raw);
        self.entries.insert(
            id,
            DocumentTab {
                title: title.into(),
                dirty: options.dirty,
                closable: options.closable,
                value,
            },
        );
        self.order.push(id);
        self.active = Some(id);
        Ok(id)
    }

    /// Activates a specific live tab without changing visual order.
    pub fn activate(&mut self, id: TabId<T>) -> Result<(), TabRegistryError> {
        self.ensure_live(id)?;
        self.active = Some(id);
        Ok(())
    }

    /// Updates the unsaved marker for one targeted document.
    pub fn set_dirty(&mut self, id: TabId<T>, dirty: bool) -> Result<(), TabRegistryError> {
        let entry = self
            .entries
            .get_mut(&id)
            .ok_or(TabRegistryError::UnknownTab)?;
        entry.dirty = dirty;
        Ok(())
    }

    /// Updates whether a targeted document exposes a close affordance.
    pub fn set_closable(&mut self, id: TabId<T>, closable: bool) -> Result<(), TabRegistryError> {
        let entry = self
            .entries
            .get_mut(&id)
            .ok_or(TabRegistryError::UnknownTab)?;
        entry.closable = closable;
        Ok(())
    }

    /// Resolves a stable numeric ID received from a renderer or automation layer.
    pub fn id_from_raw(&self, raw: u64) -> Option<TabId<T>> {
        self.order.iter().copied().find(|id| id.get() == raw)
    }

    /// Returns the selected tab's stable ID, if any.
    pub const fn active(&self) -> Option<TabId<T>> {
        self.active
    }

    /// Looks up a tab by stable identity. It never falls back to the active tab.
    pub fn get(&self, id: TabId<T>) -> Option<&DocumentTab<T>> {
        self.entries.get(&id)
    }

    /// Returns the selected tab's identity and entry, if it exists.
    pub fn active_entry(&self) -> Option<(TabId<T>, &DocumentTab<T>)> {
        let id = self.active?;
        self.get(id).map(|entry| (id, entry))
    }

    /// Iterates live tabs in visual order.
    pub fn tabs(&self) -> impl ExactSizeIterator<Item = (TabId<T>, &DocumentTab<T>)> + '_ {
        self.order.iter().copied().map(|id| {
            let entry = self
                .entries
                .get(&id)
                .expect("tab order contains only live entries");
            (id, entry)
        })
    }

    /// Number of live tabs.
    pub fn len(&self) -> usize {
        self.order.len()
    }

    /// Whether this collection contains no tabs.
    pub fn is_empty(&self) -> bool {
        self.order.is_empty()
    }

    /// Closes one targeted, closable tab.
    ///
    /// A stale ID leaves all live state unchanged and returns
    /// [`TabRegistryError::UnknownTab`].
    pub fn close(&mut self, id: TabId<T>) -> Result<DocumentTab<T>, TabRegistryError> {
        let position = self.position(id).ok_or(TabRegistryError::UnknownTab)?;
        if !self
            .entries
            .get(&id)
            .expect("tab order contains only live entries")
            .is_closable()
        {
            return Err(TabRegistryError::TabNotClosable);
        }
        let was_active = self.active == Some(id);
        let replacement = was_active
            .then(|| self.active_after_close(position))
            .flatten();
        let removed = self
            .entries
            .remove(&id)
            .expect("tab order contains only live entries");
        self.order.remove(position);
        if was_active {
            self.active = replacement;
        }
        Ok(removed)
    }

    /// Closes the selected tab, or returns `Ok(None)` for an empty collection.
    pub fn close_active(&mut self) -> Result<Option<DocumentTab<T>>, TabRegistryError> {
        self.active.map(|id| self.close(id)).transpose()
    }

    /// Selects the next tab, wrapping at the end. Useful for `Ctrl+Tab`.
    pub fn next(&mut self) -> Option<TabId<T>> {
        self.cycle(false)
    }

    /// Selects the previous tab, wrapping at the beginning. Useful for
    /// `Ctrl+Shift+Tab`.
    pub fn previous(&mut self) -> Option<TabId<T>> {
        self.cycle(true)
    }

    fn ensure_live(&self, id: TabId<T>) -> Result<(), TabRegistryError> {
        self.entries
            .contains_key(&id)
            .then_some(())
            .ok_or(TabRegistryError::UnknownTab)
    }

    fn position(&self, id: TabId<T>) -> Option<usize> {
        self.order.iter().position(|candidate| *candidate == id)
    }

    fn active_after_close(&self, position: usize) -> Option<TabId<T>> {
        self.order.get(position + 1).copied().or_else(|| {
            position
                .checked_sub(1)
                .and_then(|index| self.order.get(index).copied())
        })
    }

    fn cycle(&mut self, reverse: bool) -> Option<TabId<T>> {
        if self.order.is_empty() {
            return None;
        }
        let current = self
            .active
            .and_then(|id| self.position(id))
            .unwrap_or_else(|| if reverse { 0 } else { self.order.len() - 1 });
        let next = if reverse {
            current.checked_sub(1).unwrap_or(self.order.len() - 1)
        } else {
            (current + 1) % self.order.len()
        };
        let id = self.order[next];
        self.active = Some(id);
        Some(id)
    }
}

impl<T> Default for TabRegistry<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Eq, PartialEq)]
    struct DemoTab(u8);

    fn tab(value: u8) -> DemoTab {
        DemoTab(value)
    }

    #[test]
    fn stable_ids_survive_close_and_later_add() {
        let mut tabs = TabRegistry::new();
        let first = tabs.add("Document", tab(1)).unwrap();
        let second = tabs.add("Document", tab(2)).unwrap();
        let third = tabs.add("Document", tab(3)).unwrap();
        tabs.close(second).unwrap();
        let fourth = tabs.add("Document", tab(4)).unwrap();

        assert_eq!(first.get(), 1);
        assert_eq!(third.get(), 3);
        assert_eq!(fourth.get(), 4);
        assert_eq!(tabs.get(first).unwrap().value(), &tab(1));
        assert_eq!(
            tabs.tabs().map(|(id, _)| id).collect::<Vec<_>>(),
            vec![first, third, fourth]
        );
    }

    #[test]
    fn duplicate_titles_have_distinct_ids() {
        let mut tabs = TabRegistry::new();
        let first = tabs.add("Document", tab(1)).unwrap();
        let second = tabs.add("Document", tab(2)).unwrap();
        let third = tabs.add("Document", tab(3)).unwrap();

        assert_ne!(first, second);
        assert_ne!(second, third);
        assert_eq!(
            tabs.tabs()
                .map(|(_, entry)| entry.title())
                .collect::<Vec<_>>(),
            vec!["Document", "Document", "Document"]
        );
    }

    #[test]
    fn activation_is_targeted_and_does_not_reorder() {
        let mut tabs = TabRegistry::new();
        let first = tabs.add("One", tab(1)).unwrap();
        let second = tabs.add("Two", tab(2)).unwrap();
        let third = tabs.add("Three", tab(3)).unwrap();
        tabs.activate(first).unwrap();

        assert_eq!(tabs.active(), Some(first));
        assert_eq!(
            tabs.tabs().map(|(id, _)| id).collect::<Vec<_>>(),
            vec![first, second, third]
        );
    }

    #[test]
    fn stale_ids_are_safe() {
        let mut tabs = TabRegistry::new();
        let first = tabs.add("One", tab(1)).unwrap();
        let stale = tabs.add("Two", tab(2)).unwrap();
        tabs.close(stale).unwrap();
        let active_before = tabs.active();

        assert_eq!(tabs.activate(stale), Err(TabRegistryError::UnknownTab));
        assert!(matches!(
            tabs.close(stale),
            Err(TabRegistryError::UnknownTab)
        ));
        assert_eq!(tabs.active(), active_before);
        assert_eq!(
            tabs.tabs().map(|(id, _)| id).collect::<Vec<_>>(),
            vec![first]
        );
    }

    #[test]
    fn closing_inactive_middle_preserves_active_and_order() {
        let mut tabs = TabRegistry::new();
        let first = tabs.add("One", tab(1)).unwrap();
        let middle = tabs.add("Two", tab(2)).unwrap();
        let last = tabs.add("Three", tab(3)).unwrap();
        tabs.activate(first).unwrap();
        tabs.close(middle).unwrap();

        assert_eq!(tabs.active(), Some(first));
        assert_eq!(
            tabs.tabs().map(|(id, _)| id).collect::<Vec<_>>(),
            vec![first, last]
        );
    }

    #[test]
    fn closing_active_prefers_right_then_left_then_empty() {
        let mut tabs = TabRegistry::new();
        let first = tabs.add("One", tab(1)).unwrap();
        let middle = tabs.add("Two", tab(2)).unwrap();
        let last = tabs.add("Three", tab(3)).unwrap();

        tabs.activate(middle).unwrap();
        tabs.close(middle).unwrap();
        assert_eq!(tabs.active(), Some(last));
        tabs.close(last).unwrap();
        assert_eq!(tabs.active(), Some(first));
        tabs.close(first).unwrap();
        assert_eq!(tabs.active(), None);
        assert!(tabs.is_empty());
    }

    #[test]
    fn next_and_previous_wrap() {
        let mut tabs = TabRegistry::new();
        let first = tabs.add("One", tab(1)).unwrap();
        let second = tabs.add("Two", tab(2)).unwrap();
        let third = tabs.add("Three", tab(3)).unwrap();

        tabs.activate(third).unwrap();
        assert_eq!(tabs.next(), Some(first));
        assert_eq!(tabs.previous(), Some(third));
        tabs.activate(second).unwrap();
        assert_eq!(tabs.next(), Some(third));
        assert_eq!(tabs.previous(), Some(second));
    }

    #[test]
    fn empty_registry_has_no_selection_or_cycle_target() {
        let mut tabs = TabRegistry::<DemoTab>::new();
        assert!(tabs.is_empty());
        assert_eq!(tabs.len(), 0);
        assert_eq!(tabs.active(), None);
        assert_eq!(tabs.next(), None);
        assert_eq!(tabs.previous(), None);
        assert!(tabs.close_active().unwrap().is_none());
    }

    #[test]
    fn new_tabs_become_active_and_start_clean() {
        let mut tabs = TabRegistry::new();
        let first = tabs.add("One", tab(1)).unwrap();
        let second = tabs.add("Two", tab(2)).unwrap();

        assert_eq!(tabs.active(), Some(second));
        assert!(!tabs.get(first).unwrap().is_dirty());
        assert!(!tabs.get(second).unwrap().is_dirty());
    }

    #[test]
    fn dirty_flag_is_per_tab_and_survives_selection_changes() {
        let mut tabs = TabRegistry::new();
        let first = tabs.add("One", tab(1)).unwrap();
        let second = tabs.add("Two", tab(2)).unwrap();
        tabs.set_dirty(first, true).unwrap();
        tabs.activate(first).unwrap();
        tabs.activate(second).unwrap();

        assert!(tabs.get(first).unwrap().is_dirty());
        assert!(!tabs.get(second).unwrap().is_dirty());
    }

    #[test]
    fn permanent_tab_rejects_close_without_changing_selection() {
        let mut tabs = TabRegistry::new();
        let permanent = tabs
            .add_with_options(
                "Home",
                tab(1),
                TabOptions {
                    closable: false,
                    ..TabOptions::default()
                },
            )
            .unwrap();
        let active_before = tabs.active();

        assert!(matches!(
            tabs.close(permanent),
            Err(TabRegistryError::TabNotClosable)
        ));
        assert_eq!(tabs.active(), active_before);
        assert_eq!(tabs.len(), 1);
    }

    #[test]
    fn raw_id_lookup_only_resolves_live_tabs() {
        let mut tabs = TabRegistry::new();
        let first = tabs.add("One", tab(1)).unwrap();
        let second = tabs.add("Two", tab(2)).unwrap();
        tabs.close(second).unwrap();

        assert_eq!(tabs.id_from_raw(first.get()), Some(first));
        assert_eq!(tabs.id_from_raw(second.get()), None);
    }
}
