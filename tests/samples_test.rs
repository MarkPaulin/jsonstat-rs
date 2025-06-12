use jsonstat::*;

fn read_from_url(url: &str) -> JsonStat {
    let data = reqwest::blocking::get(url)
        .expect("could not get url")
        .text()
        .expect("could not read data");

    let stats: JsonStat = serde_json::from_str(&data).expect("could not parse json");
    stats
}

#[test]
fn can_read_samples_list() {
    let datasets = read_from_url("https://json-stat.org/samples/datasets/index.json");
    let coll = JsonStatCollection::try_from(datasets).unwrap();
    assert_eq!(
        coll.link.unwrap()["item"].len(),
        8
    );

    assert_eq!(
        coll.class,
        Class::Collection
    );
}

#[test]
fn can_read_oecd() {
    let stats = read_from_url("https://json-stat.org/samples/oecd.json");
    let dataset = JsonStatDataset::try_from(stats).unwrap();
    assert_eq!(dataset.label.unwrap(), String::from("Unemployment rate in the OECD countries 2003-2014"));
}

#[test]
fn can_read_canada() {
    let stats = read_from_url("https://json-stat.org/samples/canada.json");
    let _ = JsonStatDataset::try_from(stats).unwrap();
}

#[test]
fn can_read_oecd_canada_col() {
    let stats = read_from_url("https://json-stat.org/samples/oecd-canada-col.json");
    let _ = JsonStatCollection::try_from(stats).unwrap();
}


#[test]
fn can_read_galicia() {
    let stats = read_from_url("https://json-stat.org/samples/galicia.json");
    let _ = JsonStatDataset::try_from(stats).unwrap();
}
