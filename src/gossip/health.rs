/// Describes the current cluster health. This is an important
/// value to use while debugging or for diagnostics.
///
/// Red = Cluster is dead. Major failures/partitions.
/// Yello = There have been some partitions and other issues, but
///         at a level where the cluster can handle it.
/// Green = Everything is in good shape.
pub enum Health {
    Red,
    Yellow,
    Green
}
