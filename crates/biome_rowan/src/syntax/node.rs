use crate::green::GreenElement;
use crate::syntax::SyntaxTrivia;
use crate::syntax::element::{SyntaxElement, SyntaxElementKey};
use crate::{
    Direction, GreenNode, Language, NodeOrToken, SyntaxKind, SyntaxList, SyntaxNodeText,
    SyntaxToken, SyntaxTriviaPiece, TokenAtOffset, WalkEvent, cursor,
};
use biome_text_size::{TextRange, TextSize};
#[cfg(feature = "serde")]
use serde::Serialize;
use std::any::TypeId;
use std::fmt::{Debug, Formatter};
use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::{fmt, ops};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SyntaxNode<L: Language> {
    raw: cursor::SyntaxNode,
    _p: PhantomData<L>,
}

impl<L: Language> SyntaxNode<L> {
    pub(crate) fn new_root(green: GreenNode) -> Self {
        Self::from(cursor::SyntaxNode::new_root(green))
    }

    /// Create a new detached (root) node from a syntax kind and an iterator of slots
    ///
    /// In general this function should not be used directly but through the
    /// type-checked factory function / builders generated from the grammar of
    /// the corresponding language (eg. `biome_js_factory::make`)
    pub fn new_detached<I>(kind: L::Kind, slots: I) -> Self
    where
        I: IntoIterator<Item = Option<SyntaxElement<L>>>,
        I::IntoIter: ExactSizeIterator,
    {
        Self::from(cursor::SyntaxNode::new_root(GreenNode::new(
            kind.to_raw(),
            slots.into_iter().map(|slot| {
                slot.map(|element| match element {
                    NodeOrToken::Node(node) => GreenElement::Node(node.green_node()),
                    NodeOrToken::Token(token) => GreenElement::Token(token.green_token()),
                })
            }),
        )))
    }

    fn green_node(&self) -> GreenNode {
        self.raw.green().to_owned()
    }

    pub fn key(&self) -> SyntaxElementKey {
        let (node_data, offset) = self.raw.key();
        SyntaxElementKey::new(node_data, offset)
    }

    /// Returns the element stored in the slot with the given index. Returns [None] if the slot is empty.
    ///
    /// ## Panics
    /// If the slot index is out of bounds
    #[inline]
    pub fn element_in_slot(&self, slot: u32) -> Option<SyntaxElement<L>> {
        self.raw.element_in_slot(slot).map(SyntaxElement::from)
    }

    pub fn kind(&self) -> L::Kind {
        L::Kind::from_raw(self.raw.kind())
    }

    /// Returns the text of all descendants tokens combined, including all trivia.
    ///
    /// ```
    /// use biome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use biome_rowan::*;
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t let \t\t",
    ///         &[TriviaPiece::whitespace(3)],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    ///     builder.token(RawLanguageKind::STRING_TOKEN, "a");
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::SEMICOLON_TOKEN,
    ///         "; \t\t",
    ///         &[TriviaPiece::whitespace(3)],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    /// });
    /// assert_eq!("\n\t let \t\ta; \t\t", node.text_with_trivia());
    /// ```
    pub fn text_with_trivia(&self) -> SyntaxNodeText {
        self.raw.text()
    }

    /// Returns the text of all descendants tokens combined,
    /// excluding the first token leading trivia, and the last token trailing trivia.
    /// All other trivia is included.
    ///
    /// ```
    /// use biome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use biome_rowan::*;
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t let \t\t",
    ///         &[TriviaPiece::whitespace(3)],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    ///     builder.token(RawLanguageKind::STRING_TOKEN, "a");
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::SEMICOLON_TOKEN,
    ///         "; \t\t",
    ///         &[],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    /// });
    /// assert_eq!("let \t\ta;", node.text_trimmed());
    /// ```
    pub fn text_trimmed(&self) -> SyntaxNodeText {
        self.raw.text_trimmed()
    }

    /// Returns the range corresponding for the text of all descendants tokens combined, including all trivia.
    ///
    /// ```
    /// use biome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use biome_rowan::*;
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t let \t\t",
    ///         &[TriviaPiece::whitespace(3)],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    ///     builder.token(RawLanguageKind::STRING_TOKEN, "a");
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::SEMICOLON_TOKEN,
    ///         "; \t\t",
    ///         &[],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    /// });
    /// let range = node.text_range_with_trivia();
    /// assert_eq!(0u32, u32::from(range.start()));
    /// assert_eq!(14u32, u32::from(range.end()));
    /// ```
    pub fn text_range_with_trivia(&self) -> TextRange {
        self.raw.text_range()
    }

