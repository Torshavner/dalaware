# MCP Integration Guide

This document explains how to use Model Context Protocol (MCP) servers to extend Claude Code's capabilities within the Dreadnought project.

## What is MCP?

**Model Context Protocol (MCP)** is a standardized protocol that allows Claude Code to interact with external services (databases, APIs, file systems) through specialized MCP servers.

Think of MCP servers as microservices that expose specific capabilities to Claude Code:
- **Database MCP** → Query PostgreSQL directly
- **Grafana MCP** → Verify dashboards and panels
- **GitHub MCP** → Read issues, PRs, commits
- **Filesystem MCP** → Access files outside workspace

**C# Comparison:**
- MCP Server ≈ Microservice with REST API
- MCP Tools ≈ API endpoints (GET /dashboard/{uid})
- Claude Code MCP Client ≈ HttpClient wrapper
- MCP Configuration ≈ appsettings.json service registration

## Configured MCP Servers

### Grafana MCP Server (US-22)

**Purpose:** Programmatically interact with Grafana dashboards, datasources, and queries without manual UI access.

**Status:** ✅ Configured and ready to use

**Configuration File:** [.claude/mcp_servers.json](../.claude/mcp_servers.json)

**Environment Variables:**
```bash
GRAFANA_URL="http://localhost:3000"
GRAFANA_API_KEY="your_api_key_here"  # Generate in Grafana UI
```

**Available Tools:**
- `search_dashboards` - Find dashboards by name or list all
- `get_dashboard` - Retrieve dashboard JSON by UID
- `list_datasources` - List configured datasources (PostgreSQL, Loki)
- `query_datasource` - Execute queries against datasources

**Use Cases:**
1. Verify dashboard exists and has correct panels (US-3 lead-lag analytics)
2. Test panel queries return data
3. Export dashboard JSON to version control
4. Validate data flow from database to visualization

**Quick Start:** [docs/GRAFANA_MCP_SETUP.md](../docs/GRAFANA_MCP_SETUP.md)

**Full Documentation:** [docs/grafana_mcp_usage.md](../docs/grafana_mcp_usage.md)

**User Story:** [US-22](../docs/user_stories/22_grafana_mcp_integration.md)

### PostgreSQL MCP Server (US-21 - Not Yet Configured)

**Purpose:** Direct database queries without writing application code.

**Status:** ❌ Not configured (planned for US-21)

**Potential Tools:**
- `execute_query` - Run SQL queries
- `list_tables` - Show database schema
- `describe_table` - Get table structure

**Use Cases:**
- Verify migrations applied correctly
- Check data volume in tables
- Test SQL queries before adding to code
- Inspect hypertable configuration

**Configuration Example:**
```json
{
  "mcpServers": {
    "postgres": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-postgres"],
      "env": {
        "DATABASE_URL": "${DATABASE_URL}"
      }
    }
  }
}
```

### GitHub MCP Server (US-21 - Not Yet Configured)

**Purpose:** Access GitHub repository data (issues, PRs, commits) without browser.

**Status:** ❌ Not configured (planned for US-21)

**Potential Tools:**
- `list_issues` - Show open/closed issues
- `get_pull_request` - Retrieve PR details
- `list_commits` - View commit history

**Use Cases:**
- Link code changes to GitHub issues
- Review PR feedback without leaving IDE
- Check commit history for specific files

## How to Use MCP Servers

### Typical Workflow

**Without MCP (Manual):**
```
User: "Check if lead-lag dashboard has all 6 panels"
→ User opens browser
→ Navigate to Grafana UI (http://localhost:3000)
→ Login
→ Search for dashboard
→ Click dashboard
→ Count panels manually
→ Switch back to Claude Code
→ Report findings
Time: ~3-5 minutes, context switch
```

**With MCP (Automated):**
```
User: "Check if lead-lag dashboard has all 6 panels"
→ Claude Code uses Grafana MCP search_dashboards
→ Retrieves dashboard JSON via get_dashboard
→ Counts panels programmatically
→ Verifies panel types and titles
→ Reports findings
Time: ~10 seconds, no context switch
```

