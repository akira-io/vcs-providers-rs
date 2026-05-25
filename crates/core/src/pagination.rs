use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PaginationBuilder;

impl PaginationBuilder {
    pub fn request(self) -> PageRequestBuilder {
        PageRequestBuilder::default()
    }

    pub fn page<T>(self, items: impl IntoIterator<Item = T>) -> PageBuilder<T> {
        PageBuilder {
            items: items.into_iter().collect(),
            next: None,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PageRequestBuilder {
    limit: Option<PageLimit>,
    cursor: Option<PageCursor>,
}

impl PageRequestBuilder {
    pub fn limit(mut self, limit: u16) -> Self {
        self.limit = Some(PageLimit::make(limit));
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(PageCursor::make(cursor));
        self
    }

    pub fn optional_cursor(mut self, cursor: Option<impl Into<String>>) -> Self {
        self.cursor = cursor.map(PageCursor::make);
        self
    }

    pub fn build(self) -> PageRequest {
        PageRequest {
            limit: self.limit,
            cursor: self.cursor,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PageRequest {
    limit: Option<PageLimit>,
    cursor: Option<PageCursor>,
}

impl PageRequest {
    pub fn limit(&self) -> Option<&PageLimit> {
        self.limit.as_ref()
    }

    pub fn cursor(&self) -> Option<&PageCursor> {
        self.cursor.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PageLimit(u16);

impl PageLimit {
    pub fn make(limit: u16) -> Self {
        Self(limit)
    }

    pub fn as_u16(&self) -> u16 {
        self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PageCursor(String);

impl PageCursor {
    pub fn make(cursor: impl Into<String>) -> Self {
        Self(cursor.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Page<T> {
    items: Vec<T>,
    next: Option<PageCursor>,
}

impl<T> Page<T> {
    pub fn make(items: Vec<T>) -> Self {
        Self { items, next: None }
    }

    pub fn items(&self) -> &[T] {
        &self.items
    }

    pub fn next(&self) -> Option<&PageCursor> {
        self.next.as_ref()
    }

    pub fn has_next(&self) -> bool {
        self.next.is_some()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PageBuilder<T> {
    items: Vec<T>,
    next: Option<PageCursor>,
}

impl<T> PageBuilder<T> {
    pub fn next(mut self, cursor: impl Into<String>) -> Self {
        self.next = Some(PageCursor::make(cursor));
        self
    }

    pub fn optional_next(mut self, cursor: Option<impl Into<String>>) -> Self {
        self.next = cursor.map(PageCursor::make);
        self
    }

    pub fn build(self) -> Page<T> {
        Page {
            items: self.items,
            next: self.next,
        }
    }
}