    /// Returns the range corresponding for the text of all descendants tokens combined,
    /// excluding the first token leading  trivia, and the last token trailing trivia.
    /// All other trivia is included.
    ///
    /// ```
    /// use biome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use biome_rowan::*;
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t let \t\t",
    ///         &[TriviaPiece::whitespace(3)],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    ///     builder.token(RawLanguageKind::STRING_TOKEN, "a");
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::SEMICOLON_TOKEN,
    ///         "; \t\t",
    ///         &[],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    /// });
    /// let range = node.text_trimmed_range();
    /// assert_eq!(3u32, u32::from(range.start()));
    /// assert_eq!(11u32, u32::from(range.end()));
    /// ```
    pub fn text_trimmed_range(&self) -> TextRange {
        self.raw.text_trimmed_range()
    }

    /// Returns the leading trivia of the [first_token](SyntaxNode::first_token), or [None] if the node does not have any descendant tokens.
    ///
    /// ```
    /// use biome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use biome_rowan::*;
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t let \t\t",
    ///         &[TriviaPiece::whitespace(3)],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    ///     builder.token(RawLanguageKind::STRING_TOKEN, "a");
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::SEMICOLON_TOKEN,
    ///         "; \t\t",
    ///         &[],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    /// });
    /// let trivia = node.first_leading_trivia();
    /// assert!(trivia.is_some());
    /// assert_eq!("\n\t ", trivia.unwrap().text());
    ///
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {});
    /// let trivia = node.first_leading_trivia();
    /// assert!(trivia.is_none());
    /// ```
    pub fn first_leading_trivia(&self) -> Option<SyntaxTrivia<L>> {
        self.raw.first_leading_trivia().map(SyntaxTrivia::new)
    }

    /// Returns the trailing trivia of the  [last_token](SyntaxNode::last_token), or [None] if the node does not have any descendant tokens.
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use biome_rowan::*;
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t let \t\t",
    ///         &[TriviaPiece::whitespace(3)],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    ///     builder.token(RawLanguageKind::STRING_TOKEN, "a");
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::SEMICOLON_TOKEN,
    ///         "; \t\t",
    ///         &[],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    /// });
    /// let trivia = node.last_trailing_trivia();
    /// assert!(trivia.is_some());
    /// assert_eq!(" \t\t", trivia.unwrap().text());
    ///
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {});
    /// let trivia = node.last_trailing_trivia();
    /// assert!(trivia.is_none());
    /// ```
    pub fn last_trailing_trivia(&self) -> Option<SyntaxTrivia<L>> {
        self.raw.last_trailing_trivia().map(SyntaxTrivia::new)
    }

    pub fn parent(&self) -> Option<Self> {
        self.raw.parent().map(Self::from)
    }

    /// Returns the grandparent.
    pub fn grand_parent(&self) -> Option<Self> {
        self.parent().and_then(|parent| parent.parent())
    }

    /// Returns the index of this node inside of its parent
    #[inline]
    pub fn index(&self) -> usize {
        self.raw.index()
    }

    pub fn ancestors(&self) -> impl Iterator<Item = Self> + use<L> {
        self.raw.ancestors().map(Self::from)
    }

    pub fn children(&self) -> SyntaxNodeChildren<L> {
        SyntaxNodeChildren {
            raw: self.raw.children(),
            _p: PhantomData,
        }
    }

    /// Returns an iterator over all the slots of this syntax node.
    pub fn slots(&self) -> SyntaxSlots<L> {
        SyntaxSlots {
            raw: self.raw.slots(),
            _p: PhantomData,
        }
    }

    pub fn children_with_tokens(&self) -> SyntaxElementChildren<L> {
        SyntaxElementChildren {
            raw: self.raw.children_with_tokens(),
            _p: PhantomData,
        }
    }

    pub fn tokens(&self) -> impl DoubleEndedIterator<Item = SyntaxToken<L>> + '_ {
        self.raw.tokens().map(SyntaxToken::from)
    }

    pub fn first_child(&self) -> Option<Self> {
        self.raw.first_child().map(Self::from)
    }
    pub fn last_child(&self) -> Option<Self> {
        self.raw.last_child().map(Self::from)
    }

    pub fn first_child_or_token(&self) -> Option<SyntaxElement<L>> {
        self.raw.first_child_or_token().map(NodeOrToken::from)
    }
    pub fn last_child_or_token(&self) -> Option<SyntaxElement<L>> {
        self.raw.last_child_or_token().map(NodeOrToken::from)
    }

    pub fn next_sibling(&self) -> Option<Self> {
        self.raw.next_sibling().map(Self::from)
    }
    pub fn prev_sibling(&self) -> Option<Self> {
        self.raw.prev_sibling().map(Self::from)
    }

    pub fn next_sibling_or_token(&self) -> Option<SyntaxElement<L>> {
        self.raw.next_sibling_or_token().map(NodeOrToken::from)
    }
    pub fn prev_sibling_or_token(&self) -> Option<SyntaxElement<L>> {
        self.raw.prev_sibling_or_token().map(NodeOrToken::from)
    }

    /// Return the leftmost token in the subtree of this node.
    pub fn first_token(&self) -> Option<SyntaxToken<L>> {
        self.raw.first_token().map(SyntaxToken::from)
    }
    /// Return the rightmost token in the subtree of this node.
    pub fn last_token(&self) -> Option<SyntaxToken<L>> {
        self.raw.last_token().map(SyntaxToken::from)
    }

    pub fn siblings(&self, direction: Direction) -> impl Iterator<Item = Self> + use<L> {
        self.raw.siblings(direction).map(Self::from)
    }

    pub fn siblings_with_tokens(
        &self,
        direction: Direction,
    ) -> impl Iterator<Item = SyntaxElement<L>> + use<L> {
        self.raw
            .siblings_with_tokens(direction)
            .map(SyntaxElement::from)
    }

    pub fn descendants(&self) -> impl Iterator<Item = Self> + use<L> {
        self.raw.descendants().map(Self::from)
    }

    pub fn descendants_tokens(
        &self,
        direction: Direction,
    ) -> impl Iterator<Item = SyntaxToken<L>> + use<L> {
        self.descendants_with_tokens(direction)
            .filter_map(|x| x.as_token().cloned())
    }

    pub fn descendants_with_tokens(
        &self,
        direction: Direction,
    ) -> impl Iterator<Item = SyntaxElement<L>> + use<L> {
        self.raw
            .descendants_with_tokens(direction)
            .map(NodeOrToken::from)
    }

    /// Traverse the subtree rooted at the current node (including the current
    /// node) in preorder, excluding tokens.
    pub fn preorder(&self) -> Preorder<L> {
        Preorder {
            raw: self.raw.preorder(),
            _p: PhantomData,
        }
    }

    /// Traverse the subtree rooted at the current node (including the current
    /// node) in preorder, including tokens.
    pub fn preorder_with_tokens(&self, direction: Direction) -> PreorderWithTokens<L> {
        PreorderWithTokens {
            raw: self.raw.preorder_with_tokens(direction),
            _p: PhantomData,
        }
    }

    pub fn preorder_tokens(&self, direction: Direction) -> PreorderTokens<L> {
        PreorderTokens {
            raw: self.raw.preorder_tokens(direction),
            _p: PhantomData,
        }
    }

    /// Find a token in the subtree corresponding to this node, which covers the offset.
    /// Precondition: offset must be withing node's range.
    pub fn token_at_offset(&self, offset: TextSize) -> TokenAtOffset<SyntaxToken<L>> {
        self.raw.token_at_offset(offset).map(SyntaxToken::from)
    }

    /// Return the deepest node or token in the current subtree that fully
    /// contains the range. If the range is empty and is contained in two leaf
    /// nodes, either one can be returned. Precondition: range must be contained
    /// withing the current node
    pub fn covering_element(&self, range: TextRange) -> SyntaxElement<L> {
        NodeOrToken::from(self.raw.covering_element(range))
    }

    /// Finds a [`SyntaxElement`] which intersects with a given `range`. If
    /// there are several intersecting elements, any one can be returned.
    ///
    /// The method uses binary search internally, so it's complexity is
    /// `O(log(N))` where `N = self.children_with_tokens().count()`.
    pub fn child_or_token_at_range(&self, range: TextRange) -> Option<SyntaxElement<L>> {
        self.raw
            .child_or_token_at_range(range)
            .map(SyntaxElement::from)
    }

    /// Returns an independent copy of the subtree rooted at this node.
    ///
    /// The parent of the returned node will be `None`, the start offset will be
    /// zero, but, otherwise, it'll be equivalent to the source node.
    pub fn clone_subtree(&self) -> Self {
        Self::from(self.raw.clone_subtree())
    }

    /// Return a new version of this node detached from its parent node
    #[must_use = "syntax elements are immutable, the result of update methods must be propagated to have any effect"]
    pub fn detach(self) -> Self {
        Self {
            raw: self.raw.detach(),
            _p: PhantomData,
        }
    }

    /// Return a clone of this node with the specified range of slots replaced
    /// with the elements of the provided iterator
    #[must_use = "syntax elements are immutable, the result of update methods must be propagated to have any effect"]
    pub fn splice_slots<R, I>(self, range: R, replace_with: I) -> Self
    where
        R: ops::RangeBounds<usize>,
        I: IntoIterator<Item = Option<SyntaxElement<L>>>,
    {
        Self {
            raw: self.raw.splice_slots(
                range,
                replace_with
                    .into_iter()
                    .map(|element| element.map(cursor::SyntaxElement::from)),
            ),
            _p: PhantomData,
        }
    }

    /// Return a new version of this node with the element `prev_elem` replaced with `next_elem`
    ///
    /// `prev_elem` can be a direct child of this node, or an indirect child through any descendant node
    ///
    /// Returns `None` if `prev_elem` is not a descendant of this node
    #[must_use = "syntax elements are immutable, the result of update methods must be propagated to have any effect"]
    pub fn replace_child(
        self,
        prev_elem: SyntaxElement<L>,
        next_elem: SyntaxElement<L>,
    ) -> Option<Self> {
        Some(Self {
            raw: self.raw.replace_child(prev_elem.into(), next_elem.into())?,
            _p: PhantomData,
        })
    }

    /// Return a new version of this node with the leading trivia of its first token replaced with `trivia`.
    #[must_use = "syntax elements are immutable, the result of update methods must be propagated to have any effect"]
    pub fn with_leading_trivia_pieces<I>(self, trivia: I) -> Option<Self>
    where
        I: IntoIterator<Item = SyntaxTriviaPiece<L>>,
        I::IntoIter: ExactSizeIterator,
    {
        let first_token = self.first_token()?;
        let new_first_token = first_token.with_leading_trivia_pieces(trivia);
        self.replace_child(first_token.into(), new_first_token.into())
    }

    /// Return a new version of this node with the trailing trivia of its last token replaced with `trivia`.
    #[must_use = "syntax elements are immutable, the result of update methods must be propagated to have any effect"]
    pub fn with_trailing_trivia_pieces<I>(self, trivia: I) -> Option<Self>
    where
        I: IntoIterator<Item = SyntaxTriviaPiece<L>>,
        I::IntoIter: ExactSizeIterator,
    {
        let last_token = self.last_token()?;
        let new_last_token = last_token.with_trailing_trivia_pieces(trivia);
        self.replace_child(last_token.into(), new_last_token.into())
    }

    // Return a new version of this node with `trivia` prepended to the leading trivia of the first token.
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use biome_rowan::*;
    ///
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\t let ",
    ///         &[TriviaPiece::whitespace(2)],
    ///         &[TriviaPiece::whitespace(1)],
    ///     );
    ///     builder.token(RawLanguageKind::STRING_TOKEN, "a");
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::SEMICOLON_TOKEN,
    ///         "; ",
    ///         &[],
    ///         &[TriviaPiece::whitespace(1)],
    ///     );
    /// });
    ///
    /// let new_node = node.clone().prepend_trivia_pieces(node.last_trailing_trivia().unwrap().pieces()).unwrap();
    /// let leading_trivia = new_node.first_leading_trivia().unwrap();
    /// let trailing_trivia = new_node.last_trailing_trivia().unwrap();
    ///
    /// assert_eq!(" \t ", leading_trivia.text());
    /// assert_eq!(" ", trailing_trivia.text());
    /// ```
    #[must_use = "syntax elements are immutable, the result of update methods must be propagated to have any effect"]
    pub fn prepend_trivia_pieces<I>(self, trivia: I) -> Option<Self>
    where
        I: IntoIterator<Item = SyntaxTriviaPiece<L>>,
        I::IntoIter: ExactSizeIterator,
    {
        let first_token = self.first_token()?;
        let new_first_token = first_token.prepend_trivia_pieces(trivia);
        self.replace_child(first_token.into(), new_first_token.into())
    }

    // Return a new version of this node with `trivia` appended to the trailing trivia of the last token.
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use biome_rowan::*;
    ///
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\t let ",
    ///         &[TriviaPiece::whitespace(2)],
    ///         &[TriviaPiece::whitespace(1)],
    ///     );
    ///     builder.token(RawLanguageKind::STRING_TOKEN, "a");
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::SEMICOLON_TOKEN,
    ///         "; ",
    ///         &[],
    ///         &[TriviaPiece::whitespace(1)],
    ///     );
    /// });
    ///
    /// let new_node = node.clone().append_trivia_pieces(node.first_leading_trivia().unwrap().pieces()).unwrap();
    /// let leading_trivia = new_node.first_leading_trivia().unwrap();
    /// let trailing_trivia = new_node.last_trailing_trivia().unwrap();
    ///
    /// assert_eq!("\t ", leading_trivia.text());
    /// assert_eq!(" \t ", trailing_trivia.text());
    /// ```
    #[must_use = "syntax elements are immutable, the result of update methods must be propagated to have any effect"]
    pub fn append_trivia_pieces<I>(self, trivia: I) -> Option<Self>
    where
        I: IntoIterator<Item = SyntaxTriviaPiece<L>>,
        I::IntoIter: ExactSizeIterator,
    {
        let last_token = self.last_token()?;
        let new_last_token = last_token.append_trivia_pieces(trivia);
        self.replace_child(last_token.into(), new_last_token.into())
    }

    /// Return a new version of this node without leading and trailing newlines and whitespaces.
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use biome_rowan::*;
    ///
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n let ",
    ///         &[TriviaPiece::newline(1), TriviaPiece::whitespace(1)],
    ///         &[TriviaPiece::whitespace(1)],
    ///     );
    ///     builder.token(RawLanguageKind::STRING_TOKEN, "a");
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::SEMICOLON_TOKEN,
    ///         "; ",
    ///         &[],
    ///         &[TriviaPiece::whitespace(1)],
    ///     );
    /// });
    ///
    /// let new_node = node.trim_trivia().unwrap();
    /// let leading_trivia = new_node.first_leading_trivia().unwrap();
    /// let trailing_trivia = new_node.last_trailing_trivia().unwrap();
    ///
    /// assert_eq!("", leading_trivia.text());
    /// assert_eq!("", trailing_trivia.text());
    /// ```
    #[must_use = "syntax elements are immutable, the result of update methods must be propagated to have any effect"]
    pub fn trim_trivia(self) -> Option<Self> {
        self.trim_leading_trivia()?.trim_trailing_trivia()
    }

    /// Return a new version of this node without leading newlines and whitespaces.
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use biome_rowan::*;
    ///
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n let ",
    ///         &[TriviaPiece::newline(1), TriviaPiece::whitespace(1)],
    ///         &[TriviaPiece::whitespace(1)],
    ///     );
    ///     builder.token(RawLanguageKind::STRING_TOKEN, "a");
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::SEMICOLON_TOKEN,
    ///         "; ",
    ///         &[],
    ///         &[TriviaPiece::whitespace(1)],
    ///     );
    /// });
    ///
    /// let new_node = node.trim_leading_trivia().unwrap();
    /// let leading_trivia = new_node.first_leading_trivia().unwrap();
    /// let trailing_trivia = new_node.last_trailing_trivia().unwrap();
    ///
    /// assert_eq!("", leading_trivia.text());
    /// assert_eq!(" ", trailing_trivia.text());
    /// ```
    #[must_use = "syntax elements are immutable, the result of update methods must be propagated to have any effect"]
    pub fn trim_leading_trivia(self) -> Option<Self> {
        let first_token = self.first_token()?;
        let new_first_token = first_token.trim_leading_trivia();
        self.replace_child(first_token.into(), new_first_token.into())
    }

    /// Return a new version of this token without trailing whitespaces.
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use biome_rowan::*;
    ///
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n let ",
    ///         &[TriviaPiece::newline(1), TriviaPiece::whitespace(1)],
    ///         &[TriviaPiece::whitespace(1)],
    ///     );
    ///     builder.token(RawLanguageKind::STRING_TOKEN, "a");
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::SEMICOLON_TOKEN,
    ///         "; ",
    ///         &[],
    ///         &[TriviaPiece::whitespace(1)],
    ///     );
    /// });
    ///
    /// let new_node = node.trim_trailing_trivia().unwrap();
    /// let leading_trivia = new_node.first_leading_trivia().unwrap();
    /// let trailing_trivia = new_node.last_trailing_trivia().unwrap();
    ///
    /// assert_eq!("\n ", leading_trivia.text());
    /// assert_eq!("", trailing_trivia.text());
    /// ```
    #[must_use = "syntax elements are immutable, the result of update methods must be propagated to have any effect"]
    pub fn trim_trailing_trivia(self) -> Option<Self> {
        let last_token = self.last_token()?;
        let new_last_token = last_token.trim_trailing_trivia();
        self.replace_child(last_token.into(), new_last_token.into())
    }

    pub fn into_list(self) -> SyntaxList<L> {
        SyntaxList::new(self)
    }

    /// Whether the node contains any comments. This function checks
    /// **all the descendants** of the current node.
    pub fn has_comments_descendants(&self) -> bool {
        self.descendants_tokens(Direction::Next)
            .any(|tok| tok.has_trailing_comments() || tok.has_leading_comments())
    }

    /// It checks if the current node has trailing or leading trivia
    pub fn has_comments_direct(&self) -> bool {
        self.has_trailing_comments() || self.has_leading_comments()
    }

    /// It checks if the current node has comments at the edges:
    /// if first or last tokens contain comments (leading or trailing)
    pub fn first_or_last_token_have_comments(&self) -> bool {
        self.first_token_has_comments() || self.last_token_has_comments()
    }

    /// Whether the node contains trailing comments.
    pub fn has_trailing_comments(&self) -> bool {
        self.last_token()
            .is_some_and(|tok| tok.has_trailing_comments())
    }

    /// Whether the last token of a node has comments (leading or trailing)
    pub fn last_token_has_comments(&self) -> bool {
        self.last_token()
            .is_some_and(|tok| tok.has_trailing_comments() || tok.has_leading_comments())
    }

    /// Whether the first token of a node has comments (leading or trailing)
    pub fn first_token_has_comments(&self) -> bool {
        self.first_token()
            .is_some_and(|tok| tok.has_trailing_comments() || tok.has_leading_comments())
    }

    /// Whether the node contains leading comments.
    pub fn has_leading_comments(&self) -> bool {
        self.first_token()
            .is_some_and(|tok| tok.has_leading_comments())
    }

    /// Whether the node contains leading newlines.
    pub fn has_leading_newline(&self) -> bool {
        self.first_token()
            .is_some_and(|tok| tok.has_leading_newline())
    }
}

