//! Public traits.

use eyeball_im::{
    VectorDiff, VectorSubscriber, VectorSubscriberBatchedStream, VectorSubscriberStream,
};
use futures_core::Stream;
use imbl::Vector;

use super::{
    ops::{
        VecVectorDiffFamily, VectorDiffContainerFamily, VectorDiffContainerOps, VectorDiffFamily,
    },
    EmptyLimitStream, Filter, FilterMap, Limit,
};

/// Abstraction over stream items that the adapters in this module can deal
/// with.
pub trait VectorDiffContainer:
    VectorDiffContainerOps<Self::Element, Family = <Self as VectorDiffContainer>::Family>
{
    /// The element type of the [`Vector`][imbl::Vector] that diffs are being
    /// handled for.
    type Element: Clone + Send + Sync + 'static;

    #[doc(hidden)]
    type Family: VectorDiffContainerFamily<Member<Self::Element> = Self>;
}

impl<T: Clone + Send + Sync + 'static> VectorDiffContainer for VectorDiff<T> {
    type Element = T;
    type Family = VectorDiffFamily;
}

impl<T: Clone + Send + Sync + 'static> VectorDiffContainer for Vec<VectorDiff<T>> {
    type Element = T;
    type Family = VecVectorDiffFamily;
}

/// Extension trait for [`VectorSubscriber`].
pub trait VectorSubscriberExt<T> {
    /// Create a [`BatchedVectorSubscriber`] from `self`.
    fn batched(self) -> BatchedVectorSubscriber<T>;
}

impl<T> VectorSubscriberExt<T> for VectorSubscriber<T> {
    fn batched(self) -> BatchedVectorSubscriber<T> {
        BatchedVectorSubscriber { inner: self }
    }
}

/// A wrapper around [`VectorSubscriber`] with a different [`VectorObserver`]
/// impl.
#[derive(Debug)]
pub struct BatchedVectorSubscriber<T> {
    inner: VectorSubscriber<T>,
}

/// Abstraction over types that hold both a [`Vector`] and a stream of
/// [`VectorDiff`] updates.
///
/// See [`VectorObserverExt`] for operations available to implementers.
pub trait VectorObserver<T>: Sized {
    #[doc(hidden)]
    type Stream: Stream;

    #[doc(hidden)]
    fn into_parts(self) -> (Vector<T>, Self::Stream);
}

impl<T: Clone + Send + Sync + 'static> VectorObserver<T> for VectorSubscriber<T> {
    type Stream = VectorSubscriberStream<T>;

    fn into_parts(self) -> (Vector<T>, Self::Stream) {
        self.into_values_and_stream()
    }
}

impl<T: Clone + Send + Sync + 'static> VectorObserver<T> for BatchedVectorSubscriber<T> {
    type Stream = VectorSubscriberBatchedStream<T>;

    fn into_parts(self) -> (Vector<T>, Self::Stream) {
        self.inner.into_values_and_batched_stream()
    }
}

impl<T, S> VectorObserver<T> for (Vector<T>, S)
where
    S: Stream,
    S::Item: VectorDiffContainer,
{
    type Stream = S;

    fn into_parts(self) -> (Vector<T>, Self::Stream) {
        self
    }
}

/// Convenience methods for [`VectorObserver`]s.
///
/// See that trait for which types implement this.
pub trait VectorObserverExt<T>: VectorObserver<T>
where
    T: Clone + Send + Sync + 'static,
    <Self::Stream as Stream>::Item: VectorDiffContainer<Element = T>,
{
    /// Filter the vector's values with the given function.
    fn filter<F>(self, f: F) -> (Vector<T>, Filter<Self::Stream, F>)
    where
        F: Fn(&T) -> bool,
    {
        let (items, stream) = self.into_parts();
        Filter::new(items, stream, f)
    }

    /// Filter and map the vector's values with the given function.
    fn filter_map<U, F>(self, f: F) -> (Vector<U>, FilterMap<Self::Stream, F>)
    where
        U: Clone,
        F: Fn(T) -> Option<U>,
    {
        let (items, stream) = self.into_parts();
        FilterMap::new(items, stream, f)
    }

    /// Limit the observed values to `limit`.
    ///
    /// See [`Limit`] for more details.
    fn limit(self, limit: usize) -> (Vector<T>, Limit<Self::Stream, EmptyLimitStream>) {
        let (items, stream) = self.into_parts();
        Limit::new(items, stream, limit)
    }

    /// Limit the observed values to a number of items determined by the given
    /// stream.
    ///
    /// See [`Limit`] for more details.
    fn dynamic_limit<L>(self, limit_stream: L) -> Limit<Self::Stream, L>
    where
        L: Stream<Item = usize>,
    {
        let (items, stream) = self.into_parts();
        Limit::dynamic(items, stream, limit_stream)
    }

    /// Limit the observed values to `initial_limit` items initially, and update
    /// the limit with the value from the given stream.
    ///
    /// See [`Limit`] for more details.
    fn dynamic_limit_with_initial_value<L>(
        self,
        initial_limit: usize,
        limit_stream: L,
    ) -> (Vector<T>, Limit<Self::Stream, L>)
    where
        L: Stream<Item = usize>,
    {
        let (items, stream) = self.into_parts();
        Limit::dynamic_with_initial_limit(items, stream, initial_limit, limit_stream)
    }
}

impl<T, O> VectorObserverExt<T> for O
where
    T: Clone + Send + Sync + 'static,
    O: VectorObserver<T>,
    <Self::Stream as Stream>::Item: VectorDiffContainer<Element = T>,
{
}
