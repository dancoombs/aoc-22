use itertools::Itertools;

pub struct Input<'a>(&'a str);

impl<'a> Input<'a> {
    pub fn new(s: &'a str) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &'a str {
        self.0
    }

    pub fn split(&self, delim: &str) -> (&str, &str) {
        self.0.split(delim).next_tuple().unwrap()
    }

    pub fn split_and_tform_lines<F, T>(&self, f: F) -> impl Iterator<Item = T> + 'a
    where
        F: Fn(&str) -> T + 'a,
    {
        self.0.lines().map(f)
    }

    pub fn group_and_tform_lines<F, T>(
        &self,
        f: F,
    ) -> impl Iterator<Item = impl Iterator<Item = T> + 'a> + 'a
    where
        F: Fn(&str) -> T + 'a + Copy,
    {
        self.0.split("\n\n").map(move |g| g.lines().map(f))
    }
}
