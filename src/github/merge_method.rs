#[derive(Debug)]
pub enum MergeMethod {
    Squash,
    Merge,
}

impl From<MergeMethod> for &'static str {
    fn from(merge_method: MergeMethod) -> &'static str {
        match merge_method {
            MergeMethod::Squash => "squash",
            MergeMethod::Merge => "merge",
        }
    }
}
