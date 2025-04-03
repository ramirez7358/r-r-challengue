# r-r-challenge

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

The frontend will be available at `http://localhost:5173` by default.

---

---

## AWS EC2 Deployment Guide

Follow these steps to deploy the backend on an AWS EC2 instance:

### **Step 1: Launch an EC2 Instance**

1. Log in to your [AWS Management Console](https://aws.amazon.com/console/).
2. Navigate to **EC2** → **Instances** → **Launch Instances**.
3. Select an Amazon Machine Image (AMI), such as Amazon Linux 2023 or Ubuntu 22.04.
4. Choose an instance type (`t2.micro` is free tier eligible).
5. Configure security groups to allow:
    - Port `22` for SSH access.
    - Port `8080` for your Actix Web app.
6. Launch and download the `.pem` key file.

### **Step 2: Connect to Your EC2 Instance**

1. Open your terminal and connect using SSH:
   ```bash
   ssh -i /path/to/your-key.pem ec2-user@your-ec2-public-ip
   ```

### **Step 3: Clone the Repository and Run the Deployment Script**

1. Clone your repository:
   ```bash
   git clone https://github.com/ramirez7358/r-r-challengue.git
   cd r-r-challengue
   ```
2. Provide execute permissions to the deployment script:
   ```bash
   chmod +x deploy.sh
   ```
3. Run the script with `sudo` to install dependencies, build Docker containers, and start the services:
   ```bash
   sudo ./deploy.sh
   ```

Your application will be available at:

```
http://<your-ec2-public-ip>:8080
```

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