impl<L> SyntaxNode<L>
where
    L: Language + 'static,
{
    /// Create a [Send] + [Sync] handle to this node
    ///
    /// Returns `None` if self is not a root node
    pub fn as_send(&self) -> Option<SendNode> {
        if self.parent().is_none() {
            Some(SendNode {
                language: TypeId::of::<L>(),
                green: self.green_node(),
            })
        } else {
            None
        }
    }
}

impl<L: Language> fmt::Debug for SyntaxNode<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            let mut level = 0;
            for event in self.raw.preorder_slots() {
                match event {
                    WalkEvent::Enter(element) => {
                        for _ in 0..level {
                            write!(f, "  ")?;
                        }
                        match element {
                            cursor::SyntaxSlot::Node(node) => {
                                writeln!(f, "{}: {:?}", node.index(), Self::from(node))?
                            }
                            cursor::SyntaxSlot::Token(token) => writeln!(
                                f,
                                "{}: {:?}",
                                token.index(),
                                SyntaxToken::<L>::from(token)
                            )?,
                            cursor::SyntaxSlot::Empty { index, .. } => {
                                writeln!(f, "{index}: (empty)")?
                            }
                        }
                        level += 1;
                    }
                    WalkEvent::Leave(_) => level -= 1,
                }
            }
            assert_eq!(level, 0);
            Ok(())
        } else {
            write!(f, "{:?}@{:?}", self.kind(), self.text_range_with_trivia())
        }
    }
}

