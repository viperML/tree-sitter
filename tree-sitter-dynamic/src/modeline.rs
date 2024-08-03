
#[derive(Debug, Clone, PartialEq)]
pub struct Modeline {
    pub inherits: Vec<String>,
    pub extends: bool
}


impl Modeline {
    pub fn get(source: &str) -> Self {
        let mut inherits = Vec::new();

        for line in source.lines() {
            if ! line.starts_with(";") {
                break;
            }

            let trimmed = line.replace(";", "").trim().to_owned();

            if trimmed.starts_with("inherits: ") {
                for lang in trimmed.replace("inherits:", "").trim().split(",") {
                    inherits.push(String::from(lang.trim()));
                }
            }
        }

        Modeline {
            inherits,
            extends: false,
        }
    }
}

#[test]
fn test_modeline() {
    let s = r#";; inherits: typescript, tsx
;; extends
"#;

    let modeline = Modeline::get(s);

    assert_eq!(
        modeline,
        Modeline {
            inherits: vec![String::from("typescript"), String::from("tsx")],
            extends: false
        }
    )
}
