extern crate handlebars;

use crate::data::{ExternalEvent};
use handlebars::Handlebars;
use std::collections::BTreeMap;
use std::fs::File;
use std::io;

pub fn render_code_panel(lines: io::Lines<io::BufReader<File>>, event_line_map: &BTreeMap<usize, Vec<ExternalEvent>>) -> (String, i32) {
    /* Template creation */
    let mut handlebars = Handlebars::new();
    // We want to preserve the inputs `as is`, and want to make no changes based on html escape.
    handlebars.register_escape_fn(handlebars::no_escape);
    let line_template =
        "        <text class=\"code\" x=\"{{X_VAL}}\" y=\"{{Y_VAL}}\"> {{LINE}} </text>\n";
    // register the template. The template string will be verified and compiled.
    assert!(handlebars
        .register_template_string("code_line_template", line_template)
        .is_ok());

    /* Render the code segment of the svg to a String */
    let x = 20;
    let mut y = 90;
    let mut output = String::from("    <g id=\"code\">\n");
    let mut line_of_code = 1;
    for line in lines {
        if let Ok(line_string) = line {
            let mut data = BTreeMap::new();
            data.insert("X_VAL".to_string(), x.to_string());
            data.insert("Y_VAL".to_string(), y.to_string());
            data.insert("LINE".to_string(), line_string.clone());
            output.push_str(&handlebars.render("code_line_template", &data).unwrap());
            y = y + 20;
        }
        let mut extra_line_num = 0;
        match event_line_map.get(&(line_of_code as usize)) {
            Some(event_vec) => extra_line_num = event_vec.len(),
            None => (),
        }
        if extra_line_num > 1 {
            for i in 0..(extra_line_num - 1) {
                let mut data = BTreeMap::new();
                data.insert("X_VAL".to_string(), x.to_string());
                data.insert("Y_VAL".to_string(), y.to_string());
                data.insert("LINE".to_string(), "   ".to_string());
                output.push_str(&handlebars.render("code_line_template", &data).unwrap());
                y = y + 20;
                line_of_code = line_of_code + 1;
            }
        }
        line_of_code = line_of_code + 1;
    }
    output.push_str("    </g>\n");
    (output, line_of_code)
}
