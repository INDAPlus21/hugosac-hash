# Hugo Sacilotto - Simple Database
Single table SQL like database that stores the data in a csv file. The csv file is read when the program starts and is updated each time the data is altered.

## How to run
Run
```
cargo run
```
from the project directory to start the database.

## Commands
- `INSERT <country> <capital>` will insert a data entry into the database
- `DELETE <country>` will delete the data entry with the specified country
- `GET <country>` will print the capital of the specified country
- `ALL` will print all the data entries
- `QUIT` will quit the program
