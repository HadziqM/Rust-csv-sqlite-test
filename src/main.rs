#![allow(dead_code)]
use serde::Deserialize;
use std::cell::Cell;

pub mod csv_in;
pub mod sql_init;

#[derive(Debug,Deserialize,Clone,PartialEq)]
pub struct Input{
    date_full: String,
    date: String,
    clock: String,
    pin: String,
    nip: String,
    name: String,
    occupation: Option<String>,
    departement: Option<String>,
    office : Option<String>,
    verivication : String,
    io :String,
    workcode : String,
    sn :String,
    machine: String
}

#[derive(Debug,PartialEq, Clone)]
pub enum PrayTime {
    Duhur,
    Asyar,
    Maghrib,
    Isya,
    Subuh,
    Tahajud
}

#[derive(Debug)]
pub struct TimeLimit {
    duhur_s: String,
    duhur_f: String,
    asyar_s: String,
    asyar_f: String,
    maghrib_s: String,
    maghrib_f: String,
    isya_s: String,
    isya_f: String,
    subuh_s: String,
    subuh_f: String,
    tahajud_s: String,
    tahajud_f: String,
}

pub struct Hold {
    holder: Vec<Holder>
}
pub struct Holder {
    pin: String,
    name: String,
    pray: Cell<Vec<PrayHold>>
}
#[derive(Debug,PartialEq)]
pub struct PrayHold {
    date: (u32,u32,u32),
    pray:PrayTime,
    db_date:String,
    machine:String
}
#[derive(Debug)]
struct CSVOUT {
    pin: String,
    name: String,
    date: (u32,u32,u32),
    pray: PrayTime,
    db_date: String,
    machine: String
}
fn main() {

        let pray:TimeLimit = TimeLimit { 
            duhur_s: "12:00".to_owned(),
            duhur_f: "13:30".to_owned(),
            asyar_s: "15:00".to_owned(),
            asyar_f: "16:30".to_owned(),
            maghrib_s: "18:00".to_owned(),
            maghrib_f: "19:00".to_owned(),
            isya_s: "19:30".to_owned(),
            isya_f: "20:30".to_owned(),
            subuh_s: "03:30".to_owned(),
            subuh_f: "05:30".to_owned(),
            tahajud_s: "02:00".to_owned(),
            tahajud_f: "03:20".to_owned()
        };
    csv_in::csv2database("./26 Sep - 20 Okt 22.csv",&pray);
}
