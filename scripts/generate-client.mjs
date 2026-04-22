import { createFromRoot } from 'codama';
import { rootNodeFromAnchor } from '@codama/nodes-from-anchor';
import { renderVisitor } from '@codama/renderers-js';
import { readFileSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));

const idlPath = join(__dirname, '..', 'target', 'idl', 'flashloan.json');
const idl = JSON.parse(readFileSync(idlPath, 'utf-8'));

const codama = createFromRoot(rootNodeFromAnchor(idl));

const outputDir = join(__dirname, '..', 'app', 'src', 'generated');

codama.accept(renderVisitor(outputDir));

console.log(`Client generated at app/src/generated`);
