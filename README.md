# rust-contacts-backend

This is a backend server for a contact management system. The server is built with Rust, and the [Actix Web](https://actix.rs/) framework.
The database used here is SQLite, implemented using the [rusqlite](https://github.com/rusqlite/rusqlite) package.

## ⭐Features:
- Add contact
- List contacts
- Delete Contact
- Get specific contact.
- Edit Contact.

## Routes:
- GET `/contact` get all contacts
- GET `/contact/{id}` get a specific contact
- POST `/contact/new` create a new contact
- GET `/contact/{id}/delete` delete a contact
- POST `/contact/{id}/edit` delete a contact

## How to run
You can run the server by:
1. Cloning this repository (`git clone https://github.com/DreadedHippy/rust-contacts-backend.git`) and compiling it yourself, or
2. Downloading the precompiled binary [here](https://github.com/DreadedHippy/rust-contacts-backend/releases/tag/v1.0.0)

## Other notes:
- The server defaults to port 8080
- A demo frontend using the Ionic Framework can be found [here](https://github.com/DreadedHippy/rust-contacts-frontend/), simply follow the instructions in the README

Thank you for checking this out. If you like this project, give it a star 😉
💫Have fun
