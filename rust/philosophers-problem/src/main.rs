use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};
use crate::philosopher::{EatingPhilosopher, ThinkingPhilosopher};
use crate::fork::Fork;

mod philosopher;
mod fork;

fn main() {
    let (senderFromDiogenesToMain, receiverFromDiogenesToMain) = channel::<Fork>();
    let (senderFromMainToDiogenes, receiverFromMainToDiogenes) = channel::<Fork>();
    thread::spawn(create_philosopher_thread(String::from("Diogenes"), senderFromDiogenesToMain, receiverFromMainToDiogenes));

    let (senderFromAristotleToMain, receiverFromAristotleToMain) = channel::<Fork>();
    let (senderFromMainToAristotle, receiverFromMainToAristotle) = channel::<Fork>();
    thread::spawn(create_philosopher_thread(String::from("Aristotle"), senderFromAristotleToMain, receiverFromMainToAristotle));

    let (senderFromPlatoToMain, receiverFromPlatoToMain) = channel::<Fork>();
    let (senderFromMainToPlato, receiverFromMainToPlato) = channel::<Fork>();
    thread::spawn(create_philosopher_thread(String::from("Plato"), senderFromPlatoToMain, receiverFromMainToPlato));

    let (senderFromSocratesToMain, receiverFromSocratesToMain) = channel::<Fork>();
    let (senderFromMainToSocrates, receiverFromMainToSocrates) = channel::<Fork>();
    thread::spawn(create_philosopher_thread(String::from("Socrates"), senderFromSocratesToMain, receiverFromMainToSocrates));

    let (senderFromHippocratesToMain, receiverFromHippocratesToMain) = channel::<Fork>();
    let (senderFromMainToHippocrates, receiverFromMainToHippocrates) = channel::<Fork>();
    thread::spawn(create_philosopher_thread(String::from("Hippocrates"), senderFromHippocratesToMain, receiverFromMainToHippocrates));


}

fn create_philosopher_thread(
    name: String,
    sender: Sender<Fork>,
    receiver: Receiver<Fork>,
) -> fn() {
    let philosopher = ThinkingPhilosopher::new(name);


    return move || {

    }
}
