pub trait DataStore {
    fn data_differs(&self, other: &Self) -> bool;
}
