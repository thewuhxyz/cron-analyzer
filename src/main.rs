mod cron_analyzer;

use cron_analyzer::*;

fn main() {
    let expression = String::from("0 30 9,12,15 1,15 May-Aug Mon,Wed,Fri 2018/2");
    let cron_helper = CronAnalyzer::from(expression);
    println!("analysis: {}", &cron_helper.analyze());
}