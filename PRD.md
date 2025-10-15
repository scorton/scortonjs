# Product Requirements Document (PRD) – ScortonJS

## 1. Introduction
**ScortonJS** is an open-source JavaScript/CLI framework for building, automating, and integrating cyber awareness, behavioral risk scoring, and security audit tools.  
It enables cybersecurity professionals, developers, and organizations to create **interactive awareness modules**, **automated audits**, and **real-time risk scoring pipelines** — with built-in adherence to ISACA, NIST CSF, and COBIT 5 principles.

---

## 2. Goals & Objectives
- **Primary Goal:** Provide a modular, developer-friendly toolkit for automating cybersecurity assessments and awareness campaigns.
- **Key Objectives:**
  1. Enable **rapid prototyping** of cyber awareness scenarios.
  2. Automate **cyber risk scoring** (Cyberscore) based on technical, behavioral, and organizational factors.
  3. Standardize **audit reporting** in line with ISACA/NIST frameworks.
  4. Support **both rapid Cyberscan (<1 min)** and **deep CyberAudit (5–10 days)** modes.
  5. Make security feedback **human-centric**, mapping results to personas/archetypes.

---

## 3. Target Audience
- **Primary Users:**
  - Cybersecurity consultants
  - Corporate SOC teams
  - Developers integrating security in workflows
  - Compliance & audit teams
- **Secondary Users:**
  - SMEs with limited security budgets
  - Security awareness trainers
  - Educational institutions

---

## 4. Core Features
### 4.1 Awareness & Training Modules
- CLI commands to launch phishing simulations, USB drop simulations, OSINT awareness challenges.
- Support for **behavioral quizzes** with dynamic difficulty.
- Gamified scoring mapped to **Scorton persona taxonomy**.

### 4.2 Automated Cyberscore Engine
- Multi-factor scoring:  
  `CS = (Technical × α) + (Behavioral × β) + (Organizational × γ)`  
  with adjustable weights.
- Data ingestion from:
  - **Technical scans** (Shodan, Nmap, CVE DB)
  - **Behavioral signals** (social media exposure, phishing susceptibility)
  - **Organizational maturity** (policies, governance).

### 4.3 Dual Assessment Modes
- **Cyberscan:** quick, API-driven risk snapshot (<1 minute).
- **CyberAudit:** detailed 5–10 day audit aligned to ISACA/NIST standards.

### 4.4 Reporting & Export
- Export results as:
  - **PDF (executive summary)**
  - **JSON/YAML (machine-readable)**
  - **Markdown (developer-friendly)**
- Visual dashboards with **risk heatmaps**.

### 4.5 Extensible Architecture
- Plugins for industry-specific audits (finance, healthcare, industrial).
- Support for custom scoring formulas & sector templates.
- REST API for integration into enterprise dashboards.

---

## 5. Technical Requirements
- **Language:** Node.js (backend), TypeScript (core), React.js (UI).
- **CLI-first** architecture (`scorton init`, `scorton scan`, `scorton audit`).
- **API-first** design for scoring & reporting modules.
- **Security:**
  - Sandbox execution of scans
  - Encrypted storage of sensitive data
- **Compliance:** Pre-built mappings to NIST CSF, ISACA COBIT, ISO 27001.

---

## 6. Success Metrics
- **Adoption Rate:** Number of organizations integrating ScortonJS in security workflows.
- **Module Growth:** Number of plugins/templates developed by the community.
- **Speed & Accuracy:** <1 min Cyberscan completion, >90% accuracy alignment with manual audits.
- **Engagement:** Completion rates for awareness training modules.

---

## 7. Future Roadmap
- **v2.0:** AI-driven adaptive training modules.
- **v2.1:** Threat intelligence integration for real-time score updates.
- **v3.0:** Full integration with Scorton Compass (business dashboard) & Scorton Radar (predictive risk tracking).
- **v3.1:** Cloud-hosted ScortonJS Hub for plugin sharing and collaboration.

---

## 8. Command Line Interface (CLI) Commands

### 8.1 Core Commands
| Command | Description | Example |
|---------|-------------|---------|
| `scorton init` | Initialize a new ScortonJS project with default config and templates. | `scorton init --template healthcare` |
| `scorton config` | View or edit configuration (API keys, weights, templates). | `scorton config set alpha=0.4 beta=0.4 gamma=0.2` |
| `scorton help` | Display help and available commands. | `scorton help audit` |
| `scorton update` | Update ScortonJS and its modules. | `scorton update` |

---

### 8.2 Audit & Scoring Commands
| Command | Description | Example |
|---------|-------------|---------|
| `scorton scan <target>` | Run a **Cyberscan** (quick <1 min check). | `scorton scan example.com --mode quick` |
| `scorton audit <target>` | Run a **full CyberAudit** (multi-day, deep assessment). | `scorton audit example.com --template finance` |
| `scorton score` | Calculate Cyberscore from data sources. | `scorton score --technical 70 --behavioral 60 --organizational 80` |
| `scorton report <id>` | Generate PDF/JSON/Markdown reports. | `scorton report 1234 --format pdf` |

---

### 8.3 Awareness & Training Commands
| Command | Description | Example |
|---------|-------------|---------|
| `scorton quiz` | Launch a behavioral cybersecurity quiz. | `scorton quiz --profile finance` |
| `scorton sim phishing` | Run a phishing simulation. | `scorton sim phishing --email ceo@example.com` |
| `scorton sim usb` | Simulate a USB drop test. | `scorton sim usb --location "HQ Reception"` |
| `scorton persona map` | Map user/org to LY Family persona. | `scorton persona map --user john` |

---

### 8.4 OSINT & Recon Commands
| Command | Description | Example |
|---------|-------------|---------|
| `scorton osint <target>` | Run OSINT data collection. | `scorton osint example.com --deep` |
| `scorton leakcheck <email>` | Check if an email/domain appears in data leaks. | `scorton leakcheck ceo@example.com` |
| `scorton social-listen <keyword>` | Monitor public social signals for threats. | `scorton social-listen "Acme Corp"` |

---

### 8.5 Developer Tools
| Command | Description | Example |
|---------|-------------|---------|
| `scorton module create` | Scaffold a new ScortonJS module. | `scorton module create --name phishing-sim` |
| `scorton module list` | List installed modules. | `scorton module list` |
| `scorton module install <url>` | Install module from registry or Git. | `scorton module install github:user/module` |
| `scorton api serve` | Run local REST API server for integration. | `scorton api serve --port 4000` |

---

### 8.6 Flags & Options
- `--output <path>` → Specify output folder for reports.
- `--format <pdf|json|md>` → Select output format.
- `--template <sector>` → Use sector-specific audit template.
- `--mode <quick|deep>` → Choose Cyberscan or CyberAudit mode.
- `--profile <persona>` → Use predefined persona for awareness/training.
- `--api-key <key>` → Pass API key for third-party integrations.

---

## 9. Appendix: Developer Resources

### 9.1 Suggested Project Structure