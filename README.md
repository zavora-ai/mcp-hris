# HRIS MCP Server

[![Crates.io](https://img.shields.io/crates/v/mcp-hris.svg)](https://crates.io/crates/mcp-hris)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![ADK-Rust Enterprise](https://img.shields.io/badge/ADK--Rust-Enterprise-purple.svg)](https://enterprise.adk-rust.com)
[![Registry Ready](https://img.shields.io/badge/ADK_Registry-Ready-green.svg)](https://www.zavora.ai)

Give your AI agents full HR access — employees, departments, time-off, payroll, org charts, and headcount. 14 tools with seeded demo data that works out of the box.

## Architecture

<p align="center">
  <img src="https://raw.githubusercontent.com/zavora-ai/mcp-hris/main/docs/assets/architecture.svg" alt="MCP HRIS Architecture" width="800"/>
</p>

## Tools (14)

| Tool | Purpose | Risk Class |
|------|---------|------------|
| `list_employees` | List employees (optionally by department) | Read-only |
| `get_employee` | Get employee details by ID | Read-only |
| `create_employee` | Create a new employee record | Internal write |
| `update_employee` | Update title, department, manager, salary | Internal write |
| `list_departments` | List all departments | Read-only |
| `get_department` | Get department details | Read-only |
| `request_time_off` | Submit a time-off request | Internal write |
| `list_time_off` | List time-off requests | Read-only |
| `approve_time_off` | Approve or deny a request | Internal write |
| `list_payroll` | List payroll records by period | Read-only |
| `run_payroll` | Generate payroll for all active employees | Financial action |
| `get_org_chart` | Get org chart as a tree | Read-only |
| `get_headcount` | Headcount summary by department | Read-only |
| `get_directory` | Employee directory (name, email, title) | Read-only |

## Installation

```bash
cargo install mcp-hris
```

Or build from source:

```bash
git clone https://github.com/zavora-ai/mcp-hris
cd mcp-hris
cargo build --release
```

## Configuration

No configuration required — the server starts with seeded demo data (5 employees, 3 departments, payroll records, time-off requests).

Future backends (BambooHR, Workday, Gusto, Rippling) will use environment variables for API keys.

## Client Configuration

### Claude Desktop

```json
{
  "mcpServers": {
    "hris": {
      "command": "mcp-hris",
      "args": []
    }
  }
}
```

### Kiro

```json
{
  "mcpServers": {
    "hris": {
      "command": "mcp-hris",
      "args": []
    }
  }
}
```

### Cursor

```json
{
  "mcpServers": {
    "hris": {
      "command": "mcp-hris",
      "args": []
    }
  }
}
```

## Usage Examples

### List engineering team
```
"Show me all engineers"
→ calls list_employees with department_id for Engineering
```

### Request time off
```
"Request vacation for Bob from June 10-14"
→ calls request_time_off
```

### Run payroll
```
"Run payroll for May 2026"
→ calls run_payroll with period "2026-05"
```

### Org chart
```
"Show me the org chart"
→ calls get_org_chart — returns tree with reports
```

### Headcount
```
"What's the headcount by department?"
→ calls get_headcount — Engineering: 3, Product: 1, HR: 1
```

## Demo Data

The server starts with:
- **5 employees**: Alice Chen (VP Eng), Bob Smith (Sr Eng), Carol Davis (Head of Product), Dan Lee (Eng), Eve Johnson (HR Director)
- **3 departments**: Engineering, Product, HR
- **2 time-off requests**: Bob's vacation, Dan's sick day
- **2 payroll records**: April 2026

## MCP Server Manifest

```toml
server_id = "mcp_hris"
display_name = "HRIS"
version = "1.0.0"
domain = "business-systems"
risk_level = "medium"
writes_allowed = "gated"
transports = ["stdio"]
```

## Registry Compliance

- **HealthCheck** — async health probe for registry monitoring
- **mcp-server.toml** — manifest declaring tools, risk classes
- **Structured tracing** — `RUST_LOG` env-filter for observability

## License

Apache-2.0

---

Part of the [ADK-Rust Enterprise](https://enterprise.adk-rust.com) MCP server ecosystem.

Built with ❤️ by [Zavora AI](https://zavora.ai)
