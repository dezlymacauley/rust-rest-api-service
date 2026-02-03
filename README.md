# Rust REST API Service
_______________________________________________________________________________

### Technologies Used

| Component         | Technology                  |
|-------------------|-----------------------------|
| Backend Framework | 🦀 Axum (Rust)              |
| Database          | 🐘 Postgres (SQL)           |
| Container         | 🐳 Docker                   |
| API Testing       | 🥧 HTTPie (Python), 🐚 Bash |
| API Testing       | 🥧 HTTPie (Python), 🐚 Bash |

_______________________________________________________________________________

### Project Structure / Rationale

Two docker containers. One for Axum, and another for Postgres.

They will be connected using `Docker Compose` because Kubernetes 
would be excessive for a two container setup. 

Then HTTPie will be setup locally in this repo with with HTTPie.

I will be using `mise` and `uv` to setup `Python` and `HTTPie`.

And just as I did with my 
[Postgres Dojo](https://github.com/dezlymacauley/postgres-dojo) workflow,
I will be using my bash scripting skills and the functionality of `mise` to 
turn bash script file names into a commands that will make this a professional
development environment.

_______________________________________________________________________________

### API Endpoints

1. CREATE User

2. GET User
3. GET ALL USERS

4. UPDATE USER

5. DELETE USER

_______________________________________________________________________________
