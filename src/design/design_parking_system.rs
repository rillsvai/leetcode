struct ParkingSystem {
    big: i32,
    medium: i32,
    small: i32,
}

impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        ParkingSystem { big, medium, small }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        match car_type {
            1 if self.big >= 1 => {
                self.big -= 1;
                true
            }
            2 if self.medium >= 1 => {
                self.medium -= 1;
                true
            }
            3 if self.small >= 1 => {
                self.small -= 1;
                true
            }
            _ => false,
        }
    }
}