### Example: Verify US-3 Dashboard

**Command:**
```
Verify the lead-lag analytics dashboard matches US-3 requirements
```

**Claude Code Actions (via MCP):**
1. Read US-3 acceptance criteria → Expected: 6 panels
2. Call `search_dashboards(query="lead-lag")`
3. Call `get_dashboard(uid="lead-lag-analytics")`
4. Analyze JSON:
   - Panel count: 6 ✅
   - Panel 1: "Price Spread Over Time" (timeseries) ✅
   - Panel 2: "Current Latency Delta" (stat) ✅
   - Panel 3: "Exchange Leadership Heatmap" (heatmap) ✅
   - Panel 4: "Latency Distribution" (histogram) ✅
   - Panel 5: "Price vs Latency Scatter" (graph) ✅
   - Panel 6: "Recent Events Table" (table) ✅
5. Response: "Dashboard verified. All panels configured correctly."

### Example: Test Panel Query

**Command:**
```
Test if the price spread query returns data
```

**Claude Code Actions (via MCP):**
1. Call `get_dashboard(uid="lead-lag-analytics")`
2. Extract panel 1 query:
   ```sql
   SELECT timestamp, spread_bps
   FROM lead_lag_analytics
   WHERE timestamp > NOW() - INTERVAL '24 hours'
   ```
3. Call `query_datasource(datasource="PostgreSQL", query="...")`
4. Check result set: 150 rows returned
5. Response: "Query successful. Panel receiving data (150 data points in last 24h)."

### Example: Export Dashboard

**Command:**
```
Export the lead-lag dashboard to version control
```

**Claude Code Actions (via MCP):**
1. Call `get_dashboard(uid="lead-lag-analytics")`
2. Format JSON (pretty-print)
3. Write to `docs/grafana-visualizations/lead_lag_dashboard.json`
4. Response: "Dashboard exported to docs/grafana-visualizations/lead_lag_dashboard.json"

## Setup Instructions

### Step 1: Generate Grafana API Key

1. Start Grafana:
   ```bash
   cd docker
   docker-compose up -d grafana
   ```

2. Access Grafana UI: http://localhost:3000
   - Username: `admin`
   - Password: `admin`

3. Generate API Key:
   - Navigate to: **Configuration** (gear icon) → **API Keys**
   - Click **Add API key**
   - Name: `Claude Code MCP`
   - Role: **Admin** (full access) or **Viewer** (read-only)
   - Time to live: **Never** (local dev)
   - Click **Add** and copy the generated key

4. Update `.env`:
   ```bash
   GRAFANA_API_KEY="eyJrIjoiYXNkZmFzZGZhc2RmYXNkZiIsIm4iOiJDbGF1ZGUgQ29kZSBNQ1AiLCJpZCI6MX0="
   ```

### Step 2: Restart Claude Code

**Important:** Claude Code must be restarted to load MCP configuration changes.

1. Quit Claude Code completely
2. Restart Claude Code
3. Verify Grafana MCP tools available

### Step 3: Test MCP Connection

Try these commands:

**Test 1: List datasources**
```
Show me all Grafana datasources
```
Expected: PostgreSQL, Loki, etc.

**Test 2: Search dashboards**
```
Search for dashboards in Grafana
```
Expected: List of dashboards (or empty array if none created)

**Test 3: Get specific dashboard (if exists)**
```
Get the lead-lag analytics dashboard configuration
```
Expected: Dashboard JSON or "not found" message

## Troubleshooting

### Issue: MCP Server Not Available

**Symptom:** Grafana tools don't appear in Claude Code.

**Solutions:**
1. Verify `.claude/mcp_servers.json` exists:
   ```bash
   ls -la .claude/mcp_servers.json
   ```
2. Check Docker is running:
   ```bash
   docker ps
   ```
3. Verify `GRAFANA_API_KEY` set in `.env`:
   ```bash
   grep GRAFANA_API_KEY .env
   ```
4. Restart Claude Code

