use rayon::prelude::*;
use thirtyfour::prelude::*;
use tokio;

fn main() {
    let arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let results: Vec<Vec<String>> = arr.into_par_iter()
        .map(|month| get_suntimes(month+1, 2022))
        .collect();
    println!("{:?}", results);
}

#[tokio::main]
async fn get_suntimes(month: u8, year: u16) -> Vec<String> {
    // let mut caps = DesiredCapabilities::chrome();
    // caps.add_chrome_arg("--headless").unwrap();
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:4444/wd/hub", &caps)
        .await
        .unwrap();
    driver
        .get(format!(
            "https://www.timeanddate.com/sun/usa/austin?month={month}&year={year}"
        ))
        .await
        .unwrap();
    if driver
        .find_elements(By::Css(".selected"))
        .await
        .unwrap()
        .len()
        == 1
    {
        driver
            .find_element(By::Css(".selected"))
            .await
            .unwrap()
            .click()
            .await
            .unwrap();
    }
    let days_in_months = driver
        .find_elements(By::Css("#as-monthsun tr"))
        .await
        .unwrap();
    let mut each_day_of_month = Vec::new();
    let omit = vec![0, 1, 2, days_in_months.len() - 1];
    for each_day in days_in_months.iter().enumerate() {
        if !omit.contains(&each_day.0) {
            each_day_of_month.push(each_day.1.text().await.unwrap());
        }
    }
    driver.quit().await.unwrap();
    each_day_of_month
}
