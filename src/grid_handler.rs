use crate::Grid;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct GridLayout {
    current_layout: Grid,
    layouts: Vec<(Grid, String)>,
}
