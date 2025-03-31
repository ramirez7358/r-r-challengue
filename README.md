# r-r-challengue

## Prerequisites

Make sure you have the following installed on your machine:

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/en/download/) (with npm)
- [Docker](https://www.docker.com/products/docker-desktop/)

---

## Backend Setup

Follow these steps to set up and run the backend:

1. **Clone the Repository:**
    ```bash
    git clone https://github.com/ramirez7358/r-r-challengue.git
    cd r-r-challengue
    ```

2. **Start the Database Service:**
    ```bash
    docker-compose up db
    ```
   This will start only the PostgreSQL database service using Docker.

3. **Build the Application:**
    ```bash
    cargo b
    ```

4. **Configure Environment Variables:**
    - Copy the example environment configuration file:
    ```bash
    cp env/env_example.json env/env.json
    ```
    - Adjust any necessary configurations in `env.json`.

5. **Run the Application:**
    ```bash
    cargo run
    ```

---

## Frontend Setup

Follow these steps to set up and run the frontend:

1. **Navigate to the Frontend Directory:**
    ```bash
    cd frontend
    ```

2. **Install Dependencies:**
    ```bash
    npm install
    ```

3. **Run the Application:**
    ```bash
    npm run dev
    ```

The frontend will be available at `http://localhost:3000` by default.

---

## Additional Notes
- Ensure the database is running before starting the backend.
- If you encounter any issues, check the logs with:
  ```bash
  docker-compose logs db
  ```
- For backend issues, view Rust application logs using:
  ```bash
  cargo run
  ```