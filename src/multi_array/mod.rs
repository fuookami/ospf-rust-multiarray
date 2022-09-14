pub mod dummy_vector;
pub mod indexed;
pub mod map_vector;
pub mod multi_array;
pub mod multi_array_view;
pub mod shape;

pub use multi_array::MultiArray;
pub use multi_array_view::MultiArrayView;
pub use shape::{
    DimensionMismatchingError, DynShape, IndexCalculationError, OutOfShapeError, Shape, Shape1,
    Shape2, Shape3, Shape4,
};