impl<L: Language> fmt::Display for SyntaxNode<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.raw, f)
    }
}

impl<L: Language> From<SyntaxNode<L>> for cursor::SyntaxNode {
    fn from(node: SyntaxNode<L>) -> Self {
        node.raw
    }
}

impl<L: Language> From<cursor::SyntaxNode> for SyntaxNode<L> {
    fn from(raw: cursor::SyntaxNode) -> Self {
        Self {
            raw,
            _p: PhantomData,
        }
    }
}

/// Language-agnostic representation of the root node of a syntax tree, can be
/// sent or shared between threads
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendNode {
    language: TypeId,
    green: GreenNode,
}

impl SendNode {
    /// Downcast this handle back into a [SyntaxNode]
    ///
    /// Returns `None` if the specified language `L` is not the one this node
    /// was created with
    pub fn into_node<L>(self) -> Option<SyntaxNode<L>>
    where
        L: Language + 'static,
    {
        if TypeId::of::<L>() == self.language {
            Some(SyntaxNode::new_root(self.green))
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EmbeddedSendNode {
    offset: TextSize,
    green: GreenNode,
}

impl EmbeddedSendNode {
    /// Downcast this handle into a [SyntaxNodeWithOffset]
    pub fn into_node<L>(self) -> SyntaxNodeWithOffset<L>
    where
        L: Language + 'static,
    {
        SyntaxNodeWithOffset {
            node: SyntaxNode::new_root(self.green),
            offset: self.offset,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SyntaxNodeChildren<L: Language> {
    raw: cursor::SyntaxNodeChildren,
    _p: PhantomData<L>,
}

impl<L: Language> Iterator for SyntaxNodeChildren<L> {
    type Item = SyntaxNode<L>;
    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(SyntaxNode::from)
    }
}

#[derive(Clone)]
pub struct SyntaxElementChildren<L: Language> {
    raw: cursor::SyntaxElementChildren,
    _p: PhantomData<L>,
}

impl<L: Language> Debug for SyntaxElementChildren<L> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<L: Language> Default for SyntaxElementChildren<L> {
    fn default() -> Self {
        Self {
            raw: cursor::SyntaxElementChildren::default(),
            _p: PhantomData,
        }
    }
}

impl<L: Language> Iterator for SyntaxElementChildren<L> {
    type Item = SyntaxElement<L>;
    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(NodeOrToken::from)
    }
}

pub struct Preorder<L: Language> {
    raw: cursor::Preorder,
    _p: PhantomData<L>,
}

impl<L: Language> Preorder<L> {
    pub fn skip_subtree(&mut self) {
        self.raw.skip_subtree()
    }
}

impl<L: Language> Iterator for Preorder<L> {
    type Item = WalkEvent<SyntaxNode<L>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(|it| it.map(SyntaxNode::from))
    }
}

pub struct PreorderTokens<L: Language> {
    raw: cursor::PreorderTokens,
    _p: PhantomData<L>,
}

impl<L: Language> Iterator for PreorderTokens<L> {
    type Item = SyntaxToken<L>;
    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(SyntaxToken::from)
    }
}

pub struct PreorderWithTokens<L: Language> {
    raw: cursor::PreorderWithTokens,
    _p: PhantomData<L>,
}

impl<L: Language> PreorderWithTokens<L> {
    pub fn skip_subtree(&mut self) {
        self.raw.skip_subtree()
    }
}

impl<L: Language> Iterator for PreorderWithTokens<L> {
    type Item = WalkEvent<SyntaxElement<L>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(|it| it.map(SyntaxElement::from))
    }
}

/// Each node has a slot for each of its children regardless if the child is present or not.
/// A child that isn't present either because it's optional or because of a syntax error
/// is stored in an [SyntaxSlot::Empty] to preserve the index of each child.
#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum SyntaxSlot<L: Language> {
    /// Slot that stores a node child
    Node(SyntaxNode<L>),
    /// Slot that stores a token child
    Token(SyntaxToken<L>),
    /// Slot that marks that the child in this position isn't present in the source code.
    Empty { index: u32 },
}

