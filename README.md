## Data Preprocessing for Cybersecurity Threat Detection

### Overview

This project aims to provide an efficient data processing framework that prepares large datasets for a deep learning algorithm focused on lateral movement detection in cybersecurity. (for details of algorithm implementation, see [Bai et al. 2020](https://www.sciencedirect.com/science/article/abs/pii/S0140366420319617?via%3Dihub).

### Key Features

- Batch processing of large files (over 35GB compressed)
- Rust-based performance for time-efficient data processing
- Application in real-world big data scenarios and cybersecurity


## Getting Started

### Prerequisites

- Rust Programming Language
- Various Rust Libraries (`std::env`, `std::fs`, `bzip2`, `serde_json`, `rayon`)

### Installation

1. Clone the repository
2. Navigate to the project directory
3. Build the project with `cargo build`

## Usage

To run the project, execute the following command:

```bash
cargo run <path_to_bz2_files>
```

### Contributing
If you're interested in contributing to this data processing tool aimed at detecting lateral movement in cybersecurity, particularly involving RDP logs, you're welcome to reach out. I am are interested in expanding this tool to accommodate Python users and other real-world scenarios.

## Contact Information
For more information, questions, or collaborations, please contact me.
