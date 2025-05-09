use lumoctl::{establish_connection, models::*};
use chrono::prelude::*;
use chrono::{NaiveDate, NaiveTime};
use diesel::prelude::*;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    use lumoctl::schema::depok::dsl::*;
    matikan_semua_saklar(); // Semua saklar selain yang akan disesuaikan di bawah
    println!();
    let pin = [[1, 0], [2, 2], [3, 3]]; // [no_saklar, no_pin] pin yang akan disesuaikan

    loop {
        let connection = establish_connection();

        let tanggal_sekarang = NaiveDate::from_ymd(2020, Local::now().month(), Local::now().day());

        // Ambil waktu sholat hari ini
        let results = depok
            .filter(tanggal.eq(tanggal_sekarang))
            .limit(1)
            .load::<JadwalDepok>(&connection)
            .expect("Error loading waktu sholat");

        // Ambil Waktu Subuh
        let subuh = if let Some(x) = results[0].fajr {
            x
        } else {
            NaiveTime::from_hms(6, 0, 0)
        };

        // Ambil Waktu Syuruq untuk awal
        let awal = if let Some(x) = results[0].syuruq {
            x
        } else {
            NaiveTime::from_hms(6, 0, 0)
        };

        // Ambil waktu Mahgrib untuk akhir
        let akhir = if let Some(x) = results[0].maghrib {
            x
        } else {
            NaiveTime::from_hms(18, 0, 0)
        };

        println!("Pukul: {:?}", Local::now().time());

        // Saklar lampu teras depan
        sesuaikan_saklar(
            pin[0][0],
            pin[0][1],
            awal - chrono::Duration::minutes(10),
            akhir,
        ); // Saklar 1

        // Saklar lampu teras belakang
        sesuaikan_saklar(
            pin[1][0],
            pin[1][1],
            awal + chrono::Duration::minutes(10),
            akhir - chrono::Duration::minutes(20),
        ); // Saklar 2

        // Saklar lampu depan jendela kamar depan
        sesuaikan_saklar(
            pin[2][0],
            pin[2][1],
            subuh + chrono::Duration::minutes(10),
            akhir + chrono::Duration::minutes(20),
        ); // Saklar 3

        println!();
        sleep(Duration::new(27, 0)); // Delay 27 detik
    }
}

fn sesuaikan_saklar(no_saklar: i32, no_pin: i32, awal: NaiveTime, akhir: NaiveTime) {
    let sekarang = Local::now().time();
    print!(
        "Saklar {} [Pin {}]: OFF dari {} sampai {}. ",
        no_saklar, no_pin, awal, akhir
    );

    let status = read_gpio(no_pin);
    // let status = "0";
    print!("Status: {} ", status);

    // let saklar: bool = if status.trim() == "1" { false } else { true }; //NO
    let saklar: bool = if status.trim() == "1" { true } else { false }; //NC
    print!("(ON: {}) ", saklar);

    if (sekarang >= awal) && (sekarang <= akhir) {
        if saklar == true {
            println!("--> Set OFF!");
            // set_gpio(no_pin, 1); //NO
            set_gpio(no_pin, 0); //NC
        } else {
            println!();
        }
    } else {
        if saklar == true {
            println!();
        } else {
            println!("--> Set ON!");
            // set_gpio(no_pin, 0); //NO
            set_gpio(no_pin, 1); //NC
        }
    }
}

fn matikan_semua_saklar() {
    let pin: [i32; 5] = [4, 6, 12, 13, 14];

    let mut index = 4;
    println!("Mematikan semua saklar");
    println!("======================");
    for element in pin.iter() {
        println!("Saklar {} / Pin {} dimatikan", index, element);
        set_gpio(*element, 1);
        index = index + 1;
    }
}

fn set_gpio(pin: i32, val: i32) {
    use std::process::Command;
    Command::new("gpio")
        .arg("write")
        .arg(pin.to_string())
        .arg(val.to_string())
        .output()
        .expect("failed to write gpio");
    Command::new("gpio")
        .arg("mode")
        .arg(pin.to_string())
        .arg("out")
        .output()
        .expect("failed to set mode gpio");
}

fn read_gpio(pin: i32) -> String {
    use std::process::Command;

    let gpioread = Command::new("gpio")
        .arg("read")
        .arg(pin.to_string())
        .output()
        .expect("failed to read gpio");

    String::from_utf8_lossy(&gpioread.stdout).to_string()
}
