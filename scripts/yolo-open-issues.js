#!/usr/bin/env node
/**
 * YOLO Issue Opener
 * - Parses Phase 2.x ticket files under specs/001-selective-electron/*_TICKETS.md
 * - Creates GitHub issues for each ticket section
 * - Adds mandatory checklist to read .github/copilot-instructions.md
 * - Supports dry-run and phase filtering via env vars
 */

const fs = require('fs');
const path = require('path');
const { Octokit } = require('@octokit/rest');

const REPO_OWNER = process.env.GITHUB_REPOSITORY?.split('/')[0] || 'TayDa64';
const REPO_NAME = process.env.GITHUB_REPOSITORY?.split('/')[1] || 'Playa_Tay';
const GITHUB_TOKEN = process.env.GITHUB_TOKEN;
const DRY_RUN = (process.env.DRY_RUN || 'true').toLowerCase() === 'true';
const PHASES = (process.env.PHASES || '').split(',').map(s => s.trim()).filter(Boolean);

if (!GITHUB_TOKEN && !DRY_RUN) {
  console.error('GITHUB_TOKEN is required when DRY_RUN=false');
  process.exit(1);
}

const octokit = GITHUB_TOKEN ? new Octokit({ auth: GITHUB_TOKEN }) : null;

const TICKETS_DIR = path.resolve(process.cwd(), 'specs/001-selective-electron');

const FILES = [
  'PHASE_2_1_TICKETS.md',
  'PHASE_2_2_TICKETS.md',
  'PHASE_2_3_TICKETS.md',
  'PHASE_2_4_TICKETS.md',
  'PHASE_2_5_TICKETS.md',
  'PHASE_2_6_TICKETS.md',
];

function shouldIncludeFile(file) {
  if (PHASES.length === 0) return true;
  const match = file.match(/PHASE_(2_\d)_TICKETS/);
  if (!match) return false;
  const phase = match[1].replace('_', '.'); // e.g., 2_3 -> 2.3
  return PHASES.includes(phase);
}

function parseTickets(markdown) {
  // Split by top-level ticket headings starting with '## ['
  const sections = markdown.split(/\n##\s+\[Phase\s+2\.\d\][^\n]*\n/).slice(1);
  const headers = [...markdown.matchAll(/\n##\s+(\[Phase\s+2\.\d\][^\n]*)\n/g)].map(m => m[1]);
  const tickets = [];

  for (let i = 0; i < sections.length; i++) {
    const header = headers[i] || `Ticket ${i + 1}`;
    const body = sections[i].trim();

    // Extract title line: first '- Title:' occurrence
    const titleMatch = body.match(/-\s*Title:\s*(.+)/i);
    const fallbackTitle = header.replace(/\[|\]/g, '').trim();
    const title = titleMatch ? titleMatch[1].trim() : fallbackTitle;

    // Extract labels line: 'Labels:'
  const labelsMatch = body.match(/Labels:\s*([^\n]+)/i);
  const labels = labelsMatch ? labelsMatch[1].split(',').map(s => s.trim()) : [];

  const suggestedModels = suggestModels(labels);

    // Compose body with mandatory checklist
    const mandatoryChecklist = `\n\n---\n\n- [ ] Agent MUST read and follow .github/copilot-instructions.md before starting.\n- [ ] Follow Tauri-first architecture and repo conventions.\n- [ ] TypeScript: avoid any; use unknown + guards.\n- [ ] Do not use sleep in terminal commands.\n`;

    const modelBlock = suggestedModels.length
      ? `\n\n### Suggested models (pick best available in Copilot)\n- ${suggestedModels.join('\n- ')}\n`
      : '';

  const withYolo = labels.includes('yolo') ? labels : [...labels, 'yolo'];
  tickets.push({ title, body: body + modelBlock + mandatoryChecklist, labels: withYolo });
  }

  return tickets;
}

function suggestModels(labels) {
  const L = new Set(labels.map(l => l.toLowerCase()));
  const out = [];

  // Heuristics by label clusters
  const isRustSystems = ['rust','server','orchestrator','crypto','filesystem','networking','background-jobs','policy'].some(l => L.has(l));
  const isTsUi = ['typescript','react','ui','sdk','web'].some(l => L.has(l));
  const isCiEdge = ['ci','quality-gates','edge','cloudflare','cdn','analytics','metrics','observability','distribution','resiliency'].some(l => L.has(l));
  const isDocs = ['docs','ops','portal'].some(l => L.has(l));

  if (isRustSystems) {
    out.push('openai/o3 (reasoning, systems code)');
    out.push('openai/gpt-4.1 (long-context codegen)');
    // Premium multimodal option if available
    out.push('openai/gpt-5 (premium; complex orchestration)');
  }
  if (isTsUi) {
    out.push('openai/gpt-4.1 (TypeScript correctness)');
    out.push('openai/gpt-4.1-mini (faster, cost-efficient)');
    // Preview/code-focused
    out.push('openai/codex-mini (YAML/CLI-heavy tasks)');
  }
  if (isCiEdge) {
    out.push('openai/gpt-4.1 (infra, YAML, edge workers)');
    out.push('openai/gpt-4.1-mini (faster iterations)');
  }
  if (isDocs) {
    out.push('openai/gpt-4.1-mini (docs, examples)');
  }

  // De-dup
  return [...new Set(out)];
}

async function createIssue(ticket) {
  if (DRY_RUN) {
    console.log(`[DRY-RUN] Would create issue: ${ticket.title} with labels: ${ticket.labels.join(', ')}`);
    return;
  }
  await octokit.issues.create({
    owner: REPO_OWNER,
    repo: REPO_NAME,
    title: ticket.title,
    body: ticket.body,
    labels: ticket.labels,
  });
}

async function main() {
  const files = FILES.filter(f => shouldIncludeFile(f));
  if (files.length === 0) {
    console.log('No ticket files selected by phase filter.');
    return;
  }

  // Aggregate tickets and labels first
  const allTickets = [];
  const labelSet = new Set();
  for (const file of files) {
    const fullPath = path.join(TICKETS_DIR, file);
    if (!fs.existsSync(fullPath)) {
      console.warn(`Skipping missing file: ${fullPath}`);
      continue;
    }
    const md = fs.readFileSync(fullPath, 'utf8');
    const tickets = parseTickets(md);
    console.log(`Parsed ${tickets.length} tickets from ${file}`);
    tickets.forEach(t => t.labels.forEach(l => labelSet.add(l)));
    allTickets.push(...tickets);
  }

  // Ensure labels exist (non-dry-run only)
  if (!DRY_RUN) {
    const existing = new Set();
    const perPage = 100;
    let page = 1;
    // Paginate labels
    while (true) {
      const { data } = await octokit.issues.listLabelsForRepo({ owner: REPO_OWNER, repo: REPO_NAME, per_page: perPage, page });
      if (!data.length) break;
      data.forEach(l => existing.add(l.name));
      if (data.length < perPage) break;
      page += 1;
    }

    for (const name of labelSet) {
      if (!existing.has(name)) {
        try {
          await octokit.issues.createLabel({ owner: REPO_OWNER, repo: REPO_NAME, name, color: '0e8a16' });
        } catch (e) {
          console.warn(`Could not create label '${name}': ${e.message}`);
        }
      }
    }
  }

  // Create issues
  for (const ticket of allTickets) {
    await createIssue(ticket);
  }

  console.log('YOLO issue opening complete.');
}

main().catch(err => {
  console.error(err);
  process.exit(1);
});
