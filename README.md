# Visual Regression Testing Platform (Rust + Microservices)

A high-performance, scalable visual regression testing platform built with **Rust**, **TypeScript**, **Docker**, and **Next.js**.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.82%2B-orange.svg)
![TypeScript](https://img.shields.io/badge/typescript-5.0%2B-blue.svg)
![Docker](https://img.shields.io/badge/docker-ready-green.svg)

## 🚀 Overview

This project is a modern re-architecture of a visual regression testing system. It is designed to handle large-scale screenshot comparisons with high performance and stability.

### Key Features

- **Microservices Architecture**: Decoupled services for scalability and maintainability.
- **High Performance**: Core logic (Orchestration, Image Diffing, Reporting) implemented in **Rust**.
- **Stability**: Browser automation handled by **TypeScript** and **Playwright** (official bindings).
- **Modern UI**: Responsive and beautiful dashboard built with **Next.js**.
- **Containerized**: Fully Dockerized for easy deployment (Docker Compose / Kubernetes).
- **AI-Ready**: Designed to integrate AI models for semantic difference detection (future roadmap).

## 🏗 Architecture

The system is composed of the following services:

| Service            | Tech Stack              | Description                                                  |
| :----------------- | :---------------------- | :----------------------------------------------------------- |
| **Orchestrator**   | Rust (Axum, SQLx)       | Manages test runs, schedules jobs, and handles API requests. |
| **Worker**         | TypeScript (Playwright) | Executes browser automation and captures screenshots.        |
| **Diff Service**   | Rust (Image-rs)         | Compares images using MSSIM/Pixelmatch algorithms.           |
| **Report Service** | Rust (Axum)             | Serves the API for test results and reports.                 |
| **Frontend**       | Next.js (React)         | User interface for viewing results and managing tests.       |
| **Database**       | PostgreSQL              | Stores metadata, test results, and configurations.           |
| **Storage**        | MinIO (S3)              | Stores screenshot images.                                    |

## 🛠 Getting Started

### Prerequisites

- Docker & Docker Compose

### Installation

1.  **Clone the repository**

    ```bash
    git clone https://github.com/yourusername/visual-regression-platform.git
    cd visual-regression-platform
    ```

2.  **Start the services**

    ```bash
    docker compose up --build
    ```

3.  **Access the Dashboard**
    Open [http://localhost:3000](http://localhost:3000) in your browser.

## 🧪 Usage

### Trigger a Test Run

You can trigger a test run via the API:

```bash
curl -X POST http://localhost:8080/tests \
  -H "Content-Type: application/json" \
  -d '{"urls": ["https://example.com", "https://google.com"]}'
```

### View Results

Navigate to the dashboard to see the progress and results of your visual regression tests.

## 🔮 Roadmap

- [ ] **AI Integration**: Use Computer Vision / LLMs to detect "meaningful" changes vs. noise.
- [ ] **CI/CD Integration**: GitHub Actions / GitLab CI plugins.
- [ ] **Kubernetes Helm Charts**: For production deployment.

## 📝 License

This project is licensed under the MIT License.