impl<L: Language> SyntaxSlot<L> {
    pub fn into_node(self) -> Option<SyntaxNode<L>> {
        match self {
            Self::Node(node) => Some(node),
            _ => None,
        }
    }

    pub fn into_token(self) -> Option<SyntaxToken<L>> {
        match self {
            Self::Token(token) => Some(token),
            _ => None,
        }
    }

    pub fn into_syntax_element(self) -> Option<SyntaxElement<L>> {
        match self {
            Self::Node(node) => Some(SyntaxElement::Node(node)),
            Self::Token(token) => Some(SyntaxElement::Token(token)),
            _ => None,
        }
    }

    pub fn kind(&self) -> Option<L::Kind> {
        match self {
            Self::Node(node) => Some(node.kind()),
            Self::Token(token) => Some(token.kind()),
            Self::Empty { .. } => None,
        }
    }
}

impl<L: Language> From<cursor::SyntaxSlot> for SyntaxSlot<L> {
    fn from(raw: cursor::SyntaxSlot) -> Self {
        match raw {
            cursor::SyntaxSlot::Node(node) => Self::Node(node.into()),
            cursor::SyntaxSlot::Token(token) => Self::Token(token.into()),
            cursor::SyntaxSlot::Empty { index, .. } => Self::Empty { index },
        }
    }
}

