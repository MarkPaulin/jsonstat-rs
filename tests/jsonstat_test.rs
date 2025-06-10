use jsonstat::*;

#[test]
fn read_jsonstat() {
    let data = r#"{
        "version" : "2.0",
        "class" : "dataset",
        "label" : "Population in Tuvalu in 2002. By sex",
        "value" : [4729, 4832, 9561],
        "status": { "1" : "m" },
        "id" : ["metric", "time", "geo", "sex"],
        "size" : [1, 1, 1, 3],
        "dimension" : {
           "sex" : {
              "label" : "sex",
              "category" : {
                 "index" : {
                   "M" : 0,
                   "F" : 1,
                   "T" : 2
                 },
                 "label" : {
                   "M" : "men",
                   "F" : "women",
                   "T" : "total"
                 }
              }
           }
        },
        "extension": {
            "contact": "mark.paulin@googlemail.com"
        }
    }"#;
    
        let stats: JsonStat = serde_json::from_str(data).unwrap();
        let dataset = JsonStatDataset::try_from(stats).unwrap(); 
    
        assert_eq!(dataset.version, Version::V2_0);
        assert_eq!(dataset.class, Class::Dataset);
        assert_eq!(dataset.label.unwrap(), String::from("Population in Tuvalu in 2002. By sex"));
        assert_eq!(dataset.id, [String::from("metric"), String::from("time"), String::from("geo"), String::from("sex")]);
        assert_eq!(dataset.size, [1, 1, 1, 3]);    
}