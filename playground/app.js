import init, { compile_playground } from "./pkg/facharbeit.js";

const examples = {
  TinyPrint: {
    source: `// This is the smallest useful program in the playground.
// It calls print with one integer so you can inspect a very short token stream.
fn main() {
    // Print the number 7.
    print(7);
    // Stop the program.
    return;
}
`,
    args: "",
  },
  NestedMath: {
    source: `// This example is intentionally heavy on nested arithmetic.
// Its main purpose is to make the AST and WAT views visually interesting.
fn main() {
    // Build a deeply nested arithmetic expression step by step in one line.
    let result = ((3 + 4) * (10 - 2)) + ((18 / 3) * (5 + (2 * 2)));
    // Print the final value so the runtime output is easy to verify.
    print(result);
    // End the program.
    return;
}
`,
    args: "",
  },
  Factorial: {
    source: `// This is the classic recursive factorial example.
// It demonstrates recursion, comparison, multiplication, and nested calls.
fn fact(n) -> Int {
    // If n is 1 or smaller, the factorial is 1.
    if (n <= 1) {
        return 1;
    } else {
        // Otherwise multiply n by factorial(n - 1).
        return n * fact(n - 1);
    }
}

fn main() {
    // Print 10! so the result is large enough to feel impressive.
    print(fact(10));
    // End the program.
    return;
}
`,
    args: "",
  },
  FunctionChain: {
    source: `// This example uses several small functions that call each other.
// It is useful for understanding how function calls are wired through the compiler.
fn add(a, b) -> Int {
    // Return the sum of a and b.
    return a + b;
}

fn double_value(n) -> Int {
    // Double n by reusing add.
    return add(n, n);
}

fn square(n) -> Int {
    // Multiply n by itself.
    return n * n;
}

fn pipeline(n) -> Int {
    // First double the input, then square the result, then add 3.
    return square(double_value(n)) + 3;
}

fn main() {
    // Print the result of the whole pipeline for the input 5.
    print(pipeline(5));
    // End the program.
    return;
}
`,
    args: "",
  },
  NumberClassifier: {
    source: `// This program classifies a number into several named buckets.
// It is longer on purpose so the generated WAT has more structure to inspect.
fn classify(n) -> Int {
    // Return 100 for very small numbers.
    if (n <= 2) {
        return 100;
    } else {
        // Return 200 for numbers up to 5.
        if (n <= 5) {
            return 200;
        } else {
            // Return 300 for numbers up to 10.
            if (n <= 10) {
                return 300;
            } else {
                // Return 400 for anything larger.
                return 400;
            }
        }
    }
}

fn score(a, b, c) -> Int {
    // Add the classifications of three different values.
    return classify(a) + classify(b) + classify(c);
}

fn main() {
    // Print a combined score built from three separate classifications.
    print(score(1, 4, 12));
    // End the program.
    return;
}
`,
    args: "",
  },
  MiniToolkit: {
    source: `// This is the largest demo program in the playground.
// It combines recursion, multiple helper functions, nested calls, and many expressions.
fn max(a, b) -> Int {
    // Return the larger of the two inputs.
    if (a >= b) {
        return a;
    } else {
        return b;
    }
}

fn min(a, b) -> Int {
    // Return the smaller of the two inputs.
    if (a <= b) {
        return a;
    } else {
        return b;
    }
}

fn clamp(value, low, high) -> Int {
    // Keep value inside the interval from low to high.
    return min(max(value, low), high);
}

fn triangular(n) -> Int {
    // Compute 1 + 2 + ... + n recursively.
    if (n <= 1) {
        return 1;
    } else {
        return n + triangular(n - 1);
    }
}

fn weighted_value(a, b, c) -> Int {
    // Mix three inputs with different weights.
    return (a * 3) + (b * 2) + c;
}

fn final_score(seed) -> Int {
    // Build a larger expression from several helper functions.
    return clamp(weighted_value(seed, triangular(4), max(seed + 5, 9)), 10, 80);
}

fn main() {
    // Print the final score for the seed value 6.
    print(final_score(6));
    // End the program.
    return;
}
`,
    args: "",
  },
  BranchingShowcase: {
    source: `// This example focuses on boolean-style comparisons and nested decisions.
// It is useful when you want to inspect how conditions appear in the AST and WAT.
fn choose(a, b, c) -> Int {
    // If a is the largest value, return a special code.
    if ((a > b) == (a > c)) {
        return a + 100;
    } else {
        // Otherwise compare b and c and return a different code.
        if (b > c) {
            return b + 200;
        } else {
            return c + 300;
        }
    }
}

fn main() {
    // Print the chosen result for three concrete inputs.
    print(choose(4, 9, 7));
    // End the program.
    return;
}
`,
    args: "",
  },
};