/// Iterator over the slots of a node.
#[derive(Debug, Clone)]
pub struct SyntaxSlots<L> {
    raw: cursor::SyntaxSlots,
    _p: PhantomData<L>,
}

impl<L: Language> Iterator for SyntaxSlots<L> {
    type Item = SyntaxSlot<L>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(SyntaxSlot::from)
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.raw.size_hint()
    }

    #[inline]
    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.raw.last().map(SyntaxSlot::from)
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.raw.nth(n).map(SyntaxSlot::from)
    }
}

impl<L: Language> FusedIterator for SyntaxSlots<L> {}

impl<L: Language> ExactSizeIterator for SyntaxSlots<L> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.raw.len()
    }
}

impl<L: Language> DoubleEndedIterator for SyntaxSlots<L> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.raw.next_back().map(SyntaxSlot::from)
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.raw.nth_back(n).map(SyntaxSlot::from)
    }
}

/// Trait with extension methods for `Option<SyntaxNode>`.
pub trait SyntaxNodeOptionExt<L: Language> {
    /// Returns the kind of the node if self is [Some], [None] otherwise.
    fn kind(&self) -> Option<L::Kind>;
}

impl<L: Language> SyntaxNodeOptionExt<L> for Option<&SyntaxNode<L>> {
    fn kind(&self) -> Option<L::Kind> {
        self.map(|node| node.kind())
    }
}

