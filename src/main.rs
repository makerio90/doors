use rand::prelude::*;

#[derive(Debug, PartialEq)]
enum DoorState {
    Open,
    Empty(bool),
    Prize(bool),
}
fn main() {
    let mut number_of_loops = 0;
    let mut times_should_have_switched = 0;
    while number_of_loops < 1000000 {
        let mut doors = [
            DoorState::Empty(false),
            DoorState::Empty(false),
            DoorState::Empty(false),
        ];
        //select one door as being the prize door
        let mut rng = thread_rng();
        let x = rng.gen_range(0..doors.len());
        doors[x] = DoorState::Prize(false);
        //choose a door as being correct
        let y = rng.gen_range(0..doors.len());
        doors[y] = match doors[y] {
            DoorState::Empty(_) => DoorState::Empty(true),
            DoorState::Prize(_) => DoorState::Prize(true),
            _ => panic!("this wasn't supposed to happen!")
        };
        //open a qualifying door
        let mut i = 0;
        while i < doors.len() {
            if doors[i] == DoorState::Empty(false) {
                doors[i] = DoorState::Open;
                break;
            }
            i += 1;
        }
        //if they should have switched there awnser, increase the counter.
        for door in doors {
            if door == DoorState::Prize(false) {
                times_should_have_switched += 1;
            }
        }
        number_of_loops += 1;
    }
    let result_percent = (f64::from(times_should_have_switched) / f64::from(number_of_loops))*100.0;
    println!(
        "{} out of {} tries it would have been better to switch your awnser. \n thats a {}% chance",
        times_should_have_switched, number_of_loops, result_percent
    )
}