### Issue: "Unauthorized" Error

**Symptom:** MCP returns "Invalid API key" or "Unauthorized".

**Solutions:**
1. Regenerate API key in Grafana UI with **Admin** role
2. Update `GRAFANA_API_KEY` in `.env`
3. Restart Claude Code

### Issue: "Connection Refused"

**Symptom:** Cannot connect to `http://localhost:3000`.

**Solutions:**
1. Verify Grafana is running:
   ```bash
   curl http://localhost:3000/api/health
   ```
   Expected: `{"database":"ok","version":"..."}`

2. Start Grafana if stopped:
   ```bash
   cd docker && docker-compose up -d grafana
   ```

3. Check `GRAFANA_URL` in `.env`:
   ```bash
   cat .env | grep GRAFANA_URL
   # Should be: GRAFANA_URL="http://localhost:3000"
   ```

### Issue: Dashboard Not Found

**Symptom:** `get_dashboard` returns 404.

**Solutions:**
1. Use `search_dashboards` to find correct UID
2. Verify dashboard exists in Grafana UI
3. Check dashboard is saved (not just preview)

## MCP vs Direct API Calls

### Why Use MCP?

**Benefits:**
1. **Standardized Interface:** Consistent tool calling pattern
2. **Authentication Managed:** API keys handled by MCP server
3. **Type Safety:** MCP SDK provides typed interfaces
4. **Discoverability:** Tools self-document via MCP protocol
5. **Composability:** Multiple MCP servers work together

**Comparison:**

| Approach | Code Required | Maintenance | Type Safety |
|----------|--------------|-------------|-------------|
| Direct API | HttpClient + manual auth | High (URL changes) | Manual |
| MCP Server | Tool call | Low (MCP maintains) | Built-in |

**C# Analogy:**
- Direct API ≈ Writing raw HttpClient calls everywhere
- MCP Server ≈ Using a strongly-typed service client (like Refit or NSwag-generated clients)

## Integration with User Stories

### US-3: Lead-Lag Analytics

**Without MCP:**
- Create dashboard manually in Grafana UI
- Visually verify 6 panels exist
- Test queries by clicking "Edit" on each panel
- Hope configuration is correct

**With MCP:**
- Create dashboard (Grafana UI or MCP import)
- Verify via Claude Code: `Verify lead-lag dashboard matches US-3`
- Test queries programmatically
- Export to version control for CI/CD validation

**Benefit:** Test-driven dashboard development

### US-12: TimescaleDB Hypertable Optimization

**Without MCP:**
- Run SQL queries in pgAdmin
- Copy results to Claude Code
- Manually analyze retention policies

**With PostgreSQL MCP (US-21):**
- Query hypertable config directly: `Show hypertable configuration`
- Check compression settings: `List compressed chunks`
- Verify retention policy: `Show data older than 30 days`

**Benefit:** Direct database introspection without tool switching

## Security Considerations

### API Key Storage

**✅ Correct:**
```bash
# .env
GRAFANA_API_KEY="secret_key_here"
```
- Stored in `.env` (ignored by git)
- Never hardcoded in code
- Loaded via environment variable substitution

**❌ Incorrect:**
```json
{
  "mcpServers": {
    "grafana": {
      "env": {
        "GRAFANA_API_KEY": "secret_key_here"  // NEVER do this!
      }
    }
  }
}
```

### MCP Configuration Best Practices

1. **Read-Only by Default:** Use Viewer API key role unless modifications needed
2. **Local Only:** MCP configured for `localhost:3000` (not production)
3. **Audit Logging:** MCP server logs all operations
4. **Key Rotation:** Regenerate API keys every 90 days (local dev: never)

## Development Workflow

### Before MCP

```
1. Write code
2. Run application
3. Open browser → Grafana
4. Login
5. Navigate to dashboard
6. Visually inspect panels
7. Click "Edit" to check queries
8. Go back to Claude Code
9. Report findings
```

**Time:** ~5-10 minutes per verification
**Context switches:** 2-3

### With MCP

