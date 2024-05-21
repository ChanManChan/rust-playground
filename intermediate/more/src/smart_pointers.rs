// Topic: Smart Pointers & RefCell
//
// Summary:
//   A vehicle rental company wants to access the rentals available
//   at storefront locations. Create a program that provides access
//   to storefront rentals from the corporate headquarters.
//
// Requirements:
// * Corporate must be able to access the rentals at a storefront
// * Storefronts must be able to rent out vehicles
// * Rentals have the following attributes:
//   - Type of vehicle
//   - Vehicle Identification Number (VIN)
//   - Vehicle status:
//     * Available, Unavailable, Maintenance, Rented
//
// Notes:
// * Use Rc and RefCell to create shared mutable data structures
// * Create at least two rentals and ensure that Corporate and StoreFront
//   can both access the rental information
// * Test your program by changing the vehicle status from both a storefront
//   and from corporate

use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

#[derive(Debug)]
enum Vehicle {
    Car,
    Bike,
    Cycle,
    Truck,
}

#[derive(Debug)]
enum VehicleStatus {
    Available,
    Unavailable,
    Maintenance,
    Rented,
}

#[derive(Debug)]
struct Rental {
    rental_type: Vehicle,
    vin: String,
    status: RefCell<VehicleStatus>,
}

struct Corporate;

impl Corporate {
    fn change_rental_status(rental: Rc<Rental>) {
        let mut status = rental.status.borrow_mut();
        *status = VehicleStatus::Maintenance;
    }
}

struct StoreFront;

impl StoreFront {
    fn change_rental_status(rental: Rc<Rental>) {
        let mut status = rental.status.borrow_mut();
        *status = VehicleStatus::Rented;
    }
}

fn main() {
    let rental_1 = Rc::new(Rental {
        rental_type: Vehicle::Car,
        vin: "123".to_owned(),
        status: RefCell::new(VehicleStatus::Available),
    });

    // let rental_2 = Rc::new(Rental {
    //     rental_type: Vehicle::Cycle,
    //     vin: "321".to_owned(),
    //     status: RefCell::new(VehicleStatus::Maintenance),
    // });

    println!("Rental 1 status before: {:?}", rental_1.status.borrow());

    Corporate::change_rental_status(Rc::clone(&rental_1));

    println!(
        "Rental 1 status after corporate: {:?}",
        rental_1.status.borrow()
    );

    StoreFront::change_rental_status(Rc::clone(&rental_1));

    println!(
        "Rental 1 status after storefront: {:?}",
        rental_1.status.borrow()
    );

    println!("Rental 1: {:?}", rental_1);
}

// use std::{cell::RefCell, rc::Rc};
//
// enum Kind {
//     Car,
//     Truck,
// }
//
// #[derive(Debug, PartialEq)]
// enum Status {
//     Available,
//     Unavailable,
//     Maintenance,
//     Rented,
// }
//
// struct Rental {
//     kind: Kind,
//     vin: String,
//     status: Status,
// }
//
// type SharedData = Rc<RefCell<Vec<Rental>>>;
//
// struct Corporate(SharedData);
//
// struct StoreFront(SharedData);
//
// fn get_rentals() -> SharedData {
//     let rentals = vec![
//         Rental {
//             kind: Kind::Car,
//             vin: "123".to_owned(),
//             status: Status::Available,
//         },
//         Rental {
//             kind: Kind::Truck,
//             vin: "abc".to_owned(),
//             status: Status::Maintenance,
//         },
//     ];
//
//     Rc::new(RefCell::new(rentals))
// }
//
// fn main() {}
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn check_rc_refcell() {
//         let rentals = get_rentals();
//
//         let corporate = Corporate(Rc::clone(&rentals));
//         let storefront = StoreFront(Rc::clone(&rentals));
//
//         {
//             let mut rentals = corporate.0.borrow_mut();
//             if let Some(car) = rentals.get_mut(0) {
//                 assert_eq!(car.status, Status::Available);
//                 car.status = Status::Maintenance;
//             }
//         }
//
//         {
//             let mut rentals = storefront.0.borrow_mut();
//             if let Some(car) = rentals.get_mut(0) {
//                 assert_eq!(car.status, Status::Maintenance);
//                 car.status = Status::Rented;
//             }
//         }
//
//         if let Some(car) = corporate.0.borrow().get(0) {
//             assert_eq!(car.status, Status::Rented);
//         };
//     }
// }
//
