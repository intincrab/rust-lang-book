fn main() {
    enum SperadsheetCell { 
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row =vec![
        SperadsheetCell::Int(3),
        SperadsheetCell::Text(String::from("blue")),
        SperadsheetCell::Float(10.12),
    ];

}