const sourceEditor = document.querySelector("#source-editor");
const sourceHighlighted = document.querySelector("#source-highlighted");
const exampleSelect = document.querySelector("#example-select");
const mainArgsInput = document.querySelector("#main-args");
const compileButton = document.querySelector("#compile-button");
const runButton = document.querySelector("#run-button");
const compilerStatus = document.querySelector("#compiler-status");
const mainArity = document.querySelector("#main-arity");
const runtimeOutput = document.querySelector("#runtime-output");
const tokensOutput = document.querySelector("#tokens-output");
const astOutput = document.querySelector("#ast-output");
const watCode = document.querySelector("#wat-code");
const paneDivider = document.querySelector("#pane-divider");
const astZoomOutButton = document.querySelector("#ast-zoom-out");
const astZoomResetButton = document.querySelector("#ast-zoom-reset");
const astZoomInButton = document.querySelector("#ast-zoom-in");
const tabButtons = Array.from(document.querySelectorAll(".tab-button"));
const tabPanels = Array.from(document.querySelectorAll(".tab-panel"));

let compilerReady = false;
let lastCompilation = null;
let astZoom = 1;
let astBaseWidth = 0;
let astBaseHeight = 0;

function escapeHtml(value) {
  return value
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;");
}

function highlightSource(source) {
  const patterns = [
    [/^\/\/[^\n]*/, "token-comment"],
    [/^(fn|let|if|else|while|return)\b/, "token-keyword"],
    [/^Int\b/, "token-type"],
    [/^-?\d+\b/, "token-number"],
    [/^[A-Za-z_][A-Za-z0-9_]*(?=\s*\()/, "token-function"],
    [/^(->|==|!=|<=|>=|[=+\-*/%<>])/, "token-operator"],
    [/^[(){};,]/, "token-punctuation"],
    [/^\s+/, null],
    [/^[A-Za-z_][A-Za-z0-9_]*/, null],
    [/^./, null],
  ];

  let remaining = source;
  let html = "";

  while (remaining.length > 0) {
    let matched = false;

    for (const [pattern, className] of patterns) {
      const match = remaining.match(pattern);
      if (!match) {
        continue;
      }

      const [text] = match;
      const escaped = escapeHtml(text);
      html += className ? `<span class="${className}">${escaped}</span>` : escaped;
      remaining = remaining.slice(text.length);
      matched = true;
      break;
    }

    if (!matched) {
      html += escapeHtml(remaining[0]);
      remaining = remaining.slice(1);
    }
  }

  sourceHighlighted.innerHTML = `${html}\n`;
}

function syncEditorScroll() {
  sourceHighlighted.scrollTop = sourceEditor.scrollTop;
  sourceHighlighted.scrollLeft = sourceEditor.scrollLeft;
}

function setExample(name) {
  const example = examples[name];
  sourceEditor.value = example.source;
  mainArgsInput.value = example.args;
  runtimeOutput.textContent = "Ready.";
  highlightSource(example.source);
  syncEditorScroll();
}

function parseArgs(rawValue, expectedCount) {
  const trimmed = rawValue.trim();
  if (expectedCount === 0) {
    if (trimmed.length !== 0) {
      throw new Error("main takes no arguments.");
    }
    return [];
  }

  if (trimmed.length === 0) {
    throw new Error(`main expects ${expectedCount} argument(s).`);
  }

  const values = trimmed.split(",").map((segment) => segment.trim());
  if (values.length !== expectedCount) {
    throw new Error(`main expects ${expectedCount} argument(s), received ${values.length}.`);
  }

  return values.map((value) => {
    if (!/^-?\d+$/.test(value)) {
      throw new Error(`invalid integer argument: ${value}`);
    }
    return BigInt(value);
  });
}

function tokenKindToLabel(kind) {
  if (typeof kind === "string") {
    return kind;
  }

  const [[variant, value]] = Object.entries(kind);
  return `${variant}: ${value}`;
}

function formatTokenTitle(token, source) {
  const lexeme = formatTokenLexeme(token, source);

  if (typeof token.kind === "string") {
    if (["Fn", "If", "Else", "While", "Return", "Let", "IntType", "EOF"].includes(token.kind)) {
      return token.kind;
    }
    return `${token.kind} ${lexeme}`;
  }

  const [[variant]] = Object.entries(token.kind);
  if (variant === "Ident" || variant === "Int") {
    return `${variant}: ${lexeme}`;
  }

  return `${tokenKindToLabel(token.kind)} ${lexeme}`;
}

function tokenKindToCategory(kind) {
  if (typeof kind === "string") {
    if (["Let", "Fn", "If", "Else", "While", "Return"].includes(kind)) {
      return "keyword";
    }
    if (["Ident", "Int"].includes(kind)) {
      return "literal";
    }
    if (["LParen", "RParen", "LBrace", "RBrace", "Semicolon", "Comma", "Colon", "Arrow"].includes(kind)) {
      return "delimiter";
    }
    if (["IntType"].includes(kind)) {
      return "type";
    }
    return "symbol";
  }

  const [variant] = Object.keys(kind);
  if (variant === "Ident" || variant === "Int") {
    return "literal";
  }
  return tokenKindToCategory(variant);
}

function formatTokenLexeme(token, source) {
  const raw = source.slice(token.span.start, token.span.end);
  if (raw.length === 0) {
    return "EOF";
  }

  return JSON.stringify(raw);
}

function renderTokens(tokens) {
  tokensOutput.innerHTML = "";
  const source = sourceEditor.value;

  const list = document.createElement("div");
  list.className = "token-list";

  for (const token of tokens) {
    const item = document.createElement("article");
    item.className = `token-card token-card-${tokenKindToCategory(token.kind)}`;

    const kind = document.createElement("div");
    kind.className = "token-card-kind";
    kind.textContent = formatTokenTitle(token, source);

    const span = document.createElement("div");
    span.className = "token-card-span";
    span.textContent = `${token.span.start}..${token.span.end}`;

    item.append(kind, span);
    list.append(item);
  }

  tokensOutput.append(list);
}

function describePrimitive(value) {
  return typeof value === "string" ? `"${value}"` : String(value);
}

function isVariantObject(entries) {
  if (entries.length !== 1) {
    return false;
  }

  const [key] = entries[0];
  return /^[A-Z]/.test(key);
}

function opLabel(op) {
  const names = {
    Add: "+",
    Sub: "-",
    Mul: "*",
    Div: "/",
    Eq: "==",
    NotEq: "!=",
    Lt: "<",
    Le: "<=",
    Gt: ">",
    Ge: ">=",
  };
  return names[op] ?? op;
}

function toDiagramNode(label, value) {
  if (value === null || typeof value !== "object") {
    return {
      label,
      meta: describePrimitive(value),
      kind: "primitive",
      children: [],
    };
  }

  if (Array.isArray(value)) {
    if (label === "functions") {
      return {
        label: "functions",
        meta: `[${value.length}]`,
        kind: "group",
        children: value.map((child, index) => {
          if (child && typeof child === "object" && typeof child.name === "string") {
            return toDiagramNode(`function ${child.name}`, child);
          }
          return toDiagramNode(`function ${index + 1}`, child);
        }),
      };
    }

    if (label === "params") {
      return {
        label: "params",
        meta: `[${value.length}]`,
        kind: "group",
        children: value.map((child, index) => ({
          label: String(child),
          meta: "param",
          kind: "param",
          edgeLabel: `param ${index + 1}`,
          children: [],
        })),
      };
    }

    if (label === "body" || label === "then" || label === "else") {
      return {
        label,
        meta: `[${value.length}]`,
        kind: "group",
        children: value.map((child, index) => ({
          ...toDiagramNode("", child),
          edgeLabel: `${label} ${index + 1}`,
        })),
      };
    }

    return {
      label,
      meta: `[${value.length}]`,
      kind: "group",
      children: value.map((child, index) => toDiagramNode(String(index), child)),
    };
  }

  const entries = Object.entries(value);
  if (isVariantObject(entries)) {
    const [variant, payload] = entries[0];

    if (variant === "Binary") {
      return {
        label: opLabel(payload.op),
        meta: "binary",
        kind: "binary",
        children: [
          { ...toDiagramNode("", payload.left), edgeLabel: "left" },
          { ...toDiagramNode("", payload.right), edgeLabel: "right" },
        ],
      };
    }

    if (variant === "Call") {
      return {
        label: payload.name,
        meta: `call, ${payload.args.length} arg${payload.args.length === 1 ? "" : "s"}`,
        kind: "call",
        children: payload.args.map((arg, index) => ({
          ...toDiagramNode("", arg),
          edgeLabel: `arg ${index + 1}`,
        })),
      };
    }

    if (variant === "Local") {
      return {
        label: payload,
        meta: "local",
        kind: "local",
        children: [],
      };
    }

    if (variant === "Int") {
      return {
        label: String(payload),
        meta: "int",
        kind: "literal",
        children: [],
      };
    }

    if (variant === "Let") {
      return {
        label: payload.name,
        meta: "let",
        kind: "let",
        children: [{ ...toDiagramNode("", payload.value), edgeLabel: "value" }],
      };
    }

    if (variant === "Return") {
      return {
        label: "return",
        meta: payload === null ? "void" : "",
        kind: "return",
        children: payload === null ? [] : [{ ...toDiagramNode("", payload), edgeLabel: "value" }],
      };
    }

    if (variant === "Expr") {
      return toDiagramNode("", payload);
    }

    if (variant === "If") {
      return {
        label: "if",
        meta: "control",
        kind: "control",
        children: [
          { ...toDiagramNode("", payload.cond), edgeLabel: "cond" },
          { ...toDiagramNode("then", payload.then_block), edgeLabel: "then" },
          { ...toDiagramNode("else", payload.else_block), edgeLabel: "else" },
        ],
      };
    }

    if (variant === "While") {
      return {
        label: "while",
        meta: "control",
        kind: "control",
        children: [
          { ...toDiagramNode("", payload.cond), edgeLabel: "cond" },
          { ...toDiagramNode("body", payload.body), edgeLabel: "body" },
        ],
      };
    }

    if (variant === "FunctionDecl") {
      return {
        label: payload.name ?? "function",
        meta: "function",
        kind: "function",
        children: [
          toDiagramNode("params", payload.params),
          toDiagramNode("body", payload.body),
        ],
      };
    }

    if (payload !== null && typeof payload === "object" && !Array.isArray(payload)) {
      return {
        label: label ? `${label}: ${variant}` : variant,
        meta: "",
        kind: "variant",
        children: Object.entries(payload).map(([key, child]) => toDiagramNode(key, child)),
      };
    }

    return {
      label: label ? `${label}: ${variant}` : variant,
      meta: payload === null ? "null" : describePrimitive(payload),
      kind: "variant",
      children: [],
    };
  }

  if (label.startsWith("function ") && typeof value.name === "string") {
    return {
      label: value.name,
      meta: "function",
      kind: "function",
      children: [
        toDiagramNode("params", value.params),
        toDiagramNode("body", value.body),
      ],
    };
  }

  return {
    label,
    meta: "",
    kind: "group",
    children: entries.map(([key, child]) => toDiagramNode(key, child)),
  };
}

function layoutDiagram(node, depth = 0, state = { nextY: 0, nodes: [], edges: [], maxDepth: 0 }) {
  const xGap = 250;
  const yGap = 62;
  const nodeWidth = 144;
  const nodeHeight = 46;

  const x = 24 + depth * xGap;
  state.maxDepth = Math.max(state.maxDepth, depth);

  if (node.children.length === 0) {
    const y = 18 + state.nextY * yGap;
    state.nextY += 1;
    state.nodes.push({ ...node, x, y, width: nodeWidth, height: nodeHeight });
    return { x, y, width: nodeWidth, height: nodeHeight };
  }

  const childLayouts = node.children.map((child) => layoutDiagram(child, depth + 1, state));
  const first = childLayouts[0];
  const last = childLayouts[childLayouts.length - 1];
  const y = first.y + (last.y - first.y) / 2;

  const current = { ...node, x, y, width: nodeWidth, height: nodeHeight };
  state.nodes.push(current);

  for (const child of childLayouts) {
    state.edges.push({
      x1: x + nodeWidth,
      y1: y + nodeHeight / 2,
      x2: child.x,
      y2: child.y + child.height / 2,
      label: child.edgeLabel ?? "",
    });
  }

  return current;
}

function renderAst(ast) {
  astOutput.innerHTML = "";

  const astTree = toDiagramNode("Program", ast);
  const state = { nextY: 0, nodes: [], edges: [], maxDepth: 0 };
  layoutDiagram(astTree, 0, state);

  const width = (state.maxDepth + 1) * 340 + 172;
  const height = Math.max(state.nextY * 62 + 32, 220);
  const ns = "http://www.w3.org/2000/svg";

  const canvas = document.createElement("div");
  canvas.className = "ast-canvas";

  const svg = document.createElementNS(ns, "svg");
  svg.setAttribute("class", "ast-diagram");
  svg.setAttribute("viewBox", `0 0 ${width} ${height}`);
  svg.setAttribute("width", String(width));
  svg.setAttribute("height", String(height));

  const edgeGroup = document.createElementNS(ns, "g");
  edgeGroup.setAttribute("class", "ast-edges");
  for (const edge of state.edges) {
    const path = document.createElementNS(ns, "path");
    const midX = edge.x1 + (edge.x2 - edge.x1) / 2;
    path.setAttribute(
      "d",
      `M ${edge.x1} ${edge.y1} C ${midX} ${edge.y1}, ${midX} ${edge.y2}, ${edge.x2} ${edge.y2}`,
    );
    edgeGroup.append(path);

    if (edge.label) {
      const text = document.createElementNS(ns, "text");
      text.setAttribute("x", String(midX));
      text.setAttribute("y", String((edge.y1 + edge.y2) / 2 - 6));
      text.setAttribute("text-anchor", "middle");
      text.setAttribute("class", "ast-edge-label");
      text.textContent = edge.label;
      edgeGroup.append(text);
    }
  }
  svg.append(edgeGroup);

  const nodeGroup = document.createElementNS(ns, "g");
  nodeGroup.setAttribute("class", "ast-nodes");
  for (const node of state.nodes) {
    const group = document.createElementNS(ns, "g");
    group.setAttribute("transform", `translate(${node.x}, ${node.y})`);
    group.setAttribute("class", `ast-node ast-node-${node.kind ?? "default"}`);

    const rect = document.createElementNS(ns, "rect");
    rect.setAttribute("width", String(node.width));
    rect.setAttribute("height", String(node.height));
    rect.setAttribute("rx", "12");
    group.append(rect);

    const label = document.createElementNS(ns, "text");
    label.setAttribute("x", "12");
    label.setAttribute("y", "20");
    label.setAttribute("class", "ast-node-label");
    label.textContent = node.label;
    group.append(label);

    if (node.meta) {
      const meta = document.createElementNS(ns, "text");
      meta.setAttribute("x", "12");
      meta.setAttribute("y", "34");
      meta.setAttribute("class", "ast-node-meta");
      meta.textContent = node.meta;
      group.append(meta);
    }

    nodeGroup.append(group);
  }
  svg.append(nodeGroup);

  canvas.append(svg);
  astOutput.append(canvas);
  astBaseWidth = width;
  astBaseHeight = height;
  applyAstZoom();
}

function renderWat(wat) {
  if (window.hljs) {
    watCode.innerHTML = window.hljs.highlight(wat, { language: "wasm" }).value;
  } else {
    watCode.textContent = wat;
  }
}

function clearStructuredOutputs() {
  tokensOutput.innerHTML = "";
  astOutput.innerHTML = "";
  watCode.textContent = "";
}

function applyAstZoom() {
  const canvas = astOutput.querySelector(".ast-canvas");
  const diagram = astOutput.querySelector(".ast-diagram");
  astZoomResetButton.textContent = `${Math.round(astZoom * 100)}%`;
  if (!diagram || !canvas) {
    return;
  }

  canvas.style.width = `${astBaseWidth * astZoom}px`;
  canvas.style.height = `${astBaseHeight * astZoom}px`;
  diagram.style.transform = `scale(${astZoom})`;
}

function setAstZoom(nextZoom, focusClientX = null, focusClientY = null) {
  const previousZoom = astZoom;
  const clampedZoom = Math.max(0.35, Math.min(3, nextZoom));
  if (clampedZoom === previousZoom) {
    return;
  }

  const rect = astOutput.getBoundingClientRect();
  const focusX = focusClientX === null ? rect.left + rect.width / 2 : focusClientX;
  const focusY = focusClientY === null ? rect.top + rect.height / 2 : focusClientY;
  const localX = focusX - rect.left + astOutput.scrollLeft;
  const localY = focusY - rect.top + astOutput.scrollTop;
  const contentX = localX / previousZoom;
  const contentY = localY / previousZoom;

  astZoom = clampedZoom;
  applyAstZoom();

  astOutput.scrollLeft = contentX * astZoom - (focusX - rect.left);
  astOutput.scrollTop = contentY * astZoom - (focusY - rect.top);
}

function writeCompileOutputs(result) {
  const tokens = JSON.parse(result.tokens_json);
  const ast = JSON.parse(result.ast_json);

  renderTokens(tokens);
  renderAst(ast);
  renderWat(result.wat);
  mainArity.textContent = `main arity: ${result.main_param_count}`;
}

function activateTab(tabName) {
  for (const button of tabButtons) {
    const isActive = button.dataset.tab === tabName;
    button.classList.toggle("is-active", isActive);
    button.setAttribute("aria-selected", String(isActive));
  }

  for (const panel of tabPanels) {
    panel.classList.toggle("is-active", panel.dataset.panel === tabName);
  }
}

async function compileCurrentSource() {
  if (!compilerReady) {
    throw new Error("compiler is still loading.");
  }

  compilerStatus.textContent = "Compiling...";
  runtimeOutput.textContent = "Compilation finished. Ready to run.";

  const result = compile_playground(sourceEditor.value);
  lastCompilation = result;
  writeCompileOutputs(result);
  compilerStatus.textContent = "Compilation successful.";
  return result;
}

async function runCurrentProgram() {
  const result = lastCompilation ?? (await compileCurrentSource());
  const args = parseArgs(mainArgsInput.value, result.main_param_count);
  const printed = [];

  compilerStatus.textContent = "Instantiating module...";

  const { instance } = await WebAssembly.instantiate(result.wasm_bytes, {
    env: {
      print_i64(value) {
        printed.push(value.toString());
      },
    },
  });

  const main = instance.exports.main;
  if (typeof main !== "function") {
    throw new Error("compiled module does not export a main function.");
  }

  compilerStatus.textContent = "Running...";

  const returnValue = main(...args);
  const lines = [];

  if (printed.length > 0) {
    lines.push(`print output:\n${printed.join("\n")}`);
  } else {
    lines.push("print output:\n<none>");
  }

  if (returnValue === undefined) {
    lines.push("main returned no value");
  } else {
    lines.push(`main returned: ${returnValue.toString()}`);
  }

  runtimeOutput.textContent = lines.join("\n\n");
  compilerStatus.textContent = "Run complete.";
}

function populateExamples() {
  for (const name of Object.keys(examples)) {
    const option = document.createElement("option");
    option.value = name;
    option.textContent = name;
    exampleSelect.append(option);
  }

  exampleSelect.addEventListener("change", () => {
    setExample(exampleSelect.value);
    lastCompilation = null;
    compilerStatus.textContent = "Source changed. Compile again.";
    mainArity.textContent = "main arity: -";
    clearStructuredOutputs();
  });

  exampleSelect.value = "Factorial";
  setExample("Factorial");
}

compileButton.addEventListener("click", async () => {
  try {
    await compileCurrentSource();
  } catch (error) {
    lastCompilation = null;
    compilerStatus.textContent = "Compilation failed.";
    runtimeOutput.textContent = error instanceof Error ? error.message : String(error);
  }
});

runButton.addEventListener("click", async () => {
  try {
    await runCurrentProgram();
  } catch (error) {
    compilerStatus.textContent = "Run failed.";
    runtimeOutput.textContent = error instanceof Error ? error.message : String(error);
  }
});

sourceEditor.addEventListener("input", () => {
  highlightSource(sourceEditor.value);
  lastCompilation = null;
  compilerStatus.textContent = compilerReady
    ? "Source changed. Compile again."
    : "Loading compiler...";
  mainArity.textContent = "main arity: -";
  clearStructuredOutputs();
  runtimeOutput.textContent = "Ready.";
});

sourceEditor.addEventListener("scroll", syncEditorScroll);

populateExamples();

for (const button of tabButtons) {
  button.addEventListener("click", () => {
    activateTab(button.dataset.tab);
  });
}

astZoomOutButton.addEventListener("click", () => {
  setAstZoom(Math.round((astZoom - 0.1) * 10) / 10);
});

astZoomInButton.addEventListener("click", () => {
  setAstZoom(Math.round((astZoom + 0.1) * 10) / 10);
});

astZoomResetButton.addEventListener("click", () => {
  setAstZoom(1);
});

let astPanState = null;

astOutput.addEventListener("pointerdown", (event) => {
  if (event.button !== 0) {
    return;
  }

  event.preventDefault();
  astPanState = {
    pointerId: event.pointerId,
    startX: event.clientX,
    startY: event.clientY,
    scrollLeft: astOutput.scrollLeft,
    scrollTop: astOutput.scrollTop,
  };
  astOutput.classList.add("is-panning");
  astOutput.setPointerCapture(event.pointerId);
});

astOutput.addEventListener("pointermove", (event) => {
  if (!astPanState || astPanState.pointerId !== event.pointerId) {
    return;
  }

  event.preventDefault();
  astOutput.scrollLeft = astPanState.scrollLeft - (event.clientX - astPanState.startX);
  astOutput.scrollTop = astPanState.scrollTop - (event.clientY - astPanState.startY);
});

function stopAstPanning(event) {
  if (!astPanState || astPanState.pointerId !== event.pointerId) {
    return;
  }

  astOutput.classList.remove("is-panning");
  astOutput.releasePointerCapture(event.pointerId);
  astPanState = null;
}

astOutput.addEventListener("pointerup", stopAstPanning);
astOutput.addEventListener("pointercancel", stopAstPanning);

astOutput.addEventListener(
  "wheel",
  (event) => {
    if (!astOutput.querySelector(".ast-diagram")) {
      return;
    }

    if (event.ctrlKey || event.metaKey) {
      event.preventDefault();
      const factor = event.deltaY < 0 ? 1.1 : 0.9;
      setAstZoom(astZoom * factor, event.clientX, event.clientY);
    }
  },
  { passive: false },
);

function setEditorPaneWidthFromClientX(clientX) {
  const workspace = document.querySelector(".workspace");
  if (!workspace) {
    return;
  }

  const bounds = workspace.getBoundingClientRect();
  const relativeX = clientX - bounds.left;
  const minWidth = 280;
  const maxWidth = bounds.width - 420 - 24;
  const clamped = Math.max(minWidth, Math.min(maxWidth, relativeX));
  const percent = (clamped / bounds.width) * 100;
  document.documentElement.style.setProperty("--editor-pane-width", `${percent}%`);
}

if (paneDivider) {
  paneDivider.addEventListener("pointerdown", (event) => {
    if (window.innerWidth <= 980) {
      return;
    }

    paneDivider.classList.add("is-dragging");
    paneDivider.setPointerCapture(event.pointerId);
    setEditorPaneWidthFromClientX(event.clientX);
  });

  paneDivider.addEventListener("pointermove", (event) => {
    if (!paneDivider.classList.contains("is-dragging")) {
      return;
    }
    setEditorPaneWidthFromClientX(event.clientX);
  });

  const stopDragging = (event) => {
    if (!paneDivider.classList.contains("is-dragging")) {
      return;
    }
    paneDivider.classList.remove("is-dragging");
    if (event.pointerId !== undefined) {
      paneDivider.releasePointerCapture(event.pointerId);
    }
  };

  paneDivider.addEventListener("pointerup", stopDragging);
  paneDivider.addEventListener("pointercancel", stopDragging);
}

try {
  await init();
  compilerReady = true;
  compilerStatus.textContent = "Compiler loaded.";
} catch (error) {
  compilerStatus.textContent = "Compiler failed to load.";
  runtimeOutput.textContent = error instanceof Error ? error.message : String(error);
}
