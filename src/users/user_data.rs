//_____________________________________________________________________________

// This struct is for getting the details of a User,
// so therefore it will include all the fields

// I use i32 here to map it to the SERIAL data type in Postgres.
// Even though `id` should not be negative, the reason why I use i32 and not
// u32 is because Postgres does not have the concept of unsigned integers.
// That needs to be enforces at both the database layer and the application
// layer later on.

// These are the integer types in Postgres:

// -- SMALLINT (2 bytes | 16 bits): -32,768 to 32,767
// -- INT (4 bytes | 32 bits): -2,147,483,648 to 2,147,483,647
// -- BIGINT (8 bytes | 64 bits): -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807

// #[derive(Se)]
// pub struct User {
//     id: i32,
//     name: String,
//     email: String
// }

//_____________________________________________________________________________
