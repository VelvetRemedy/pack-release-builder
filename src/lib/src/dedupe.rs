use super::pdtfs::{check_if_dir_exists, find_files_in_dir};
use super::pdthash::get_hash;

pub fn dedupe(dir: String) {
    check_if_dir_exists(&dir);
    let recursive = true;
    let extensions = Some(vec![".zip"]);
    let files = find_files_in_dir(&dir, recursive, &extensions);
    let mut records: Vec<(String, String)> = vec![];
    for file in files {
        let hash = get_hash(&file);
        records.push((hash, file));
    }
    records.sort();
    let mut dupes: Vec<Vec<(String, String)>> = vec![];
    let mut i = 0;
    while i < records.len() {
        let dupe: Vec<_> = records.clone().into_iter().filter(|x| records[i].0 == x.0).collect();
        if dupe.len() > 1 {
            dupes.push(dupe.clone());
        }
        i += dupe.len();
    }
    dupes.sort();
    dupes.dedup();
    println!("{:#?}", dupes);
}
