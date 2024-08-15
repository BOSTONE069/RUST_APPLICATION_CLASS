use std::collections::HashSet;

pub fn test_hashset_type() {

   let mut planets = HashSet::from(["Mercury", "Venus", "Earth"]);

   let planet_list_more = HashSet::from(["Earth", "Mars", "Jupiter", "Saturn"]);

   let planet_diff = planets.difference(&planet_list_more); // subratc the difference

   let planet_symdiff = planets.symmetric_difference(&planet_list_more); //merges the difference

//    for planet in planets {
//     println!("{:#?}", planet);
//    }

    // for planet in planet_diff {
    //     println!("{:#?}", planet);
    // }

    planets.insert("Uranus");
    planets.insert("Neptune");
    planets.insert("Pluto");

    for planet in planets{
        println!("{:#?}", planet);
    }
}