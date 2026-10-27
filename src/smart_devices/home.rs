use super::functions::ReportDevice;
use super::room::Room;

pub struct SmartHouse {
    pub name: String,
    pub rooms: Vec<Room>,
}

impl SmartHouse {
    pub fn new(name: String, rooms: Vec<Room>) -> Self {
        Self { name, rooms }
    }

    pub fn get_room(&self, index: usize) -> &Room {
        if index >= self.rooms.len() {
            panic!(
                "Индекс комнаты {} выходит за пределы дома {}!",
                index, self.name
            );
        }
        &self.rooms[index]
    }

    pub fn get_room_mut(&mut self, index: usize) -> &mut Room {
        if index >= self.rooms.len() {
            panic!(
                "Индекс комнаты {} выходит за пределы дома {}!",
                index, self.name
            );
        }
        &mut self.rooms[index]
    }

    pub fn print_report(&self) {
        println!("=== ОТЧЕТ ПО ДОМУ: {} ===", self.name);
        for room in &self.rooms {
            room.print_report();
        }
        println!("=================================\n");
    }
}
