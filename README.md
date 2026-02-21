# 🔎 KYT – Know Your Target

**KYT (Know Your Target)** is a high-performance, persona-driven password generation engine written in Rust.

It is designed for:

- ✅ Authorized red-team simulations  
- ✅ Enterprise IAM password policy auditing  
- ✅ CTF challenge development  
- ✅ Security research & compliance testing  

KYT intelligently models how humans create passwords based on personal, professional, and contextual data.

---

# 🚀 Features

## 🧠 Persona-Based Mutation Engine
- Cross-category combinator (depth ≤ 3 / ≤ 4 in aggressive mode)
- Identity, Geography, Professional & Personal data modeling
- Intelligent date fragment derivation (DD, MM, YY, YYYY, DDMM)
- Reverse variants
- Case permutations
- L33t substitutions
- Symbol prefix / suffix / infix injection
- Numeric mask expansion (000–999 / 0000–9999 in aggressive mode)
- Policy-aware pruning
- Deduplicated output

---

## 🔥 Aggressive CTF Mode
Enable deeper combinator logic and expanded mutation space:

```bash
--aggressive
```

Adds:
- Depth 4 cross stacking
- Larger numeric mask space
- Extended symbol injection
- Multi-level l33t mutations

---

## ⚙ Streaming Bruteforce Engine
Optional full keyspace generation:

```bash
--bruteforce
```

- Cartesian charset enumeration
- Streaming output (no memory explosion)
- Resume checkpoint support
- Parallelized with Rayon

---

## 📊 Enterprise Password Policy Support
- Minimum / Maximum length
- Required uppercase
- Required lowercase
- Required numeric
- Required symbol
- Mandatory inclusion strings
- Exclusion strings
- Pre-validation before write

---

# 🏗 Architecture

```
KYT
 ├── interview.rs       # Interactive persona builder
 ├── models.rs          # Data structures
 ├── mutation.rs        # Persona mutation engine
 ├── bruteforce.rs      # Streaming brute engine
 ├── policy.rs          # Password policy validation
 ├── writer.rs          # Buffered output writer
 └── main.rs            # CLI entrypoint
```

Parallelization powered by:
- `rayon`
- `indicatif` (CLI progress)
- `clap` (CLI parsing)

---

# 📦 Installation

## Prerequisites

Install Rust:

### Windows
Download from:
https://rustup.rs

Or PowerShell:
```powershell
winget install Rustlang.Rustup
```

---

### Linux
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

---

### macOS
```bash
brew install rust
```

or via rustup:
```bash
curl https://sh.rustup.rs -sSf | sh
```

---

# 🔨 Build KYT

Clone the repository:

```bash
git clone https://github.com/YOUR_USERNAME/KYT-Know-Your-Target.git
cd KYT-Know-Your-Target
```

Build release binary:

```bash
cargo build --release
```

Binary will be located at:

```
target/release/KYT-Know_Your_Target
```

---

# 🖥 Deployment

## Windows

After building:

```
target\release\KYT-Know_Your_Target.exe
```

You can move the `.exe` into:

```
C:\Windows\System32
```

or any directory added to PATH.

---

## Linux

Move binary to:

```bash
sudo mv target/release/KYT-Know_Your_Target /usr/local/bin/kyt
```

Now run:

```bash
kyt
```

---

## macOS

Same as Linux:

```bash
sudo mv target/release/KYT-Know_Your_Target /usr/local/bin/kyt
```

---

# 🧪 Usage

## Interactive Mode

```bash
cargo run --release
```

---

## Aggressive CTF Mode

```bash
cargo run --release -- --aggressive
```

---

## JSON Persona Mode (CI/CD Friendly)

```bash
cargo run --release -- --json persona.json
```

---

## Streaming Bruteforce Mode

```bash
cargo run --release -- --bruteforce
```

Resume from checkpoint:

```bash
cargo run --release -- --bruteforce --resume 5000000
```

---

# 🧠 Example Output

For:

- Name: vedant
- Nicknames: vb, bond
- Birthdate: 07032006
- Policy: 6–8 chars, upper+lower+digit+symbol

KYT may generate:

```
Vb0600!
Bond06!
Vedant07!
V3d@nt06!
Bond_06!
```

---

# 📈 Performance

- Parallel mutation via Rayon
- HashSet-based deduplication
- Streaming mode for large-scale brute enumeration
- Efficient early length pruning
- Optimized release builds recommended

---

# 🔐 Security Notice

KYT is designed strictly for:

- Authorized penetration testing
- Security auditing
- Educational use
- CTF development

Do not use this tool against systems without explicit written authorization.

The author assumes no liability for misuse.

---

# 🛠 Roadmap

- [ ] Entropy scoring module
- [ ] Probability ranking engine
- [ ] Streaming mutation mode (no HashSet)
- [ ] Performance metrics dashboard
- [ ] Distributed generation support
- [ ] Wordlist export formats (hashcat/JTR masks)

---

# 📜 License

MIT License

---

# 👨‍💻 Author

Vedant Bondekar  
Cybersecurity Researcher  

---

# ⭐ If You Like This Project

Star the repository and contribute!

---