```
1. Write code
2. Run application
3. Ask Claude Code: "Verify dashboard"
4. Claude Code uses MCP → instant feedback
```

**Time:** ~10-30 seconds
**Context switches:** 0

**Productivity gain:** ~80-90% reduction in verification time

## MCP Architecture

```
┌─────────────────┐
│  Claude Code    │
│  (MCP Client)   │
└────────┬────────┘
         │
         │ MCP Protocol (JSON-RPC)
         │
    ┌────▼─────┬──────────┬──────────┐
    │          │          │          │
┌───▼───┐ ┌───▼───┐ ┌───▼───┐ ┌───▼───┐
│Grafana│ │Postgres│ │GitHub │ │FS MCP│
│  MCP  │ │  MCP   │ │  MCP  │ │Server │
└───┬───┘ └───┬────┘ └───┬───┘ └───┬───┘
    │         │          │         │
    │    HTTP │     HTTP │    File │
    │    API  │     SQL  │    I/O  │
    │         │          │         │
┌───▼─────────▼──────────▼─────────▼───┐
│   External Services & Resources      │
│  (Grafana, PostgreSQL, GitHub, etc.) │
└──────────────────────────────────────┘
```

## Next Steps

1. **Complete US-22 Setup:**
   - Generate Grafana API key
   - Update `.env` with API key
   - Restart Claude Code
   - Test: `Show me all Grafana datasources`

2. **Implement US-3 Dashboard:**
   - Create lead-lag analytics dashboard
   - Verify via MCP: `Verify lead-lag dashboard`
   - Export to version control

3. **Explore US-21:**
   - Configure PostgreSQL MCP server
   - Configure GitHub MCP server
   - Test database queries via MCP

4. **Document Workflows:**
   - Add MCP verification to testing guidelines
   - Update user stories with MCP validation steps
   - Create dashboard export/import workflow

## References

### Documentation
- **MCP Quick Start:** [docs/GRAFANA_MCP_SETUP.md](../docs/GRAFANA_MCP_SETUP.md)
- **MCP Usage Guide:** [docs/grafana_mcp_usage.md](../docs/grafana_mcp_usage.md)
- **US-22 (Grafana MCP):** [docs/user_stories/22_grafana_mcp_integration.md](../docs/user_stories/22_grafana_mcp_integration.md)
- **US-21 (General MCP):** [docs/user_stories/21_mcp_server_integration.md](../docs/user_stories/21_mcp_server_integration.md)

### Official MCP Resources
- **MCP Protocol Spec:** https://modelcontextprotocol.io
- **Grafana MCP Server:** https://github.com/grafana/mcp-grafana
- **Grafana HTTP API:** https://grafana.com/docs/grafana/latest/developers/http_api/
- **MCP TypeScript SDK:** https://github.com/modelcontextprotocol/typescript-sdk

### Related Tools
- **Loki MCP Server:** `grafana/loki-mcp` (log queries)
- **Tempo MCP Server:** `grafana/tempo-mcp-server` (distributed tracing)
- **PostgreSQL MCP Server:** `@modelcontextprotocol/server-postgres`

## Common Lingo

| Term | Meaning | Usage |
|------|---------|-------|
| MCP | Model Context Protocol | "Use MCP to verify dashboard" |
| MCP Server | External service adapter | "Grafana MCP server" |
| MCP Tool | Callable function | "Call get_dashboard tool" |
| MCP Client | Claude Code's MCP interface | "MCP client sends request" |
| Dashboard UID | Grafana dashboard identifier | "UID: lead-lag-analytics" |
| Datasource | Grafana data source | "PostgreSQL datasource" |
| Panel | Grafana visualization | "Panel 1: Time series" |

## Version Info

- **MCP Protocol Version:** 1.0
- **Grafana MCP Server:** Official `mcp/grafana` Docker image
- **Configuration File:** [.claude/mcp_servers.json](../.claude/mcp_servers.json)
- **Last Updated:** 2025-12-17
- **Status:** Grafana MCP configured and ready, PostgreSQL/GitHub MCP planned (US-21)
