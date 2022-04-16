#![feature(type_name_of_val)]

use std::{
    any::type_name_of_val,
    io::{stdin, stdout, Write},
    thread,
    time::Duration,
};

use lratio::*;

struct You;

ratio!(You);

fn get_argument_response() -> impl L
       + Ratio
       + DontCare
       + DidntAsk
       + CryAboutIt
       + StayMad
       + GetReal
       + Mald
       + Seethe
       + Cope
       + Basic
       + SkillIssue
       + YouFellOff
       + Triggered
       + Redpilled
       + GetALife
       + OkAnd
       + Cringe
       + NotBased
       + TouchGrass
       + Donowalled
       + Send
       + Sync
       + 'static {
    You
}

fn main() {
    print!("Send a messsage > ");
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    println!("Processing message...");
    thread::sleep(Duration::from_millis(1000));

    let response = get_argument_response();
    println!("Response: {}", type_name_of_val(&response));
}
