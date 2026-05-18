pub struct Inventory {
    pub hotbar: [[Option<String>; 3]; 3], // 3 rows of 3 columns each
    pub hotbar_selected: (usize, usize), // (row, column)
    pub inventory: [[Option<String>; 6]; 3], // 3 rows of 6 columns each
}
impl Inventory {
    pub fn new() -> Self {
        Self {
            hotbar: Default::default(),
            hotbar_selected: (1, 1),
            inventory: Default::default(),
        }
    }
    pub fn get_selected_hotbar_item(&self) -> Option<&String> {
        let (row, col) = self.hotbar_selected;
        self.hotbar.get(row).and_then(|r: &[Option<String>; 3]| r.get(col)).and_then(|item: &Option<String>| item.as_ref())
    }
}