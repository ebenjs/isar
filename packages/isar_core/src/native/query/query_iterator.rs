use super::collection_iterator::CollectionIterator;
use super::unsorted_distinct_query_iterator::UnsortedDistinctQueryIterator;
use super::unsorted_query_iterator::UnsortedQueryIterator;
use super::Query;
use crate::core::error::Result;
use crate::native::native_collection::NativeCollection;
use crate::native::native_object::NativeObject;
use crate::native::native_txn::NativeTxn;

pub(crate) enum QueryIterator<'a> {
    Sorted,
    Unsorted(UnsortedQueryIterator<'a>),
    UnsortedDistinct(UnsortedDistinctQueryIterator<'a>),
}

impl<'a> QueryIterator<'a> {
    pub fn new(
        txn: &'a NativeTxn,
        collection: &NativeCollection,
        query: &Query,
        ignore_sort: bool,
    ) -> Result<Self> {
        let collection_iterators = query
            .indexes
            .iter()
            .map(|index| CollectionIterator::new(txn, collection, index))
            .collect::<Result<Vec<_>>>()?;
        if !query.sort.is_empty() && !ignore_sort {
            todo!()
        } else if !query.distinct.is_empty() {
            let iterator = QueryIterator::UnsortedDistinct(UnsortedDistinctQueryIterator::new(
                collection_iterators,
                query.filter.clone(),
                query.distinct.clone(),
                query.offset,
                query.limit,
            ));
            Ok(iterator)
        } else {
            let iterator = QueryIterator::Unsorted(UnsortedQueryIterator::new(
                collection_iterators,
                false,
                query.filter.clone(),
                query.offset,
                query.limit,
            ));
            Ok(iterator)
        }
    }
}

impl<'txn> Iterator for QueryIterator<'txn> {
    type Item = (i64, NativeObject<'txn>);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            QueryIterator::Sorted => todo!(),
            QueryIterator::Unsorted(iterator) => iterator.next(),
            QueryIterator::UnsortedDistinct(iterator) => iterator.next(),
        }
    }
}
