use crate::{LoginInfo, ONE_WEEK};
use chrono::{Datelike, NaiveDate};
use headless_chrome::{Browser, LaunchOptions, Tab};
use std::error::Error;

pub struct BrowserHandler {
    _browser: Browser,
    tab: std::sync::Arc<Tab>,
    login_info: LoginInfo,
}

impl BrowserHandler {
    pub fn new(login_info: LoginInfo) -> Result<Self, Box<dyn Error>> {
        let mut options = LaunchOptions::default();
        options.idle_browser_timeout = std::time::Duration::from_secs(600);

        if cfg!(target_os = "linux") {
            options.sandbox = false;
            options
                .args
                .extend_from_slice(&[&std::ffi::OsStr::new("--disable-setuid-sandbox")]);
        }

        let _browser = Browser::new(options)?;
        let tab = _browser.new_tab()?;

        Ok(Self {
            _browser,
            tab,
            login_info,
        })
    }

    pub fn login(&self) -> Result<(), Box<dyn Error>> {
        self.tab
            .navigate_to("https://www.e-license.jp/el31/mSg1DWxRvAI-brGQYS-1OA%3D%3D")?;
        self.tab.wait_until_navigated()?;
        std::thread::sleep(std::time::Duration::from_secs(2));

        self.tab.find_element("input[id='studentId']")?.click()?;
        self.tab.type_str(self.login_info.id())?;

        self.tab.find_element("input[id='password']")?.click()?;
        self.tab.type_str(self.login_info.password())?;

        self.tab.find_element("button[id='login']")?.click()?;
        self.wait_for_navigation()?;

        Ok(())
    }

    pub fn navigate_next_day(&self) -> Result<(), Box<dyn Error>> {
        self.tab
            .find_element("button[class='btn btn-sm btn-light nextWeek']")?
            .click()?;
        self.wait_for_navigation()?;
        Ok(())
    }

    pub fn navigate_prev_day(&self) -> Result<(), Box<dyn Error>> {
        self.tab
            .find_element("button[class='btn btn-sm btn-light lastWeek']")?
            .click()?;
        self.wait_for_navigation()?;
        Ok(())
    }

    pub fn get_start_date(&self) -> Result<NaiveDate, Box<dyn Error>> {
        let start_date_string = self
            .tab
            .find_element("tr[class='date']")?
            .get_inner_text()?;

        // "(曜日)"を除去
        let clean_date: String = start_date_string
            .chars()
            .take_while(|&c| c != '(')
            .collect();

        let formatted_date = format!("{}年{}", chrono::Utc::now().year(), clean_date);
        Ok(NaiveDate::parse_from_str(
            &formatted_date,
            "%Y年%m月%d日\n",
        )?)
    }

    pub fn get_table_vec(&self) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
        let table = self
            .tab
            .find_element("table[class='set yoyakuTable baseTable']")?;
        let mut tbody_vec = vec![vec![]; ONE_WEEK];

        for (i, tr) in table.find_elements("tr[class='date']")?.iter().enumerate() {
            for td in tr.find_elements("td")? {
                let status = td.get_attribute_value("class")?.unwrap();
                tbody_vec[i].push(status);
            }
        }
        Ok(tbody_vec)
    }

    fn wait_for_navigation(&self) -> Result<(), Box<dyn Error>> {
        self.tab.wait_until_navigated()?;
        std::thread::sleep(std::time::Duration::from_secs(5));
        Ok(())
    }
}