impl<L: Language> SyntaxNodeOptionExt<L> for Option<SyntaxNode<L>> {
    fn kind(&self) -> Option<L::Kind> {
        self.as_ref().kind()
    }
}

/// A syntax node that contains an offset
///
/// This allows embedded content to maintain correct source positions relative
/// to the parent document. For example, JavaScript code in an HTML `<script>` tag
/// starting at position 100 will have all its ranges offset by 100.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SyntaxNodeWithOffset<L: Language> {
    pub node: SyntaxNode<L>,
    pub offset: TextSize,
}

impl<L: Language> SyntaxNodeWithOffset<L> {
    pub fn new(node: SyntaxNode<L>, offset: TextSize) -> Self {
        Self { node, offset }
    }

    /// Returns the text range including all trivia, adjusted for base offset
    pub fn text_range_with_trivia(&self) -> TextRange {
        let range = self.node.text_range_with_trivia();
        TextRange::new(range.start() + self.offset, range.end() + self.offset)
    }

    /// Returns the trimmed text range, adjusted for base offset
    pub fn text_trimmed_range(&self) -> TextRange {
        let range = self.node.text_trimmed_range();
        TextRange::new(range.start() + self.offset, range.end() + self.offset)
    }

    /// Get the base offset applied to this node
    pub fn base_offset(&self) -> TextSize {
        self.offset
    }

    /// Get the underlying syntax node without offset
    pub fn inner(&self) -> &SyntaxNode<L> {
        &self.node
    }

    /// Convert back to the underlying syntax node, discarding offset information
    pub fn into_inner(self) -> SyntaxNode<L> {
        self.node
    }

    // Forward common methods from SyntaxNode
    pub fn kind(&self) -> L::Kind {
        self.node.kind()
    }

    pub fn text_with_trivia(&self) -> crate::SyntaxNodeText {
        self.node.text_with_trivia()
    }

    pub fn text_trimmed(&self) -> crate::SyntaxNodeText {
        self.node.text_trimmed()
    }

    /// Create a [Send] + [Sync] handle to this node.
    ///
    /// ### Panics
    ///
    /// It panics if the `base_offset` isn't greater than zero
    pub fn as_embedded_send(&self) -> EmbeddedSendNode {
        debug_assert!(self.offset > 0.into(), "range must be greater than 0");
        EmbeddedSendNode {
            green: self.node.green_node(),
            offset: self.offset,
        }
    }
}
