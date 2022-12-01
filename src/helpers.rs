/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

pub struct Input<'a>(&'a str);

impl<'a> Input<'a> {
    pub fn new(s: &'a str) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &'a str {
        self.0
    }

    pub fn tform_and_group_lines<F, T>(
        &self,
        f: F,
    ) -> impl Iterator<Item = impl Iterator<Item = T> + 'a> + 'a
    where
        F: Fn(&str) -> T + 'static + Copy,
    {
        self.0
            .split("\n\n")
            .map(move |g| g.lines().map(f))
    }
}
