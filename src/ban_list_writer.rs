pub struct EDOProBanListEntry {
    // Different than konami id
    pub ygo_pro_id: i32,
    pub count: i32,
    pub card_name: String,
}

impl EDOProBanListEntry {
    fn to_line(&self) -> String {
        return format!("{} {} --{}", self.ygo_pro_id, self.count, self.card_name)
    }
}

pub fn write(date: &str, ban_list_entries: Vec<EDOProBanListEntry>) -> Result<(), std::io::Error> {
    let mut contents = Vec::<String>::new();

    let file_name = format!("{}_master_duel.lflist.conf", date);

    contents.push(format!("#[{} Master Duel]", date));
    contents.push(format!("!{} Master Duel", date));

    contents.push(String::from("#Forbidden"));
    contents.extend(ban_list_entries.iter()
        .filter(|entry| entry.count == 0)
        .map(|entry| entry.to_line()));

    contents.push(String::from("#Limited"));
    contents.extend(ban_list_entries.iter()
        .filter(|entry| entry.count == 1)
        .map(|entry| entry.to_line()));

    contents.push(String::from("Semi-limited"));
    contents.extend(ban_list_entries.iter()
        .filter(|entry| entry.count == 2)
        .map(|entry| entry.to_line()));

    let content = contents.join("\n");

    std::fs::write(file_name, content)
}