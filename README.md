# Simple URL Shortener 
A simple and fast URL shortener, implemented in **Rust**.

---

## Overview

This project is a minimalistic **URL shortener** developed in Rust for learning and demonstration purposes. It provides a basic RESTful API to create, list, and redirect shortened URLs.
It also uses Redis as storage. It supports aliases with time expiration.

**Important:**
This project is **strictly for learning and development purposes**. It **must not be used in production environments** and contains known vulnerabilities (such as lack of robust authentication and non-persistent storage) which are planned to be addressed in future work.

---

## How to Run

### Prerequisites
Make sure you have **Rust** installed on your machine. You can install it using [rustup](https://rustup.rs/).


### Compilation and Execution

1.  Clone the repository:
    ```bash
    git clone https://github.com/hrchlhck/url-shortener.git
    cd url-shortener
    ```

2. Configure the environment variables
    ```bash
    REDIS_ADDRESS=localhost
    API_ADDRESS=localhost
    API_PORT=8080
    ```

4.  Run the application:
    ```bash
    cargo run
    ```

The server will start and be accessible at `http://127.0.0.1:8080` (or your configured port).

---

## API Endpoints

The application exposes the following REST endpoints. It is assumed the server is running on `http://127.0.0.1:8080`.

| Method | Endpoint | Description | Usage Example |
| :---: | :--- | :--- | :--- |
| **POST** | `/new` | **Creates** a new shortened URL. | Send a JSON payload containing the original URL in the request body. |
| **GET** | `/list` | **Lists** all currently stored shortened URLs. | Returns an array of URL objects. |
| **GET** | `/<short>` | **Redirects** to the original URL associated with the short code. | Replace `<short>` with the generated code (e.g., `/abcde`). |

### Creation Example (`POST /new`)

You can use `curl` to test creating a new URL:

```bash
curl -X POST http://127.0.0.1:8080/new \
     -H "Content-Type: application/json" \
     -d '{"short_url": "abcde", "long_url": "https://github.com/hrchlhck/url-shortener", "expiration": 30}'
```

## Future work

The following features are planned for future development:
- [ ] Enable TLS communication (HTTPS).
- [ ] Support for OTP authentication.
- [x] Database support for URL persistence.
- [ ] Development of a user-friendly front-end.

## Contributions
Contributions are welcome. Feel free to open issues or submit pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/hrchlhck/url-shortener/blob/main/LICENSE) file for details.
